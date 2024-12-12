use std::{future::Future, ops::Deref, sync::Arc};

use leptos::{prelude::*, task::spawn_local};
use parking_lot::Mutex;
use send_wrapper::SendWrapper;
use shared::sync_engine::{IdbNotification, IdbNotifier};
use typesafe_idb::Txn;

type Inner<S> = AsyncDerived<S>;

type DeregisterNotifierFunc = Arc<Mutex<Option<Box<dyn Fn() + Sync + Send>>>>;

pub struct IdbSignalInner<S> {
    /// Is an Option because idb is async, and so on initial render, this will be None
    inner: Inner<S>,

    /// Is an Option because idb is async, and so on initial render, this will be None
    deregister_notifier: DeregisterNotifierFunc,
}

/// NOTE: I'm actually not sure if this is ever called due to not understanding totally the life
/// cycle of a signal, as well as the behavior of teh arena allocation stuff.
impl<S> Drop for IdbSignalInner<S> {
    fn drop(&mut self) {
        if let Some(deregister_notifier) = self.deregister_notifier.lock().deref() {
            deregister_notifier()
        }
    }
}

pub struct IdbSignal<S> {
    inner: ArenaItem<Arc<IdbSignalInner<S>>>,
}

impl<S> Copy for IdbSignal<S> {}

impl<S> Clone for IdbSignal<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: 'static + Send + Sync> IdbSignal<S> {
    pub fn read(&self) -> <Inner<S> as Read>::Value {
        self.inner.try_get_value().unwrap().inner.read()
    }
}

impl<T> IdbSignal<T>
where
    // for RwSignal
    T: 'static + Sync + Send,
{
    pub fn new<Markers, Mode, Fut, Deregister>(
        make_txn: impl Fn() -> Txn<Markers, Mode> + 'static,
        compute_val: impl Fn(Arc<Txn<Markers, Mode>>) -> Fut + 'static,
        register_notifier: impl FnOnce(IdbNotifier) -> Deregister + 'static,
    ) -> Self
    where
        Fut: Future<Output = T>,
        Markers: 'static,
        Mode: 'static,
        Deregister: Fn() + Send + Sync + 'static,
    {
        let compute_val = SendWrapper::new(Arc::new(compute_val));
        let make_txn = SendWrapper::new(Arc::new(move || Arc::new(make_txn())));

        // Make sure to update the value when the dependencies of `compute_val` change.
        let compute_val2 = compute_val.clone();
        let make_txn2 = make_txn.clone();
        let async_derived = AsyncDerived::new(move || {
            let make_txn = make_txn2.clone();
            let compute_val = compute_val2.clone();
            async move { SendWrapper::new(compute_val(make_txn())).await }
        });

        let deregister_notifier: DeregisterNotifierFunc = Arc::new(Mutex::new(None));
        let deregister_notifier2 = deregister_notifier.clone();

        spawn_local(async move {
            let txn = (make_txn)();
            let reactivity_trackers = txn.reactivity_trackers();

            let notifier = Box::new(move |notification: IdbNotification| {
                let reactivity_trackers2 = reactivity_trackers.clone();
                let compute_val2 = compute_val.clone();
                let make_txn2 = make_txn.clone();
                if notification.matches_triggered_trackers(&reactivity_trackers2) {
                    spawn_local(async move {
                        async_derived.set(Some(compute_val2(make_txn2()).await));
                    });
                };
            });

            *deregister_notifier2.lock() = Some(Box::new(register_notifier(notifier)));
        });

        Self {
            inner: ArenaItem::new_with_storage(Arc::new(IdbSignalInner {
                inner: async_derived,
                deregister_notifier,
            })),
        }
    }
}
