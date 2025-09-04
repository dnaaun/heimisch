use std::sync::Arc;

use typed_db::{Db, DbBuilder, RawDbTrait, ReadOnly, ReadWrite, Table, TableMarker};

use crate::sync_engine::optimistic::{db::MaybeOptimistic, optimistic_changes::OptimisticChanges};

use super::{reactivity_trackers::CommitListener, TxnBuilderWithOptimisticChanges};

pub struct DbWithOptimisticChanges<RawDb: RawDbTrait, StoreMarkers> {
    inner: Db<RawDb, StoreMarkers>,
    optimistic_updates: Arc<OptimisticChanges>,
    pub(crate) listener: CommitListener,
}

impl<RawDb: RawDbTrait, StoreMarkers> DbWithOptimisticChanges<RawDb, StoreMarkers> {
    #[track_caller]
    pub async fn new(
        inner: DbBuilder<RawDb, StoreMarkers>,
        listener: CommitListener,
    ) -> Result<Self, RawDb::Error> {
        Ok(Self {
            inner: inner.build().await?,
            optimistic_updates: Default::default(),
            listener,
        })
    }
}

impl<RawDb: RawDbTrait, DbTableMarkers> DbWithOptimisticChanges<RawDb, DbTableMarkers> {
    #[track_caller]
    pub fn txn(&self) -> TxnBuilderWithOptimisticChanges<'_, RawDb, DbTableMarkers, (), ReadOnly> {
        TxnBuilderWithOptimisticChanges::new(
            self.inner.txn(),
            self.optimistic_updates.clone(),
            Some(self.listener.clone()),
        )
    }

    /// Shortcut
    pub async fn get<T>(
        &self,

        // The type here is the same thing as `Id` itself. But this helps the type system
        // understand that fact.
        id: &T::Id,
    ) -> Result<Option<T>, RawDb::Error>
    where
        T: Table,
        DbTableMarkers: TableMarker<T>,
    {
        self.table::<T>().await.get(id).await
    }

    // Shortcut
    pub async fn get_all_optimistically<T>(&self) -> Result<Vec<MaybeOptimistic<T>>, RawDb::Error>
    where
        DbTableMarkers: TableMarker<T>,
        T: Table,
    {
        self.table::<T>().await.get_all_optimistically().await
    }

    // Shortcut
    pub async fn get_optimistically<T>(
        &self,
        id: &T::Id,
    ) -> Result<Option<MaybeOptimistic<T>>, RawDb::Error>
    where
        T: Table,
        DbTableMarkers: TableMarker<T>,
    {
        self.table::<T>().await.get_optimistically(id).await
    }

    // Shortcut
    pub async fn put<T: Table>(&self, item: &T) -> Result<(), RawDb::Error>
    where
        DbTableMarkers: TableMarker<T>,
    {
        self.table_rw::<T>().await.put(item).await
    }

    // Shortcut
    pub async fn delete<T: Table>(&self, id: &T::Id) -> Result<(), RawDb::Error>
    where
        DbTableMarkers: TableMarker<T>,
    {
        self.table_rw::<T>().await.delete(id).await
    }

    /// Shortcut
    #[track_caller]
    pub async fn table<S: Table + 'static>(
        &self,
    ) -> super::TableWithOptimisticChanges<RawDb, S, ReadOnly>
    where
        DbTableMarkers: TableMarker<S>,
    {
        self.txn().with_table::<S>().build().await.table::<S>()
    }

    /// Shortcut
    #[track_caller]
    pub async fn table_rw<S: Table + 'static>(
        &self,
    ) -> super::TableWithOptimisticChanges<RawDb, S, ReadWrite>
    where
        DbTableMarkers: TableMarker<S>,
    {
        self.txn()
            .with_table::<S>()
            .read_write()
            .build()
            .await
            .table::<S>()
    }

    pub fn get_optimistic_to_realistic_for_creations<S: Table>(
        &self,
        optimistic_id: &S::Id,
    ) -> Option<S::Id> {
        self.optimistic_updates
            .get_optimistic_to_realistic_for_creations::<S>(optimistic_id)
    }
}
