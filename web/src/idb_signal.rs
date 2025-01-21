use std::{
    future::{Future, IntoFuture},
    ops::Deref,
    panic::Location,
    sync::Arc,
};

use leptos::prelude::*;
use parking_lot::Mutex;
use shared::sync_engine::DbSubscription;
use typesafe_idb::Txn;

type DontKNowWhatToNameYou<S> = AsyncDerived<S, LocalStorage>;

type DeregisterNotifierFunc = Arc<Mutex<Option<Arc<dyn Fn() + Sync + Send>>>>;

pub struct IdbSignalInner<S> {
    /// Is an Option because idb is async, and so on initial render, this will be None
    local_resource: DontKNowWhatToNameYou<S>,
    /// Is an Option because idb is async, and so on initial render, this will be None
    deregister_notifier: DeregisterNotifierFunc,

    #[allow(dead_code)]
    defined_at: &'static Location<'static>,
}

/// NOTE: I'm actually not sure if this is ever called due to not understanding totally the life
/// cycle of a signal, as well as the behavior of the arena allocation stuff.
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

#[allow(unused)]
impl<S: 'static + Clone> IdbSignal<S> {
    pub fn get(&self) -> Option<S> {
        self.inner.try_get_value().unwrap().local_resource.get()
    }

    pub fn get_untracked(&self) -> Option<S> {
        self.inner
            .try_get_value()
            .unwrap()
            .local_resource
            .read_untracked()
            .clone()
    }
}

impl<T> IntoFuture for IdbSignal<T>
where
    AsyncDerived<T, LocalStorage>: std::future::IntoFuture,
    T: 'static,
{
    type Output = <AsyncDerived<T, LocalStorage> as IntoFuture>::Output;
    type IntoFuture = <AsyncDerived<T, LocalStorage> as IntoFuture>::IntoFuture;

    fn into_future(self) -> Self::IntoFuture {
        self.inner
            .try_get_value()
            .unwrap()
            .local_resource
            .into_future()
    }
}

impl<T> IdbSignal<T>
where
    T: std::fmt::Debug + 'static,
{
    #[track_caller]
    pub fn new<Markers, Mode, Fut, Deregister>(
        make_txn: impl Fn() -> Txn<Markers, Mode> + 'static,
        compute_val: impl Fn(Arc<Txn<Markers, Mode>>) -> Fut + 'static,
        register_notifier: impl Fn(DbSubscription) -> Deregister + 'static,
    ) -> Self
    where
        Fut: Future<Output = T>,
        Markers: 'static,
        Mode: 'static,
        Deregister: Fn() + Send + Sync + 'static,
    {
        let register_notifier = Arc::new(register_notifier);

        let compute_val = Arc::new(compute_val);

        let make_txn = Arc::new(move || Arc::new(make_txn()));

        let deregister_notifier: DeregisterNotifierFunc = Arc::new(Mutex::new(None));
        let deregister_notifier_copy = deregister_notifier.clone();

        let trigger = Trigger::new();

        let defined_at = Location::caller();

        let async_derived = AsyncDerived::new_unsync(move || {
            let make_txn = make_txn.clone();
            let compute_val = compute_val.clone();
            let deregister_notifier = deregister_notifier.clone();
            let register_notifier = register_notifier.clone();
            async move {
                // tracing::trace!("In async block in async derived.");
                trigger.track();
                let txn = make_txn();
                let val = compute_val(txn.clone()).await;

                let db_subscription = DbSubscription {
                    original_reactivity_trackers: txn.reactivity_trackers(),
                    func: Arc::new(move || {
                        // tracing::trace!("IndexedDB notification change received for idb_signal defined at: {defined_at}");
                        trigger.notify();
                    }),
                };

                *deregister_notifier.lock() = Some(Arc::new(register_notifier(db_subscription)));

                val
            }
        });

        Self {
            inner: ArenaItem::new_with_storage(Arc::new(IdbSignalInner {
                local_resource: async_derived,
                deregister_notifier: deregister_notifier_copy,
                defined_at,
            })),
        }
    }
}
