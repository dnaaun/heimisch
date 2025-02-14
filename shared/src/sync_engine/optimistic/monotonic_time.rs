#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Copy, Eq)]
pub struct MonotonicTime {
    inner: i64,
}

impl MonotonicTime {
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

impl Default for MonotonicTime {
    fn default() -> Self {
        Self::new()
    }
}
