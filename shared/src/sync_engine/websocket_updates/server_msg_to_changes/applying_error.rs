use std::fmt::Debug;
use crate::sync_engine::error::SyncError;

#[derive(Debug)]
pub enum ApplyingError<W: Debug> {
    Sync(SyncError<W>),

    /// It's gonna take us a while to implement how all the github webhook update variants
    /// translate to local db changes. In the mean time, we return this.
    NotImplemented,
}

pub type ApplyingResult<T, W> = Result<T, ApplyingError<W>>;

impl<W: Debug> From<SyncError<W>> for ApplyingError<W> {
    fn from(value: SyncError<W>) -> Self {
        ApplyingError::Sync(value)
    }
}
