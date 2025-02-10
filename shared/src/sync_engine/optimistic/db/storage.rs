use std::{panic::Location, rc::Rc};

use typesafe_idb::{ReadOnly, ReadWrite, Store, StoreMarker, Txn, TypesafeDb, TypesafeDbBuilder};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::{reactivity_trackers::CommitListener, Error, TxnBuilderWithOptimisticChanges};

pub struct DbWithOptimisticChanges<StoreMarkers> {
    inner: TypesafeDb<StoreMarkers>,
    optimistic_updates: Rc<OptimisticChanges>,
    pub(crate) listener: CommitListener,
}

impl<StoreMarkers> DbWithOptimisticChanges<StoreMarkers> {
    #[track_caller]
    pub async fn new(
        inner: TypesafeDbBuilder<StoreMarkers>,
        listener: CommitListener,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: inner
                .build()
                .await
                .map_err(|e| Error::new(e, Location::caller()))?,
            optimistic_updates: Rc::new(Default::default()),
            listener,
        })
    }
}

impl<DbStoreMarkers> DbWithOptimisticChanges<DbStoreMarkers> {
    #[track_caller]
    pub fn txn(&self) -> TxnBuilderWithOptimisticChanges<'_, DbStoreMarkers, (), ReadOnly> {
        TxnBuilderWithOptimisticChanges::new(
            Txn::builder(&self.inner),
            self.optimistic_updates.clone(),
            Some(self.listener.clone()),
            Location::caller(),
        )
    }

    /// Shortcut
    #[track_caller]
    pub fn object_store<S: Store>(
        &self,
    ) -> Result<super::ObjectStoreWithOptimisticChanges<S, ReadOnly>, Error>
    where
        DbStoreMarkers: StoreMarker<S>,
    {
        self.txn().with_store::<S>().build().object_store::<S>()
    }

    /// Shortcut
    #[track_caller]
    pub fn object_store_rw<S: Store>(
        &self,
    ) -> Result<super::ObjectStoreWithOptimisticChanges<S, ReadWrite>, Error>
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
