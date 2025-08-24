use std::{cell::RefCell, future::Future, panic::Location, rc::Rc};

use maplit::{hashmap, hashset};

use crate::{
    typed_db::{IndexSpec, Present, RawDbTrait, Table, TableAccess, TxnMode},
    sync_engine::optimistic::optimistic_changes::OptimisticChanges,
};

use super::{
    index::IndexWithOptimisticChanges, reactivity_trackers::ReactivityTrackers, CommitListener,
    Error, SerializedId,
};

#[derive(Clone, derive_more::Constructor)]
pub struct TableWithOptimisticChanges<RawDb: RawDbTrait, S: Table, Mode> {
    optimistic_changes: Rc<OptimisticChanges>,
    inner: TableAccess<RawDb, S, Mode>,
    pub reactivity_trackers: Rc<RefCell<ReactivityTrackers>>,
    pub commit_listener: Option<CommitListener>,
    location: &'static Location<'static>,
}

#[derive(Clone, derive_more::Constructor, derive_more::Deref, Debug, PartialEq, Eq, Hash)]
pub struct MaybeOptimistic<S> {
    #[deref]
    inner: S,
    pub is_optimistic: bool,
}

impl<S> MaybeOptimistic<S> {
    pub fn into_inner(self) -> S {
        self.inner
    }

    pub fn map<T>(self, f: impl FnOnce(S) -> T) -> MaybeOptimistic<T> {
        MaybeOptimistic {
            inner: f(self.inner),
            is_optimistic: self.is_optimistic,
        }
    }

    pub fn map_ref<T>(&self, f: impl FnOnce(&S) -> T) -> MaybeOptimistic<T> {
        MaybeOptimistic {
            inner: f(&self.inner),
            is_optimistic: self.is_optimistic,
        }
    }
}

impl<RawDb, S, Mode> TableWithOptimisticChanges<RawDb, S, Mode>
where
    Mode: TxnMode,
    RawDb: RawDbTrait,
    S: Table + 'static,
{
    pub async fn get_optimistically(
        &self,
        id: &S::Id,
    ) -> Result<Option<MaybeOptimistic<S>>, RawDb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_read(S::NAME, SerializedId::new_from_id::<S>(id));

        if self.optimistic_changes.deletes.latest::<S>(id).is_some() {
            return Ok(None);
        }
        let optimistically_updated = self.optimistic_changes.updates.latest_downcasted::<S>(&id);
        if let Some(o) = optimistically_updated {
            return Ok(Some(MaybeOptimistic::new(o, true)));
        }
        let optimistically_created = self.optimistic_changes.creations.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_created {
            return Ok(Some(MaybeOptimistic::new(o, true)));
        };

        self.inner
            .get(&id)
            .await
            .map(|o| o.map(|o| MaybeOptimistic::new(o, false)))
    }

    pub async fn get(&self, id: &S::Id) -> Result<Option<S>, RawDb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_read(S::NAME, SerializedId::new_from_id::<S>(id));

        self.inner.get(id).await
    }

    pub async fn get_all_optimistically(&self) -> Result<Vec<MaybeOptimistic<S>>, RawDb::Error> {
        self.reactivity_trackers.borrow_mut().add_bulk_read(S::NAME);

        let from_db_filtered = self
            .inner
            .get_all()
            .await?
            .into_iter()
            .filter(|r| {
                self.optimistic_changes
                    .deletes
                    .latest::<S>(r.id())
                    .is_none()
            })
            .map(|r| {
                self.optimistic_changes
                    .updates
                    .latest_downcasted(r.id())
                    .map(|o| MaybeOptimistic::new(o, true))
                    .unwrap_or(MaybeOptimistic::new(r, false))
            });
        let mut all = Vec::from_iter(from_db_filtered);
        all.extend(
            self.optimistic_changes
                .creations
                .all_the_latest_downcasted()
                .into_iter()
                .map(|o| MaybeOptimistic::new(o, true)),
        );

        Ok(all)
    }

    #[allow(dead_code)]
    pub(crate) async fn get_all(&self) -> Result<Vec<S>, RawDb::Error> {
        self.reactivity_trackers.borrow_mut().add_bulk_read(S::NAME);

        self.inner.get_all().await
    }

    pub fn index<IS: IndexSpec<Table = S>>(
        &self,
    ) -> Result<IndexWithOptimisticChanges<'_, IS>, Error> {
        Ok(IndexWithOptimisticChanges::new(
            self.optimistic_changes.clone(),
            self.inner
                .index::<IS>()
                .map_err(|e| Error::new(e, self.location))?,
            &self.reactivity_trackers,
            self.location,
        ))
    }
}

impl<RawDb: RawDbTrait, S: Table, Mode> TableWithOptimisticChanges<RawDb, S, Mode>
where
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    /// Abstract away writing both the equivalent optimistic id (if such
    /// an id exists in optimistic creations) and the actual id
    /// to the reactivity trackers.
    pub fn add_to_reactivity_during_write(&self, id: &S::Id) {
        let serialized_id = SerializedId::new_from_id::<S>(id);
        let optimistic_id = self
            .optimistic_changes
            .get_realistic_to_optimistic_for_creations::<S>(id)
            .map(|i| SerializedId::new_from_id::<S>(&i));

        let mut reactivity_trackers = self.reactivity_trackers.borrow_mut();
        reactivity_trackers.add_modification(S::NAME, serialized_id);
        if let Some(optimistic_id) = optimistic_id {
            reactivity_trackers.add_modification(S::NAME, optimistic_id);
        }
    }

    pub async fn delete(&self, id: &S::Id) -> Result<(), RawDb::Error> {
        self.inner.delete(id).await?;
        self.optimistic_changes.remove_successful_for_id::<S>(id);

        self.add_to_reactivity_during_write(id);

        Ok(())
    }

    pub async fn put(&self, item: &S) -> Result<(), RawDb::Error> {
        self.inner.put(item).await?;
        self.optimistic_changes
            .remove_successful_for_id::<S>(item.id());

        self.add_to_reactivity_during_write(item.id());

        Ok(())
    }

    pub fn update_optimistically(
        &self,
        row: S,
        update_fut: impl Future<Output = Result<(), ()>> + 'static,
    ) {
        let reactivity_trackers = ReactivityTrackers {
            stores_modified: hashmap![S::NAME => hashset![SerializedId::new_from_row(&row)]],
            ..Default::default()
        };
        self.optimistic_changes.update(row, update_fut);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            tracing::trace!(
                "In ObjectStoreWithOptimisticChanges::update calling commit listener with {:?}",
                reactivity_trackers
            );
            commit_listener(&reactivity_trackers);
        }
    }

    pub fn create_optimistically(
        &self,
        row: S,
        create_fut: impl Future<Output = Result<S::Id, ()>> + 'static,
    ) {
        let reactivity_trackers = ReactivityTrackers {
            stores_modified: hashmap![S::NAME => hashset![SerializedId::new_from_row(&row)]],
            ..Default::default()
        };

        self.optimistic_changes.create(row, create_fut);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            tracing::trace!(
                "In ObjectStoreWithOptimisticChanges::create calling commit listener with {:?}",
                reactivity_trackers
            );
            commit_listener(&reactivity_trackers);
        }
    }
}
