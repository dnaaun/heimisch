use std::sync::Arc;

use typed_db::{RawDbTrait, ReadOnly, ReadWrite, Table, Txn, TxnBuilder, TxnMode};

use crate::sync_engine::optimistic::{db::MaybeOptimistic, optimistic_changes::OptimisticChanges};

use super::{
    reactivity_trackers::{CommitListener, ReactivityTrackers},
    table::TableWithOptimisticChanges,
};

#[derive(derive_more::From)]
struct TxnWithOptimisticChangesInner<RawDb: RawDbTrait, Mode> {
    inner_txn: Txn<RawDb, Mode>,
    commit_listener: Option<CommitListener>,
}

pub struct TxnWithOptimisticChanges<RawDb: RawDbTrait, Mode> {
    optimistic_updates: Arc<OptimisticChanges>,
    /// RTI: Will be None if and only if the transaction is committed or aborted.
    /// RTI upkeep: `.commit()` and `.abort()` take a `self`.
    inner: Option<TxnWithOptimisticChangesInner<RawDb, Mode>>,

    /// Could probably pass out &mut references istead of RefCell, but let's go for easy mode Rust.
    pub reactivity_trackers: ReactivityTrackers,
}

impl<RawDb: RawDbTrait, Mode> TxnWithOptimisticChanges<RawDb, Mode> {
    pub async fn get<S: Table>(&self, id: &S::Id) -> Result<Option<S>, RawDb::Error>
    where
        Mode: TxnMode,
    {
        self.table::<S>().get(id).await
    }

    pub async fn get_optimistically<S: Table>(
        &self,
        id: &S::Id,
    ) -> Result<Option<MaybeOptimistic<S>>, RawDb::Error>
    where
        Mode: TxnMode,
    {
        self.table::<S>().get_optimistically(id).await
    }

    pub fn table<S>(&self) -> TableWithOptimisticChanges<RawDb, S, Mode>
    where
        S: Table,
        Mode: TxnMode,
    {
        let inner = self.inner.as_ref().expect("");
        TableWithOptimisticChanges::new(
            self.optimistic_updates.clone(),
            Arc::new(inner.inner_txn.table::<S>()),
            self.reactivity_trackers.clone(),
            inner.commit_listener.clone(),
        )
    }

    pub async fn commit(mut self) -> Result<ReactivityTrackers, RawDb::Error> {
        self.commit_impl().await?;
        Ok(self.reactivity_trackers.clone())
    }

    async fn commit_impl(&mut self) -> Result<(), RawDb::Error> {
        // Note how we `.take()` this? That's how we make sure that, in the Drop impl, we don't
        //   (1) commit the inner transaction, and
        //   (2) invoke the commit listener twice.
        if let Some(TxnWithOptimisticChangesInner {
            inner_txn,
            commit_listener,
        }) = self.inner.take()
        {
            inner_txn.commit().await?;
            if let Some(listener) = commit_listener {
                listener(&self.reactivity_trackers);
            };
        };

        Ok(())
    }

    pub fn reactivity_trackers(&self) -> ReactivityTrackers {
        self.reactivity_trackers.clone()
    }

    pub async fn abort(mut self) -> Result<(), RawDb::Error> {
        self.inner.take().expect("").inner_txn.abort().await?;
        Ok(())
    }
}

impl<RawDb: RawDbTrait, Mode> Drop for TxnWithOptimisticChanges<RawDb, Mode> {
    fn drop(&mut self) {
        let _ = self.commit_impl();
    }
}

#[derive(derive_more::Constructor)]
pub struct TxnBuilderWithOptimisticChanges<'db, RawDb: RawDbTrait, Mode> {
    inner: TxnBuilder<'db, RawDb, Mode>,
    optimistic_updates: Arc<OptimisticChanges>,
    commit_listener: Option<CommitListener>,
}

impl<'db, RawDb: RawDbTrait, Mode> TxnBuilderWithOptimisticChanges<'db, RawDb, Mode>
where
    Mode: TxnMode,
{
    #[track_caller]
    pub fn with_table<H2>(self) -> TxnBuilderWithOptimisticChanges<'db, RawDb, Mode>
    where
        H2: Table + 'static,
    {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.with_table::<H2>(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    #[track_caller]
    pub fn read_write(self) -> TxnBuilderWithOptimisticChanges<'db, RawDb, ReadWrite> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_write(),
            optimistic_updates: self.optimistic_updates,
            commit_listener: self.commit_listener,
        }
    }

    #[track_caller]
    pub fn read_only(self) -> TxnBuilderWithOptimisticChanges<'db, RawDb, ReadOnly> {
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

impl<RawDb: RawDbTrait, Mode> TxnBuilderWithOptimisticChanges<'_, RawDb, Mode>
where
    Mode: TxnMode,
{
    pub async fn build(self) -> TxnWithOptimisticChanges<RawDb, Mode> {
        TxnWithOptimisticChanges {
            optimistic_updates: self.optimistic_updates.clone(),
            inner: Some((self.inner.build().await, self.commit_listener).into()),
            reactivity_trackers: Default::default(),
        }
    }
}
