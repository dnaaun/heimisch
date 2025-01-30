use std::sync::Arc;

use typesafe_idb::{ReadOnly, ReadWrite, Store, StoreMarker, Txn, TxnBuilder, TxnMode};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::object_store::ObjectStoreWithOptimisticChanges;

pub struct TxnWithOptimisticChanges<C, Mode> {
    optimistic_updates: Arc<OptimisticChanges>,
    inner: Txn<C, Mode>,
}

impl<Markers, Mode> TxnWithOptimisticChanges<Markers, Mode> {
    pub fn object_store<S>(
        &self,
    ) -> Result<ObjectStoreWithOptimisticChanges<'_, S, Mode>, typesafe_idb::Error>
    where
        S: Store,
        Markers: StoreMarker<S>,
    {
        Ok(ObjectStoreWithOptimisticChanges::new(
            self.optimistic_updates.clone(),
            self.inner.object_store::<S>()?,
        ))
    }

    pub async fn commit(self) -> Result<typesafe_idb::ReactivityTrackers, idb::Error> {
        self.inner.commit().await
    }

    pub fn reactivity_trackers(&self) -> typesafe_idb::ReactivityTrackers {
        self.inner.reactivity_trackers()
    }

    pub async fn abort(self) -> Result<(), idb::Error> {
        self.inner.abort().await
    }
}

#[derive(derive_more::Constructor)]
pub struct TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, Mode> {
    inner: TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode>,
    optimistic_updates: Arc<OptimisticChanges>,
}

impl<'db, DbTableMarkers, TxnTableMarkers, Mode>
    TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, Mode>
where
    TxnTableMarkers: Default,
{
    pub fn with_store<H2>(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, (H2::Marker, TxnTableMarkers), Mode>
    where
        H2: Store,
        DbTableMarkers: StoreMarker<H2>,
    {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.with_store::<H2>(),
            optimistic_updates: self.optimistic_updates,
        }
    }

    pub fn read_write(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, ReadWrite> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_write(),
            optimistic_updates: self.optimistic_updates,
        }
    }

    pub fn read_only(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, ReadOnly> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_only(),
            optimistic_updates: self.optimistic_updates,
        }
    }

    pub fn with_no_commit_listener(self) -> Self {
        Self {
            inner: self.inner.with_no_commit_listener(),
            optimistic_updates: self.optimistic_updates,
        }
    }
}

impl<TxnTableMarkers, DbTableMarkers, Mode>
    TxnBuilderWithOptimisticChanges<'_, DbTableMarkers, TxnTableMarkers, Mode>
where
    Mode: TxnMode,
{
    pub fn build(self) -> TxnWithOptimisticChanges<TxnTableMarkers, Mode> {
        TxnWithOptimisticChanges {
            optimistic_updates: self.optimistic_updates.clone(),
            inner: self.inner.build(),
        }
    }
}
