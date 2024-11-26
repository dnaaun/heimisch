use std::{future::Future, rc::Rc};

use shared::sync_engine::{DbStoreMarkers, SyncEngine};
use typesafe_idb::{Txn, TypesafeDb};

use crate::idb_signal::IdbSignal;

pub trait IdbSignalFromSyncEngine<DbStoreMarkers> {
    /// Will create a signal that will be recomputed every time an indexeddb change is committed by
    /// the sync engine.
    /// Note that the "reactivity tracking" depends on detecting which stores, and which ids, are
    /// accessed when the function passed here is called the first time. (pretty much in the style
    /// of signals from leptos/solid/whatever).
    fn idb_signal<TxnStoreMarkers, Mode, Fut, T>(
        &self,
        make_txn: impl Fn(&TypesafeDb<DbStoreMarkers>) -> Txn<TxnStoreMarkers, Mode> + 'static,
        compute_val: impl Fn(Rc<Txn<TxnStoreMarkers, Mode>>) -> Fut + 'static,
    ) -> IdbSignal<T>
    where
        TxnStoreMarkers: 'static,
        Mode: 'static,
        Fut: Future<Output = T>,
        T: 'static;
}

impl IdbSignalFromSyncEngine<DbStoreMarkers> for SyncEngine {
    /// TODO: This doesn't react to dependencies changing in `compute_val()`
    fn idb_signal<TxnStoreMarkers, Mode, Fut, T>(
        &self,
        make_txn: impl Fn(&TypesafeDb<DbStoreMarkers>) -> Txn<TxnStoreMarkers, Mode> + 'static,
        compute_val: impl Fn(Rc<Txn<TxnStoreMarkers, Mode>>) -> Fut + 'static,
    ) -> IdbSignal<T>
    where
        TxnStoreMarkers: 'static,
        Fut: Future<Output = T>,
        Mode: 'static,
        T: 'static,
    {
        let db = self.db.clone();
        let make_txn = move || make_txn(db.as_ref());
        let idb_notifiers = self.idb_notifiers.clone();
        let register_notifier = move |thingy| {
            let mut idb_notifiers = idb_notifiers.lock();
            Box::new(idb_notifiers.add(thingy))
        };
        IdbSignal::new(make_txn, compute_val, register_notifier)
    }
}
