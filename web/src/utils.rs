use std::rc::Rc;

pub fn rc<T>(t: T) -> Rc<T> {
    Rc::new(t)
}
