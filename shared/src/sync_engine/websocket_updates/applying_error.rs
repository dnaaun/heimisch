use crate::sync_engine::error::SyncError;
use std::fmt::Debug;

use super::transport::TransportTrait;

#[derive(Debug)]
pub enum ApplyingError<W: TransportTrait> {
    Sync(SyncError<W>),

    /// It's gonna take us a while to implement how all the github webhook update variants
    /// translate to local db changes. In the mean time, we return this.
    NotImplemented,
}

pub type ApplyingResult<T, W> = Result<T, ApplyingError<W>>;

impl<W: TransportTrait, T: Into<SyncError<W>>> From<T> for ApplyingError<W> {
    fn from(value: T) -> Self {
        Self::Sync(value.into())
    }
}
