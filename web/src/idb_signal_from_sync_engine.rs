use leptos::prelude::*;
use std::{future::Future, sync::Arc};

use send_wrapper::SendWrapper;
use shared::sync_engine::{DbStoreMarkers, SyncEngine};
use typesafe_idb::{Chain, ReadOnly, Txn, TxnBuilder, TxnMode};

use crate::{frontend_error::FrontendError, idb_signal::IdbSignal};

pub trait IdbSignalFromSyncEngine<DbStoreMarkers, TxnStoreMarkers, Mode, Fut, T>
where
    T: 'static,
{
    /// Will create a signal that will be recomputed every time an indexeddb change is committed by
    /// the sync engine.
    /// Note that the "reactivity tracking" depends on detecting which stores, and which ids, are
    /// accessed when the function passed here is called the first time. (pretty much in the style
    /// of signals from leptos/solid/whatever).
    fn idb_signal(
        &self,
        make_txn: impl for<'a> Fn(
                TxnBuilder<'a, DbStoreMarkers, Chain<(), ()>, ReadOnly>,
            ) -> Txn<TxnStoreMarkers, Mode>
            + 'static,
        compute_val: impl Fn(Arc<Txn<TxnStoreMarkers, Mode>>) -> Fut + 'static,
    ) -> IdbSignal<Result<T, FrontendError>>;
}

impl<TT, TxnStoreMarkers, Mode, Fut, T>
    IdbSignalFromSyncEngine<DbStoreMarkers, TxnStoreMarkers, Mode, Fut, T> for SyncEngine<TT>
where
    TxnStoreMarkers: 'static,
    Fut: Future<Output = Result<T, FrontendError>>,
    Mode: TxnMode + 'static,
    T: 'static + std::fmt::Debug,
{
    #[track_caller]
    fn idb_signal(
        &self,
        make_txn: impl for<'a> Fn(
                TxnBuilder<'a, DbStoreMarkers, Chain<(), ()>, ReadOnly>,
            ) -> Txn<TxnStoreMarkers, Mode>
            + 'static,
        compute_val: impl Fn(Arc<Txn<TxnStoreMarkers, Mode>>) -> Fut + 'static,
    ) -> IdbSignal<Result<T, FrontendError>> {
        let db = SendWrapper::new(self.db.clone());
        let make_txn = move || make_txn(db.txn().with_no_commit_listener());
        let db_subscriptions = self.db_subscriptions.clone();

        IdbSignal::new(make_txn, compute_val, move |db_subscription| {
            db_subscriptions.add(db_subscription)
        })
    }
}
