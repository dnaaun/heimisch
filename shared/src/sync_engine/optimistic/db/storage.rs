use std::{panic::Location, rc::Rc};

use typed_db::{Db, DbBuilder, RawDbTrait, ReadOnly, ReadWrite, Table, TableMarker};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::{reactivity_trackers::CommitListener, TxnBuilderWithOptimisticChanges};

pub struct DbWithOptimisticChanges<RawDb: RawDbTrait, StoreMarkers> {
    inner: Db<RawDb, StoreMarkers>,
    optimistic_updates: Rc<OptimisticChanges>,
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
            optimistic_updates: Rc::new(Default::default()),
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
            Location::caller(),
        )
    }

    /// Shortcut
    #[track_caller]
    pub fn table<S: Table + 'static>(
        &self,
    ) -> Result<super::TableWithOptimisticChanges<RawDb, S, ReadOnly>, RawDb::Error>
    where
        DbTableMarkers: TableMarker<S>,
    {
        Ok(self.txn().with_table::<S>().build()?.table::<S>()?)
    }

    /// Shortcut
    #[track_caller]
    pub fn table_rw<S: Table + 'static>(
        &self,
    ) -> Result<super::TableWithOptimisticChanges<RawDb, S, ReadWrite>, RawDb::Error>
    where
        DbTableMarkers: TableMarker<S>,
    {
        Ok(self
            .txn()
            .with_table::<S>()
            .read_write()
            .build()?
            .table::<S>()?)
    }

    pub fn get_optimistic_to_realistic_for_creations<S: Table>(
        &self,
        optimistic_id: &S::Id,
    ) -> Option<S::Id> {
        self.optimistic_updates
            .get_optimistic_to_realistic_for_creations::<S>(optimistic_id)
    }
}
