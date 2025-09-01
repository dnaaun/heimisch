use typed_db::RawDbTrait;

use crate::{
    avail::{MergeError, NotAvailableError},
    endpoints::endpoint_client::OwnApiError,
};
use std::{fmt::Debug, panic::Location};

use super::{
    conversions::conversion_error::ConversionError, websocket_updates::transport::TransportTrait,
};

pub enum SyncErrorSrc<Transport: TransportTrait, RawDb: RawDbTrait> {
    OwnApi(OwnApiError),
    Github(github_api::simple_error::SimpleError),
    Db(RawDb::Error),
    SerdeToString(serde_json::Error),
    Jiff(jiff::Error),
    Merge(MergeError),
    Ewebsock(ewebsock::Error),
    /// These are things like: the user that owns a repository in our db not existing in our db.
    DataModel(String),
    Transport(Transport::TransportError),
    NotAvailable(NotAvailableError),
}

impl<T: TransportTrait, RawDb: RawDbTrait> Debug for SyncErrorSrc<T, RawDb> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncErrorSrc::OwnApi(err) => write!(f, "SyncErrorSrc::OwnApi({:?})", err),
            SyncErrorSrc::Github(err) => write!(f, "SyncErrorSrc::Github({:?})", err),
            SyncErrorSrc::Db(err) => write!(f, "SyncErrorSrc::Db({:?})", err),
            SyncErrorSrc::SerdeToString(err) => write!(f, "SyncErrorSrc::SerdeToString({:?})", err),
            SyncErrorSrc::Jiff(err) => write!(f, "SyncErrorSrc::Jiff({:?})", err),
            SyncErrorSrc::Merge(err) => write!(f, "SyncErrorSrc::Merge({:?})", err),
            SyncErrorSrc::Ewebsock(err) => write!(f, "SyncErrorSrc::Ewebsock({:?})", err),
            SyncErrorSrc::DataModel(msg) => write!(f, "SyncErrorSrc::DataModel({})", msg),
            SyncErrorSrc::Transport(err) => write!(f, "SyncErrorSrc::WebSocket({:?})", err),
            SyncErrorSrc::NotAvailable(err) => write!(f, "SyncErrorSrc::NotAvailable({:?})", err),
        }
    }
}

impl<T: TransportTrait, RawDb: RawDbTrait> From<SyncErrorSrc<T, RawDb>> for SyncError<T, RawDb> {
    #[track_caller]
    fn from(value: SyncErrorSrc<T, RawDb>) -> Self {
        Self {
            source: value,
            location: Location::caller(),
        }
    }
}

#[allow(dead_code)]
pub struct SyncError<Transport: TransportTrait, RawDb: RawDbTrait> {
    source: SyncErrorSrc<Transport, RawDb>,
    location: &'static Location<'static>,
}

impl<Transport: TransportTrait, RawDb: RawDbTrait> Debug for SyncError<Transport, RawDb> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncError")
            .field("source", &self.source)
            .field("backtrace", &self.location)
            .finish()
    }
}

impl<Transport: TransportTrait, RawDb: RawDbTrait> From<ConversionError>
    for SyncError<Transport, RawDb>
{
    fn from(value: ConversionError) -> Self {
        match value {
            ConversionError::Merge(merge_error) => SyncErrorSrc::Merge(merge_error),
            ConversionError::Json(error) => SyncErrorSrc::SerdeToString(error),
            ConversionError::Jiff(err) => SyncErrorSrc::Jiff(err),
        }
        .into()
    }
}

impl<Transport: TransportTrait, RawDb: RawDbTrait> SyncError<Transport, RawDb> {
    /// We don't derive `From<>` because a `String` might accidentally get converted (which is what
    /// `ewebsock::Error` really is).
    pub fn from_ewebsock(error: ewebsock::Error) -> Self {
        SyncErrorSrc::Ewebsock(error).into()
    }
}

pub type SyncResult<T, Transport, RawDb> = Result<T, SyncError<Transport, RawDb>>;

impl<W: TransportTrait, T, RawDb: RawDbTrait> From<github_api::apis::Error<T>>
    for SyncError<W, RawDb>
{
    #[track_caller]
    fn from(value: github_api::apis::Error<T>) -> Self {
        SyncErrorSrc::Github(value.into()).into()
    }
}

impl<W: TransportTrait, RawDb: RawDbTrait> From<OwnApiError> for SyncError<W, RawDb> {
    #[track_caller]
    fn from(value: OwnApiError) -> Self {
        SyncErrorSrc::OwnApi(value).into()
    }
}

impl<W: TransportTrait, RawDb: RawDbTrait> From<serde_json::Error> for SyncError<W, RawDb> {
    #[track_caller]
    fn from(value: serde_json::Error) -> Self {
        SyncErrorSrc::SerdeToString(value).into()
    }
}
impl<W: TransportTrait, RawDb: RawDbTrait> From<MergeError> for SyncError<W, RawDb> {
    #[track_caller]
    fn from(value: MergeError) -> Self {
        SyncErrorSrc::Merge(value).into()
    }
}

impl<W: TransportTrait, RawDb: RawDbTrait> From<NotAvailableError> for SyncError<W, RawDb> {
    #[track_caller]
    fn from(value: NotAvailableError) -> Self {
        SyncErrorSrc::NotAvailable(value).into()
    }
}

pub trait RawDbErrorToSyncError<I, Transport: TransportTrait, RawDb: RawDbTrait> {
    /// I cannot `impl<RawDb: RawDbTrait> From<RawDb::Error>` because of that Rust rule that prevents
    /// multiple impls of the same trait for the same type.
    /// So this trait is supposed to be a nicer substitute.
    fn tse(self) -> Result<I, SyncError<Transport, RawDb>>;
}

impl<I, Db: RawDbTrait, Transport: TransportTrait> RawDbErrorToSyncError<I, Transport, Db>
    for Result<I, Db::Error>
{
    fn tse(self) -> Result<I, SyncError<Transport, Db>> {
        match self {
            Ok(i) => Ok(i),
            Err(e) => Err(SyncErrorSrc::Db(e).into()),
        }
    }
}
