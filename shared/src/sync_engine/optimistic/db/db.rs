use std::sync::Arc;

use typesafe_idb::{ReadOnly, Txn, TypesafeDb};

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
            Some(self.listener.clone())
        )
    }
}
