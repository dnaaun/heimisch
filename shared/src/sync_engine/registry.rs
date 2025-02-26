use std::{cell::RefCell, collections::HashMap, rc::Rc};

use idalloc::Slab;
use send_wrapper::SendWrapper;

struct Inner<T> {
    alloc: Slab<u32>,
    map: HashMap<u32, T>,
}

/// The idea is that one calls `.add()` to add an object to the registry, and it
/// will return a function that will remove it from the registry when called.
/// And then one can do `registry.get()` to get all the objects in the registry.
pub struct Registry<T> {
    inner: SendWrapper<Rc<RefCell<Inner<T>>>>,
}

/// We want to avoid requiring `T: Default` here, so we can't derive this.
impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            inner: SendWrapper::new(Rc::new(RefCell::new(Inner {
                alloc: Default::default(),
                map: Default::default(),
            }))),
        }
    }
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<T> Registry<T> {
    /// Will return the function that will remove it from the registry
    pub fn add(&self, t: T) -> impl Fn() + Send + Sync {
        let id = {
            let mut inner = self.inner.borrow_mut();
            let id = inner.alloc.next();
            inner.map.insert(id, t);
            id
        };
        let inner = self.inner.clone();

        move || {
            let item = inner.borrow_mut().map.remove(&id);
        }
    }
}

impl<T: Clone> Registry<T> {
    pub fn get(&self) -> Vec<T> {
        self.inner.borrow().map.values().cloned().collect()
    }
}
