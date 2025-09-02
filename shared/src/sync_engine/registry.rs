use derivative::Derivative;
use std::{collections::HashMap, sync::Arc};

use idalloc::Slab;
use parking_lot::Mutex;

struct Inner<T> {
    alloc: Slab<u32>,
    map: HashMap<u32, T>,
}

/// The idea is that one calls `.add()` to add an object to the registry, and it
/// will return a function that will remove it from the registry when called.
/// And then one can do `registry.get()` to get all the objects in the registry.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Registry<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

/// We want to avoid requiring `T: Default` here, so we can't derive this.
impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                alloc: Default::default(),
                map: Default::default(),
            })),
        }
    }
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<T: Send + Sync> Registry<T> {
    /// Will return the function that will remove it from the registry
    pub fn add(&self, t: T) -> impl Fn() + Send + Sync {
        let id = {
            let mut inner = self.inner.lock();
            let id = inner.alloc.next();
            inner.map.insert(id, t);
            id
        };
        let inner = self.inner.clone();

        move || {
            let _item = inner.lock().map.remove(&id);
        }
    }
}

impl<T: Clone> Registry<T> {
    pub fn get(&self) -> Vec<T> {
        self.inner.lock().map.values().cloned().collect()
    }
}
