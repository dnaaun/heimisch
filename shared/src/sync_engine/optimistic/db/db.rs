use std::sync::Arc;

use typesafe_idb::{ReadOnly, ReadWrite, Store, StoreMarker, Txn, TypesafeDb};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::{reactivity_trackers::CommitListener, TxnBuilderWithOptimisticChanges};

pub struct DbWithOptimisticChanges<StoreMarkers> {
    inner: TypesafeDb<StoreMarkers>,
    optimistic_updates: Arc<OptimisticChanges>,
    pub(crate) listener: CommitListener,
}

impl<StoreMarkers> DbWithOptimisticChanges<StoreMarkers> {
    pub fn new(inner: TypesafeDb<StoreMarkers>, listener: CommitListener) -> Self {
        Self {
            inner,
            optimistic_updates: Arc::new(Default::default()),
            listener,
        }
    }
}

impl<DbStoreMarkers> DbWithOptimisticChanges<DbStoreMarkers> {
    pub fn txn(&self) -> TxnBuilderWithOptimisticChanges<'_, DbStoreMarkers, (), ReadOnly> {
        TxnBuilderWithOptimisticChanges::new(
            Txn::builder(&self.inner),
            self.optimistic_updates.clone(),
            Some(self.listener.clone()),
        )
    }

    /// Shortcut
    pub fn object_store<S: Store>(
        &self,
    ) -> Result<super::ObjectStoreWithOptimisticChanges<S, ReadOnly>, typesafe_idb::Error>
    where
        DbStoreMarkers: StoreMarker<S>,
    {
        self.txn().with_store::<S>().build().object_store::<S>()
    }

    /// Shortcut
    pub fn object_store_rw<S: Store>(
        &self,
    ) -> Result<super::ObjectStoreWithOptimisticChanges<S, ReadWrite>, typesafe_idb::Error>
    where
        DbStoreMarkers: StoreMarker<S>,
    {
        self.txn()
            .with_store::<S>()
            .read_write()
            .build()
            .object_store::<S>()
    }
}
