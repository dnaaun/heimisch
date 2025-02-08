use std::{cell::RefCell, future::Future, panic::Location, rc::Rc};

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

impl<S, Mode> ObjectStoreWithOptimisticChanges<S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, id: &S::Id) -> Result<Option<S>, Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_read(S::NAME, SerializedId::new_from_id::<S>(id));

        if self.optimistic_changes.deletes.latest::<S>(id).is_some() {
            return Ok(None);
        }
        let optimistically_updated = self.optimistic_changes.updates.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_updated {
            return Ok(Some(o));
        }
        let optimistically_created = self.optimistic_changes.creations.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_created {
            return Ok(Some(o));
        };

        self.inner
            .get(id)
            .await
            .map_err(|e| Error::new(e, self.location))
    }

    pub(crate) async fn no_optimism_get(&self, id: &S::Id) -> Result<Option<S>, Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_read(S::NAME, SerializedId::new_from_id::<S>(id));

        self.inner
            .get(id)
            .await
            .map_err(|e| super::Error::new(e, self.location))
    }

    pub async fn get_all(&self) -> Result<Vec<S>, super::Error> {
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
                    .unwrap_or(r)
            });
        let mut all = Vec::from_iter(from_db_filtered);
        all.extend(
            self.optimistic_changes
                .creations
                .all_the_latest_downcasted(),
        );

        Ok(all)
    }

    #[allow(dead_code)]
    pub(crate) async fn no_optimism_get_all(&self) -> Result<Vec<S>, typesafe_idb::Error> {
        self.reactivity_trackers.borrow_mut().add_bulk_read(S::NAME);

        self.inner.get_all().await
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
    pub async fn no_optimism_delete(&self, id: &S::Id) -> Result<(), Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_modification(S::NAME, SerializedId::new_from_id::<S>(id));

        self.inner
            .delete(id)
            .await
            .map_err(|e| Error::new(e, self.location))?;
        self.optimistic_changes.remove_obsoletes_for_id::<S>(id);
        Ok(())
    }

    pub async fn no_optimism_put(&self, item: &S) -> Result<(), Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_modification(S::NAME, SerializedId::new_from_row(item));

        self.inner
            .put(item)
            .await
            .map_err(|e| Error::new(e, self.location))?;
        self.optimistic_changes
            .remove_obsoletes_for_id::<S>(item.id());
        Ok(())
    }
    pub fn update(&self, row: S, update_fut: impl Future<Output = Result<(), ()>> + 'static) {
        self.reactivity_trackers
            .borrow_mut()
            .add_modification(S::NAME, SerializedId::new_from_row(&row));
        self.optimistic_changes.update(row, update_fut);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            commit_listener(&self.reactivity_trackers.borrow());
        }
    }

    pub fn create(&self, row: S, create_fut: impl Future<Output = Result<S::Id, ()>> + 'static) {
        self.reactivity_trackers
            .borrow_mut()
            .add_modification(S::NAME, SerializedId::new_from_row(&row));
        self.optimistic_changes.create(row, create_fut);

        if let Some(commit_listener) = self.commit_listener.as_ref() {
            commit_listener(&self.reactivity_trackers.borrow());
        }
    }
}

