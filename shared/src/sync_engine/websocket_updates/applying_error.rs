use crate::sync_engine::{error::SyncError, typed_transport::TypedTransportTrait};
use std::fmt::Debug;

#[derive(Debug)]
pub enum ApplyingError<W: TypedTransportTrait> {
    Sync(SyncError<W>),

    /// It's gonna take us a while to implement how all the github webhook update variants
    /// translate to local db changes. In the mean time, we return this.
    NotImplemented,
}

pub type ApplyingResult<T, W> = Result<T, ApplyingError<W>>;

impl<W: TypedTransportTrait, T: Into<SyncError<W>>> From<T> for ApplyingError<W> {
    fn from(value: T) -> Self {
        Self::Sync(value.into())
    }
}
