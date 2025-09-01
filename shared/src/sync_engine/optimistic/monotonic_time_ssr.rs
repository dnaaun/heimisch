use std::time::Duration;

#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Copy, Eq)]
pub struct MonotonicTime {
    inner: Duration,
}

impl MonotonicTime {
    pub fn new() -> Self {
        Self {
            inner: std::time::Instant::now().elapsed(),
        }
    }
}

impl Default for MonotonicTime {
    fn default() -> Self {
        Self::new()
    }
}
