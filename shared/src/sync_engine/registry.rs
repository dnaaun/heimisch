use std::{cell::RefCell, collections::HashMap, rc::Rc};

use idalloc::Slab;
use send_wrapper::SendWrapper;

pub struct Registry<T> {
    alloc: Slab<u32>,
    map: SendWrapper<Rc<RefCell<HashMap<u32, T>>>>,
}

/// We want to avoid requiring `T: Default` here, so we can't derive this.
impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            alloc: Default::default(),
            map: SendWrapper::new(Rc::new(Default::default())),
        }
    }
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Will return the function that will remove it from the registry
    pub fn add(&mut self, t: T) -> impl Fn() + Send + Sync {
        let id = self.alloc.next();
        self.map.borrow_mut().insert(id, t);

        let map = self.map.clone();

        move || {
            map.borrow_mut().remove(&id);
        }
    }
}
