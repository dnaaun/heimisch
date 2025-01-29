use std::collections::{BTreeMap, HashMap};

use parking_lot::RwLock;
pub use status::Status;
use typesafe_idb::{SerializedId, Store, StoreName};

use super::monotonic_time::OptimisticTime;

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

pub struct OptimisticChangeMap<T, SuccessMarker = ()> {
    inner: RwLock<
        HashMap<
            StoreName,
            HashMap<SerializedId, BTreeMap<OptimisticTime, Status<T, SuccessMarker>>>,
        >,
    >,
}

impl<V: Store> Default for OptimisticChangeMap<V, ()> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T, SuccessMarker: Eq> OptimisticChangeMap<T, SuccessMarker> {
    pub fn insert<S: Store>(&self, id: &S::Id, v: T) -> OptimisticTime {
        let id = SerializedId::new_from_id::<S>(id);
        let time = OptimisticTime::new();
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
    pub fn remove_pending<S: Store>(&self, id: &S::Id, time: &OptimisticTime) {
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
        time: &OptimisticTime,
        marker: SuccessMarker,
    ) {
        let id = SerializedId::new_from_id::<S>(id);
        self.inner
            .write()
            .get_mut(&S::NAME)
            .unwrap()
            .get_mut(&id)
            .unwrap()
            .get_mut(time)
            .unwrap()
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
