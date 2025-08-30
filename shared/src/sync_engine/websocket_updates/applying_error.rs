use typed_db::RawDbTrait;

use crate::sync_engine::error::SyncError;
use std::fmt::Debug;

use super::transport::TransportTrait;

#[derive(Debug)]
pub enum ApplyingError<W: TransportTrait, RawDb: RawDbTrait> {
    Sync(SyncError<W, RawDb>),

    /// It's gonna take us a while to implement how all the github webhook update variants
    /// translate to local db changes. In the mean time, we return this.
    NotImplemented,
}

pub type ApplyingResult<T, Transport, RawDb: RawDbTrait> =
    Result<T, ApplyingError<Transport, RawDb>>;

impl<RawDb, Transport, T> From<T> for ApplyingError<Transport, RawDb>
where
    RawDb: RawDbTrait,
    Transport: TransportTrait,
    T: Into<SyncError<Transport, RawDb>>,
{
    fn from(value: T) -> Self {
        Self::Sync(value.into())
    }
}
