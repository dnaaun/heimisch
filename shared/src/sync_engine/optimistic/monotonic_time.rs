#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Copy, Eq)]
pub struct OptimisticTime {
    inner: i64,
}

impl OptimisticTime {
    #[cfg(not(feature = "hydrate"))]
    pub fn new() -> Self {
        todo!()
    }
    #[cfg(feature = "hydrate")]
    pub fn new() -> Self {
        use leptos::prelude::window;
        Self {
            inner: window().performance().unwrap().now() as i64,
        }
    }
}
