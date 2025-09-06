use leptos::prelude::*;
use std::{future::Future, sync::Arc};

use shared::{
    backend_api_trait::BackendApiTrait,
    sync_engine::{
        optimistic::db::{TxnBuilderWithOptimisticChanges, TxnWithOptimisticChanges},
        SyncEngine, TransportTrait,
    },
};
use typed_db::ReadOnly;
use utils::JustSend;

use crate::{frontend_error::FrontendError, idb_signal::IdbSignal};

pub trait IdbSignalFromSyncEngine<Fut, T>
where
    T: 'static + std::fmt::Debug,
{
    /// Will create a signal that will be recomputed every time an indexeddb change is committed by
    /// the sync engine.
    /// Note that the "reactivity tracking" depends on detecting which stores, and which ids, are
    /// accessed when the function passed here is called the first time. (pretty much in the style
    /// of signals from leptos/solid/whatever).
    fn idb_signal(
        &self,
        make_txn: impl for<'a> AsyncFn(
                TxnBuilderWithOptimisticChanges<'a, JustSend<idb::Database>, ReadOnly>,
            )
                -> TxnWithOptimisticChanges<JustSend<idb::Database>, ReadOnly>
            + 'static,
        compute_val: impl Fn(Arc<TxnWithOptimisticChanges<JustSend<idb::Database>, ReadOnly>>) -> Fut
            + 'static,
    ) -> IdbSignal<Result<T, FrontendError>>;
}

impl<BA, TT, GH, Fut, T> IdbSignalFromSyncEngine<Fut, T>
    for SyncEngine<JustSend<idb::Database>, BA, TT, GH>
where
    Fut: Future<Output = Result<T, FrontendError>>,
    T: 'static + std::fmt::Debug,
    TT: TransportTrait,
    BA: BackendApiTrait,
{
    #[track_caller]
    fn idb_signal(
        &self,
        make_txn: impl for<'a> AsyncFn(
                TxnBuilderWithOptimisticChanges<'a, JustSend<idb::Database>, ReadOnly>,
            )
                -> TxnWithOptimisticChanges<JustSend<idb::Database>, ReadOnly>
            + 'static,
        compute_val: impl Fn(Arc<TxnWithOptimisticChanges<JustSend<idb::Database>, ReadOnly>>) -> Fut
            + 'static,
    ) -> IdbSignal<Result<T, FrontendError>> {
        let db = self.db.clone();
        let make_txn = async move || make_txn(db.txn().with_no_commit_listener()).await;
        let db_subscriptions = self.db_subscriptions.clone();

        IdbSignal::new(make_txn, compute_val, move |db_subscription| {
            db_subscriptions.add(db_subscription)
        })
    }
}
