use std::{collections::HashMap, sync::Arc};

use idalloc::Slab;
use parking_lot::Mutex;
use send_wrapper::SendWrapper;

pub struct Registry<T> {
    alloc: Slab<u32>,
    map: Arc<Mutex<HashMap<u32, SendWrapper<T>>>>,
}

/// We want to avoid requiring `T: Default` here, so we can't derive this.
impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            alloc: Default::default(),
            map: Default::default(),
        }
    }
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            alloc: Default::default(),
            map: Default::default(),
        }
    }

    /// Will return the function that will remove it from the registry
    pub fn add(&mut self, t: T) -> impl Fn() + Send + Sync {
        let id = self.alloc.next();
        self.map.lock().insert(id, SendWrapper::new(t));

        let map = self.map.clone();

        move || {
            map.lock().remove(&id);
        }
    }
}
