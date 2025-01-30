use std::{cell::RefCell, sync::Arc};

use typesafe_idb::{IndexSpec, ObjectStore, Present, Store, TxnMode};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::{index::IndexWithOptimisticChanges, reactivity_trackers::ReactivityTrackers, SerializedId};

#[derive(Clone, derive_more::Constructor)]
pub struct ObjectStoreWithOptimisticChanges<'txn, S, Mode> {
    optimistic_changes: Arc<OptimisticChanges>,
    inner: ObjectStore<S, Mode>,
    pub(crate) reactivity_trackers: &'txn RefCell<ReactivityTrackers>,
}

impl<S, Mode> ObjectStoreWithOptimisticChanges<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, id: &S::Id) -> Result<Option<S>, typesafe_idb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_access(S::NAME, SerializedId::new_from_id::<S>(id));

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

        self.inner.get(id).await
    }

    pub(crate) async fn no_optimism_get(
        &self,
        id: &S::Id,
    ) -> Result<Option<S>, typesafe_idb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_access(S::NAME, SerializedId::new_from_id::<S>(id));

        self.inner.get(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<S>, typesafe_idb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_access(S::NAME);

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

    pub(crate) async fn no_optimism_get_all(&self) -> Result<Vec<S>, typesafe_idb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_access(S::NAME);

        self.inner.get_all().await
    }

    pub fn index<IS: IndexSpec<Store = S>>(
        &self,
    ) -> Result<IndexWithOptimisticChanges<'_, IS>, typesafe_idb::Error> {
        Ok(IndexWithOptimisticChanges::new(
            self.optimistic_changes.clone(),
            self.inner.index::<IS>()?,
            &self.reactivity_trackers,
        ))
    }
}

impl<S, Mode> ObjectStoreWithOptimisticChanges<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    pub async fn no_optimism_delete(&self, id: &S::Id) -> Result<(), typesafe_idb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_modification(S::NAME, SerializedId::new_from_id::<S>(id));

        self.inner.delete(id).await?;
        self.optimistic_changes.remove_obsoletes_for_id::<S>(id);
        Ok(())
    }

    pub async fn no_optimism_put(&self, item: &S) -> Result<(), typesafe_idb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_modification(S::NAME, SerializedId::new_from_row(item));

        self.inner.put(item).await?;
        self.optimistic_changes
            .remove_obsoletes_for_id::<S>(item.id());
        Ok(())
    }
}
