use std::collections::{BTreeMap, HashMap};

use parking_lot::RwLock;
use typesafe_idb::{SerializedId, Store, StoreName};

use super::monotonic_time::OptimisticTime;

/// The value inside is never `None`. But having an option is the only way I know of enabling
/// the `mark_successful()` function.
#[derive(Clone)]
pub enum Status<T> {
    Pending(Option<T>),
    Successful(Option<T>),
}

impl<T> Status<T> {
    pub fn mark_successful(&mut self) {
        match self {
            Status::Pending(t) => {
                let t = std::mem::take(t);
                *self = Status::Successful(t)
            }
            _ => (),
        }
    }

    pub fn read(&self) -> &T {
        match self {
            Status::Pending(p) => p,
            Status::Successful(s) => s,
        }
        .as_ref()
        .unwrap()
    }

    pub fn get(self) -> T {
        match self {
            Status::Pending(p) => p,
            Status::Successful(s) => s,
        }
        .unwrap()
    }
}

impl<T> From<T> for Status<T> {
    fn from(value: T) -> Self {
        Self::Pending(Some(value))
    }
}

pub struct OptimisticChangeMap<V> {
    inner: RwLock<HashMap<StoreName, HashMap<SerializedId, BTreeMap<OptimisticTime, Status<V>>>>>,
}

impl<V> Default for OptimisticChangeMap<V> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<V> OptimisticChangeMap<V> {
    pub fn insert<S: Store>(&self, id: &S::Id, v: V) -> OptimisticTime {
        let id = SerializedId::new_from_id::<S>(&id);
        let time = OptimisticTime::new();
        self.inner
            .write()
            .entry(S::NAME)
            .or_default()
            .entry(id)
            .or_default()
            .insert(time, v.into());
        time
    }

    pub fn remove<S: Store>(&self, id: &S::Id, time: &OptimisticTime) {
        let id = SerializedId::new_from_id::<S>(&id);
        let mut by_id_len = None;
        let mut inner = self.inner.write();
        if let Some(by_id) = inner.get_mut(&S::NAME) {
            let mut by_time_len = None;
            if let Some(by_time) = by_id.get_mut(&id) {
                by_time.remove(time);
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
    pub fn mark_successful<S: Store>(&self, id: &S::Id, time: &OptimisticTime) {
        let id = SerializedId::new_from_id::<S>(&id);
        self.inner
            .write()
            .get_mut(&S::NAME)
            .unwrap()
            .get_mut(&id)
            .unwrap()
            .get_mut(time)
            .unwrap()
            .mark_successful();
    }
}
impl<V: Clone> OptimisticChangeMap<V> {
    pub fn latest<S: Store>(&self, id: &S::Id) -> Option<Status<V>> {
        let id = SerializedId::new_from_id::<S>(&id);
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
