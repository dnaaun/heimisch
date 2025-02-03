use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
    rc::Rc,
    sync::Arc,
};

use parking_lot::RwLock;
pub use status::Status;
use typesafe_idb::{Store, StoreName};

use super::{db::SerializedId, monotonic_time::MonotonicTime};

mod status {
    /// The value inside is never `None`. But having an option is the only way I know of enabling
    /// the `mark_successful()` function.
    #[derive(Clone)]
    pub enum Status<T, SuccessMarker> {
        Pending(Option<T>),
        Successful { t: T, marker: SuccessMarker },
    }

    impl<T, SuccessMarker> Status<T, SuccessMarker> {
        pub fn new(t: T) -> Status<T, SuccessMarker> {
            Self::Pending(Some(t))
        }
        pub fn mark_successful(&mut self, marker: SuccessMarker) {
            if let Status::Pending(t) = self {
                let t = std::mem::take(t).unwrap();
                *self = Status::Successful { marker, t }
            } else {
                panic!("Tried to mark_successful something that was already marked as such!")
            }
        }

        pub fn as_successful(&self) -> Option<&T> {
            match self {
                Status::Successful { t, .. } => Some(t),
                _ => None,
            }
        }

        pub fn read(&self) -> &T {
            match self {
                Status::Pending(p) => p.as_ref().unwrap(),
                Status::Successful { t, .. } => t,
            }
        }

        pub fn get(self) -> T {
            match self {
                Status::Pending(p) => p.unwrap(),
                Status::Successful { t, .. } => t,
            }
        }
    }

    impl<T> From<T> for Status<T, ()> {
        fn from(value: T) -> Self {
            Self::Pending(Some(value))
        }
    }
}

type OptimisticChangeMapInner<T, SuccessMarker> =
    HashMap<StoreName, HashMap<SerializedId, BTreeMap<MonotonicTime, Status<T, SuccessMarker>>>>;

pub struct OptimisticChangeMap<T, SuccessMarker = ()> {
    inner: Arc<RwLock<OptimisticChangeMapInner<T, SuccessMarker>>>,
}

impl<T, S> Clone for OptimisticChangeMap<T, S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T, S> Default for OptimisticChangeMap<T, S> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T, SuccessMarker: Eq + std::fmt::Debug> OptimisticChangeMap<T, SuccessMarker> {
    pub fn insert<S: Store>(&self, id: &S::Id, v: T) -> MonotonicTime {
        let id = SerializedId::new_from_id::<S>(id);
        let time = MonotonicTime::new();
        self.inner
            .write()
            .entry(S::NAME)
            .or_default()
            .entry(id)
            .or_default()
            .insert(time, Status::new(v));
        time
    }

    /// Will panic if the thing is not pending, or if it doesn't exist.
    pub fn remove_pending<S: Store>(&self, id: &S::Id, time: &MonotonicTime) {
        let id = SerializedId::new_from_id::<S>(id);
        let mut by_id_len = None;
        let mut inner = self.inner.write();
        if let Some(by_id) = inner.get_mut(&S::NAME) {
            let mut by_time_len = None;
            if let Some(by_time) = by_id.get_mut(&id) {
                match by_time.entry(*time) {
                    std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                        if let Status::Pending(_) = occupied_entry.get() {
                            occupied_entry.remove();
                        } else {
                            panic!("Is not pending.");
                        }
                    }
                    _ => panic!("Doesn't exist."),
                }
                by_time_len = Some(by_time.len());
            }
            if by_time_len == Some(0) {
                by_id.remove(&id);
            }
            by_id_len = Some(by_id.len());
        }
        if by_id_len == Some(0) {
            inner.remove(&S::NAME);
        }
    }

    pub fn remove_all_successful<S: Store>(&self, id: &S::Id, success_marker: &SuccessMarker) {
        let id = SerializedId::new_from_id::<S>(id);
        let mut by_id_len = None;
        let mut inner = self.inner.write();
        if let Some(by_id) = inner.get_mut(&S::NAME) {
            let mut by_time_len = None;
            if let Some(by_time) = by_id.get_mut(&id) {
                let to_remove_keys = by_time
                    .iter()
                    .filter_map(|(time, status)| match status {
                        Status::Successful { marker, .. } if marker == success_marker => Some(time),
                        _ => None,
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                for key in to_remove_keys {
                    by_time.remove(&key);
                }
                by_time_len = Some(by_time.len());
            }
            if by_time_len == Some(0) {
                by_id.remove(&id);
            }
            by_id_len = Some(by_id.len());
        }
        if by_id_len == Some(0) {
            inner.remove(&S::NAME);
        }
    }

    pub fn mark_successful<S: Store>(
        &self,
        id: &S::Id,
        time: &MonotonicTime,
        marker: SuccessMarker,
    ) {
        tracing::info!("Marking succesful id={id:?}, and marker={marker:?}");
        let id = SerializedId::new_from_id::<S>(id);
        self.inner
            .write()
            .get_mut(&S::NAME)
            .unwrap()
            .get_mut(&id)
            .expect("id to mark successful not found")
            .get_mut(time)
            .expect("could not find the monotonic time to mark succesful")
            .mark_successful(marker);
    }
}

impl<T: Clone, SuccessMarker: Clone> OptimisticChangeMap<T, SuccessMarker> {
    pub fn latest<S: Store>(&self, id: &S::Id) -> Option<Status<T, SuccessMarker>> {
        let id = SerializedId::new_from_id::<S>(id);
        Some(
            self.inner
                .read()
                .get(&S::NAME)?
                .get(&id)?
                .last_key_value()?
                .1
                .clone(),
        )
    }
}

impl<SuccessMarker> OptimisticChangeMap<Rc<dyn Any>, SuccessMarker> {
    pub fn latest_downcasted<S: Store + 'static>(&self, id: &S::Id) -> Option<S> {
        let id = SerializedId::new_from_id::<S>(id);
        Some(
            self.inner
                .read()
                .get(&S::NAME)?
                .get(&id)?
                .last_key_value()?
                .1
                .read()
                .downcast_ref::<S>()
                .expect("")
                .clone(),
        )
    }

    pub fn all_the_latest_downcasted<S: Store + 'static>(&self) -> Vec<S> {
        self.inner
            .read()
            .get(&S::NAME)
            .map(|s| s.values())
            .into_iter()
            .flatten()
            .filter_map(|v| v.last_key_value().map(|(_time, thing)| thing))
            .map(|thing| thing.read().downcast_ref::<S>().expect("").clone())
            .collect()
    }
}
