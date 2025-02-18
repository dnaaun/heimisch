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
    /// the `mark_realistic()` function.
    #[derive(Clone)]
    pub enum Status<T, RealisticId> {
        Optimistic(Option<T>),
        Realistic { t: T, realistic_id: RealisticId },
    }

    impl<T, RealisticId> Status<T, RealisticId> {
        pub fn new(t: T) -> Status<T, RealisticId> {
            Self::Optimistic(Some(t))
        }
        pub fn mark_realistic(&mut self, marker: RealisticId) {
            if let Status::Optimistic(t) = self {
                let t = std::mem::take(t).unwrap();
                *self = Status::Realistic {
                    realistic_id: marker,
                    t,
                }
            } else {
                panic!("Tried to mark_realistic something that was already marked as such!")
            }
        }

        pub fn as_realistic(&self) -> Option<&T> {
            match self {
                Status::Realistic { t, .. } => Some(t),
                _ => None,
            }
        }

        pub fn read(&self) -> &T {
            match self {
                Status::Optimistic(p) => p.as_ref().unwrap(),
                Status::Realistic { t, .. } => t,
            }
        }

        pub fn get(self) -> T {
            match self {
                Status::Optimistic(p) => p.unwrap(),
                Status::Realistic { t, .. } => t,
            }
        }
    }

    impl<T> From<T> for Status<T, ()> {
        fn from(value: T) -> Self {
            Self::Optimistic(Some(value))
        }
    }
}

struct OptimisticChangeMapInner<T, RealisticId> {
    changes:
        HashMap<StoreName, HashMap<SerializedId, BTreeMap<MonotonicTime, Status<T, RealisticId>>>>,
    realistic_to_optimistic: HashMap<RealisticId, SerializedId>,
}

impl<T, RealisticId> Default for OptimisticChangeMapInner<T, RealisticId> {
    fn default() -> Self {
        Self {
            changes: Default::default(),
            realistic_to_optimistic: Default::default(),
        }
    }
}

pub struct OptimisticChangeMap<T, RealisticId = ()> {
    inner: Arc<RwLock<OptimisticChangeMapInner<T, RealisticId>>>,
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

impl<T, RealisticId: Eq + std::fmt::Debug> OptimisticChangeMap<T, RealisticId> {
    pub fn insert<S: Store>(&self, optimistic_id: &S::Id, v: T) -> MonotonicTime {
        let optimistic_id = SerializedId::new_from_id::<S>(optimistic_id);
        let time = MonotonicTime::new();
        self.inner
            .write()
            .changes
            .entry(S::NAME)
            .or_default()
            .entry(optimistic_id)
            .or_default()
            .insert(time, Status::new(v));
        time
    }

    /// Will panic if the thing is not pending, or if it doesn't exist.
    pub fn remove_pending<S: Store>(&self, optimistic_id: &S::Id, time: &MonotonicTime) {
        let optimistic_id = SerializedId::new_from_id::<S>(optimistic_id);
        let mut by_id_len = None;
        let mut inner = self.inner.write();
        if let Some(by_id) = inner.changes.get_mut(&S::NAME) {
            let mut by_time_len = None;
            if let Some(by_time) = by_id.get_mut(&optimistic_id) {
                match by_time.entry(*time) {
                    std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                        if let Status::Optimistic(_) = occupied_entry.get() {
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
                by_id.remove(&optimistic_id);
            }
            by_id_len = Some(by_id.len());
        }
        if by_id_len == Some(0) {
            inner.changes.remove(&S::NAME);
        }
    }

    pub fn remove_all_realistic<S: Store>(
        &self,
        optimistic_id: &S::Id,
        realistic_id: &RealisticId,
    ) {
        let optimistic_id = SerializedId::new_from_id::<S>(optimistic_id);
        let mut by_id_len = None;
        let mut inner = self.inner.write();
        if let Some(by_id) = inner.changes.get_mut(&S::NAME) {
            let mut by_time_len = None;
            if let Some(by_time) = by_id.get_mut(&optimistic_id) {
                let to_remove_keys = by_time
                    .iter()
                    .filter_map(|(time, status)| match status {
                        Status::Realistic {
                            realistic_id: realistic_id_candidate,
                            ..
                        } if realistic_id_candidate == realistic_id => Some(time),
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
                by_id.remove(&optimistic_id);
            }
            by_id_len = Some(by_id.len());
        }
        if by_id_len == Some(0) {
            inner.changes.remove(&S::NAME);
        }
    }
}

impl<T, RealisticId: Clone + Eq + std::hash::Hash> OptimisticChangeMap<T, RealisticId> {
    pub fn mark_realistic<S: Store>(
        &self,
        optimistic_id: &S::Id,
        time: &MonotonicTime,
        realistic_id: RealisticId,
    ) {
        let optimistic_id = SerializedId::new_from_id::<S>(optimistic_id);
        let mut inner = self.inner.write();
        inner
            .changes
            .get_mut(&S::NAME)
            .unwrap()
            .get_mut(&optimistic_id)
            .expect("id to mark realistic not found")
            .get_mut(time)
            .expect("could not find the monotonic time to mark realistic")
            .mark_realistic(realistic_id.clone());

        inner
            .realistic_to_optimistic
            .insert(realistic_id, optimistic_id);
    }

    pub fn get_realistic_to_optimistic(&self) -> HashMap<RealisticId, SerializedId> {
        self.inner.read().realistic_to_optimistic.clone()
    }
}

impl<T: Clone, RealisticId: Clone> OptimisticChangeMap<T, RealisticId> {
    pub fn latest<S: Store>(&self, optimistic_id: &S::Id) -> Option<Status<T, RealisticId>> {
        let optimistic_id = SerializedId::new_from_id::<S>(optimistic_id);
        Some(
            self.inner
                .read()
                .changes
                .get(&S::NAME)?
                .get(&optimistic_id)?
                .last_key_value()?
                .1
                .clone(),
        )
    }
}

impl<RealisticId> OptimisticChangeMap<Rc<dyn Any>, RealisticId> {
    pub fn latest_downcasted<S: Store + 'static>(&self, optimistic_id: &S::Id) -> Option<S> {
        let optimistic_id = SerializedId::new_from_id::<S>(optimistic_id);
        Some(
            self.inner
                .read()
                .changes
                .get(&S::NAME)?
                .get(&optimistic_id)?
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
            .changes
            .get(&S::NAME)
            .map(|s| s.values())
            .into_iter()
            .flatten()
            .filter_map(|v| v.last_key_value().map(|(_time, thing)| thing))
            .map(|thing| thing.read().downcast_ref::<S>().expect("").clone())
            .collect()
    }
}
