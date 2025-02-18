use std::{cell::RefCell, future::Future, panic::Location, rc::Rc};

use maplit::{hashmap, hashset};
use typesafe_idb::{IndexSpec, ObjectStore, Present, Store, TxnMode};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::{
    index::IndexWithOptimisticChanges, reactivity_trackers::ReactivityTrackers, CommitListener,
    Error, SerializedId,
};

#[derive(Clone, derive_more::Constructor)]
pub struct ObjectStoreWithOptimisticChanges<S, Mode> {
    optimistic_changes: Rc<OptimisticChanges>,
    inner: ObjectStore<S, Mode>,
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

impl<S, Mode> ObjectStoreWithOptimisticChanges<S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, id: &S::Id) -> Result<Option<MaybeOptimistic<S>>, Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_read(S::NAME, SerializedId::new_from_id::<S>(id));

        if self.optimistic_changes.deletes.latest::<S>(id).is_some() {
            return Ok(None);
        }
        let optimistically_updated = self.optimistic_changes.updates.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_updated {
            return Ok(Some(MaybeOptimistic::new(o, true)));
        }
        let optimistically_created = self.optimistic_changes.creations.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_created {
            return Ok(Some(MaybeOptimistic::new(o, true)));
        };

        self.inner
            .get(id)
            .await
            .map_err(|e| Error::new(e, self.location))
            .map(|o| o.map(|o| MaybeOptimistic::new(o, false)))
    }

    pub async fn no_optimism_get(&self, id: &S::Id) -> Result<Option<S>, Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_read(S::NAME, SerializedId::new_from_id::<S>(id));

        self.inner
            .get(id)
            .await
            .map_err(|e| super::Error::new(e, self.location))
    }

    pub async fn get_all(&self) -> Result<Vec<MaybeOptimistic<S>>, super::Error> {
        self.reactivity_trackers.borrow_mut().add_bulk_read(S::NAME);

        let from_db_filtered = self
            .inner
            .get_all()
            .await
            .map_err(|e| super::Error::new(e, self.location))?
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
    pub(crate) async fn no_optimism_get_all(&self) -> Result<Vec<S>, Error> {
        self.reactivity_trackers.borrow_mut().add_bulk_read(S::NAME);

        self.inner
            .get_all()
            .await
            .map_err(|e| Error::new(e, self.location))
    }

    pub fn index<IS: IndexSpec<Store = S>>(
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

impl<S, Mode> ObjectStoreWithOptimisticChanges<S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    pub async fn delete(&self, id: &S::Id) -> Result<(), Error> {
        self.inner
            .delete(id)
            .await
            .map_err(|e| Error::new(e, self.location))?;
        self.optimistic_changes.remove_obsoletes_for_id::<S>(id);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            let reactivity_trackers = ReactivityTrackers {
                stores_modified: hashmap![S::NAME => hashset![SerializedId::new_from_id::<S>(id)]],
                ..Default::default()
            };
            commit_listener(&reactivity_trackers);
        }

        Ok(())
    }

    pub async fn put(&self, item: &S) -> Result<(), Error> {
        self.inner
            .put(item)
            .await
            .map_err(|e| Error::new(e, self.location))?;
        self.optimistic_changes
            .remove_obsoletes_for_id::<S>(item.id());

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            let reactivity_trackers = ReactivityTrackers {
                stores_modified: hashmap![S::NAME => hashset![SerializedId::new_from_row(item)]],
                ..Default::default()
            };
            commit_listener(&reactivity_trackers);
        }

        Ok(())
    }
    pub fn update(&self, row: S, update_fut: impl Future<Output = Result<(), ()>> + 'static) {
        let reactivity_trackers = ReactivityTrackers {
            stores_modified: hashmap![S::NAME => hashset![SerializedId::new_from_row(&row)]],
            ..Default::default()
        };
        self.optimistic_changes.update(row, update_fut);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            commit_listener(&reactivity_trackers);
        }
    }

    pub fn create(&self, row: S, create_fut: impl Future<Output = Result<S::Id, ()>> + 'static) {
        let reactivity_trackers = ReactivityTrackers {
            stores_modified: hashmap![S::NAME => hashset![SerializedId::new_from_row(&row)]],
            ..Default::default()
        };
        self.optimistic_changes.create(row, create_fut);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            commit_listener(&reactivity_trackers);
        }
    }
}
