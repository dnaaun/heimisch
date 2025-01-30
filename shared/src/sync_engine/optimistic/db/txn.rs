use std::{cell::RefCell, sync::Arc};

use typesafe_idb::{ReadOnly, ReadWrite, Store, StoreMarker, Txn, TxnBuilder, TxnMode};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::{
    object_store::ObjectStoreWithOptimisticChanges,
    reactivity_trackers::{CommitListener, ReactivityTrackers},
};

#[derive(derive_more::From)]
struct TxnWithOptimisticChangesInner<C, Mode> {
    idb_txn: Txn<C, Mode>,
    commit_listener: Option<CommitListener>,
}

pub struct TxnWithOptimisticChanges<C, Mode> {
    optimistic_updates: Arc<OptimisticChanges>,
    /// RTI: Will be None if and only if the transaction is committed or aborted.
    /// RTI upkeep: `.commit()` and `.abort()` take a `self`.
    inner: Option<TxnWithOptimisticChangesInner<C, Mode>>,

    /// Could probably pass out &mut references istead of RefCell, but let's go for easy mode Rust.
    reactivity_trackers: RefCell<ReactivityTrackers>,
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
            self.inner.as_ref().expect("").idb_txn.object_store::<S>()?,
            &self.reactivity_trackers,
        ))
    }

    pub fn commit(mut self) -> Result<ReactivityTrackers, idb::Error> {
        self.commit_impl()
    }

    fn commit_impl(&mut self) -> Result<ReactivityTrackers, idb::Error> {
        // Note how we `.take()` this? That's how we make sure that, in the Drop impl, we don't
        //   (1) commit the inner transaction, and
        //   (2) invoke the commit listener twice.
        let TxnWithOptimisticChangesInner {
            idb_txn,
            commit_listener,
        } = self.inner.take().expect("");
        idb_txn.commit()?;
        if let Some(listener) = commit_listener {
            // tracing::trace!("Invoking listener for txn commit");
            listener(&self.reactivity_trackers.borrow());
        } else {
            // tracing::trace!("No listener to invoke for txn commit.");
        };
        Ok(self.reactivity_trackers.take())
    }

    pub fn reactivity_trackers(&self) -> ReactivityTrackers {
        self.reactivity_trackers.borrow().clone()
    }

    pub fn abort(mut self) -> Result<(), idb::Error> {
        self.inner.take().expect("").idb_txn.abort()
    }
}

impl<Markers, Mode> Drop for TxnWithOptimisticChanges<Markers, Mode> {
    fn drop(&mut self) {
        let _ = self
            .commit_impl()
            .expect("Couldn't commit a transaction in the Drop impl.");
    }
}

#[derive(derive_more::Constructor)]
pub struct TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, Mode> {
    inner: TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode>,
    optimistic_updates: Arc<OptimisticChanges>,
    commit_listener: Option<CommitListener>,
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
            commit_listener: self.commit_listener,
        }
    }

    pub fn read_write(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, ReadWrite> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_write(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    pub fn read_only(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, ReadOnly> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_only(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    pub fn with_no_commit_listener(self) -> Self {
        Self {
            inner: self.inner,
            optimistic_updates: self.optimistic_updates,
            commit_listener: None,
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
            inner: Some((self.inner.build(), self.commit_listener).into()),
            reactivity_trackers: Default::default(),
        }
    }
}
