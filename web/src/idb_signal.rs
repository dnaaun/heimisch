use std::{future::Future, ops::Deref, rc::Rc, sync::Arc};

use leptos::{prelude::*, task::spawn_local};
use parking_lot::Mutex;
use send_wrapper::SendWrapper;
use shared::sync_engine::{IdbNotification, IdbNotifier};
use typesafe_idb::Txn;

use crate::utils::rc;

type Inner<S> = RwSignal<SendWrapper<Option<S>>>;

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

impl<S: 'static> IdbSignal<S> {
    pub fn read(&self) -> impl Deref<Target = SendWrapper<std::option::Option<S>>> {
        self.inner.try_get_value().unwrap().inner.read()
    }
}

impl<T> IdbSignal<T>
where
    // for RwSignal
    T: 'static,
{
    pub fn new<Markers, Mode, Fut, Deregister>(
        make_txn: impl Fn() -> Txn<Markers, Mode> + 'static,
        compute_val: impl Fn(Rc<Txn<Markers, Mode>>) -> Fut + 'static,
        register_notifier: impl FnOnce(IdbNotifier) -> Deregister + 'static,
    ) -> Self
    where
        Fut: Future<Output = T>,
        Markers: 'static,
        Mode: 'static,
        Deregister: Fn() + Send + Sync + 'static,
    {
        let rw_signal = RwSignal::new(SendWrapper::new(Option::<T>::None));
        let deregister_notifier: DeregisterNotifierFunc = Arc::new(Mutex::new(None));
        let deregister_notifier2 = deregister_notifier.clone();

        let make_txn = Rc::new(make_txn);
        let compute_val = Rc::new(compute_val);
        spawn_local(async move {
            let txn = Rc::new(make_txn());
            let value = compute_val(txn.clone()).await;
            let reactivity_trackers = txn.reactivity_trackers();
            *rw_signal.write() = SendWrapper::new(Some(value));

            let notifier = Box::new(move |notification: IdbNotification| {
                let reactivity_trackers2 = reactivity_trackers.clone();
                let compute_val2 = compute_val.clone();
                let make_txn2 = make_txn.clone();
                if notification.matches_triggered_trackers(&reactivity_trackers2) {
                    spawn_local(async move {
                        rw_signal.set(SendWrapper::new(Some(compute_val2(rc(make_txn2())).await)));
                    });
                };
            });

            *deregister_notifier2.lock() = Some(Box::new(register_notifier(notifier)));
        });

        Self {
            inner: ArenaItem::new_with_storage(Arc::new(IdbSignalInner {
                inner: rw_signal,
                deregister_notifier,
            })),
        }
    }
}
