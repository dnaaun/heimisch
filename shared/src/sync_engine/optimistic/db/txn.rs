use std::{cell::RefCell, rc::Rc};

use typed_db::{RawDbTrait, ReadOnly, ReadWrite, Table, TableMarker, Txn, TxnBuilder, TxnMode};

use crate::sync_engine::optimistic::{db::MaybeOptimistic, optimistic_changes::OptimisticChanges};

use super::{
    object_store::TableWithOptimisticChanges,
    reactivity_trackers::{CommitListener, ReactivityTrackers},
};

#[derive(derive_more::From)]
struct TxnWithOptimisticChangesInner<RawDb: RawDbTrait, C, Mode> {
    idb_txn: Txn<RawDb, C, Mode>,
    commit_listener: Option<CommitListener>,
}

pub struct TxnWithOptimisticChanges<RawDb: RawDbTrait, C, Mode> {
    optimistic_updates: Rc<OptimisticChanges>,
    /// RTI: Will be None if and only if the transaction is committed or aborted.
    /// RTI upkeep: `.commit()` and `.abort()` take a `self`.
    inner: Option<TxnWithOptimisticChangesInner<RawDb, C, Mode>>,

    /// Could probably pass out &mut references istead of RefCell, but let's go for easy mode Rust.
    reactivity_trackers: Rc<RefCell<ReactivityTrackers>>,
}

impl<RawDb: RawDbTrait, Markers, Mode> TxnWithOptimisticChanges<RawDb, Markers, Mode> {
    pub async fn get<S: Table>(&self, id: &S::Id) -> Result<Option<S>, RawDb::Error>
    where
        Markers: TableMarker<S>,
        Mode: TxnMode,
    {
        self.table::<S>().get(id).await
    }

    pub async fn get_optimistically<S: Table>(
        &self,
        id: &S::Id,
    ) -> Result<Option<MaybeOptimistic<S>>, RawDb::Error>
    where
        Markers: TableMarker<S>,
        Mode: TxnMode,
    {
        self.table::<S>().get_optimistically(id).await
    }

    pub fn table<S>(&self) -> TableWithOptimisticChanges<RawDb, S, Mode>
    where
        S: Table,
        Markers: TableMarker<S>,
    {
        let inner = self.inner.as_ref().expect("");
        TableWithOptimisticChanges::new(
            self.optimistic_updates.clone(),
            Rc::new(inner.idb_txn.table::<S>()),
            self.reactivity_trackers.clone(),
            inner.commit_listener.clone(),
        )
    }

    pub fn commit(mut self) -> Result<ReactivityTrackers, RawDb::Error> {
        self.commit_impl()?;
        Ok(RefCell::clone(&self.reactivity_trackers).into_inner())
    }

    fn commit_impl(&mut self) -> Result<(), RawDb::Error> {
        // Note how we `.take()` this? That's how we make sure that, in the Drop impl, we don't
        //   (1) commit the inner transaction, and
        //   (2) invoke the commit listener twice.
        if let Some(TxnWithOptimisticChangesInner {
            idb_txn,
            commit_listener,
        }) = self.inner.take()
        {
            idb_txn.commit()?;
            if let Some(listener) = commit_listener {
                tracing::trace!(
                    "Invoking listener for txn commit with reactivity trackers: {:?}",
                    self.reactivity_trackers.borrow()
                );
                listener(&self.reactivity_trackers.borrow());
            } else {
                // tracing::trace!("No listener to invoke for txn commit.");
            };
        };

        Ok(())
    }

    pub fn reactivity_trackers(&self) -> ReactivityTrackers {
        self.reactivity_trackers.borrow().clone()
    }

    pub fn abort(mut self) -> Result<(), RawDb::Error> {
        self.inner.take().expect("").idb_txn.abort()?;
        Ok(())
    }
}

impl<RawDb: RawDbTrait, Markers, Mode> Drop for TxnWithOptimisticChanges<RawDb, Markers, Mode> {
    fn drop(&mut self) {
        let _ = self.commit_impl();
    }
}

#[derive(derive_more::Constructor)]
pub struct TxnBuilderWithOptimisticChanges<
    'db,
    RawDb: RawDbTrait,
    DbTableMarkers,
    TxnTableMarkers,
    Mode,
> {
    inner: TxnBuilder<'db, RawDb, DbTableMarkers, TxnTableMarkers, Mode>,
    optimistic_updates: Rc<OptimisticChanges>,
    commit_listener: Option<CommitListener>,
}

impl<'db, RawDb: RawDbTrait, DbTableMarkers, TxnTableMarkers, Mode>
    TxnBuilderWithOptimisticChanges<'db, RawDb, DbTableMarkers, TxnTableMarkers, Mode>
where
    TxnTableMarkers: Default,
    Mode: TxnMode,
{
    #[track_caller]
    pub fn with_table<H2>(
        self,
    ) -> TxnBuilderWithOptimisticChanges<
        'db,
        RawDb,
        DbTableMarkers,
        (H2::Marker, TxnTableMarkers),
        Mode,
    >
    where
        H2: Table + 'static,
        DbTableMarkers: TableMarker<H2>,
    {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.with_table::<H2>(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    #[track_caller]
    pub fn read_write(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, RawDb, DbTableMarkers, TxnTableMarkers, ReadWrite>
    {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_write(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    #[track_caller]
    pub fn read_only(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, RawDb, DbTableMarkers, TxnTableMarkers, ReadOnly>
    {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_only(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    #[track_caller]
    pub fn with_no_commit_listener(self) -> Self {
        Self {
            inner: self.inner,
            optimistic_updates: self.optimistic_updates,
            commit_listener: None,
        }
    }
}

impl<RawDb: RawDbTrait, TxnTableMarkers, DbTableMarkers, Mode>
    TxnBuilderWithOptimisticChanges<'_, RawDb, DbTableMarkers, TxnTableMarkers, Mode>
where
    Mode: TxnMode,
    TxnTableMarkers: Default,
{
    pub fn build(self) -> TxnWithOptimisticChanges<RawDb, TxnTableMarkers, Mode> {
        TxnWithOptimisticChanges {
            optimistic_updates: self.optimistic_updates.clone(),
            inner: Some((self.inner.build(), self.commit_listener).into()),
            reactivity_trackers: Default::default(),
        }
    }
}
