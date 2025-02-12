use crate::{avail::{MergeError, NotAvailableError}, endpoints::endpoint_client::OwnApiError};
use std::{fmt::Debug, panic::Location};

use super::{
    conversions::conversion_error::ConversionError,
    optimistic::db::Error,
    websocket_updates::transport::TransportTrait,
};

pub enum SyncErrorSrc<T: TransportTrait> {
    OwnApi(OwnApiError),
    Github(github_api::simple_error::SimpleError),
    Db(Error),
    SerdeToObject(typesafe_idb::serde_abstraction::Error),
    SerdeToString(serde_json::Error),
    Jiff(jiff::Error),
    Merge(MergeError),
    Ewebsock(ewebsock::Error),
    /// These are things like: the user that owns a repository in our db not existing in our db.
    DataModel(String),
    Transport(T::TransportError),
    NotAvailable(NotAvailableError)
}

impl<T: TransportTrait> Debug for SyncErrorSrc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncErrorSrc::OwnApi(err) => write!(f, "SyncErrorSrc::OwnApi({:?})", err),
            SyncErrorSrc::Github(err) => write!(f, "SyncErrorSrc::Github({:?})", err),
            SyncErrorSrc::Db(err) => write!(f, "SyncErrorSrc::Db({:?})", err),
            SyncErrorSrc::SerdeToObject(err) => write!(f, "SyncErrorSrc::SerdeToObject({:?})", err),
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

impl<T: TransportTrait> From<SyncErrorSrc<T>> for SyncError<T> {
    #[track_caller]
    fn from(value: SyncErrorSrc<T>) -> Self {
        Self {
            source: value,
            location: Location::caller(),
        }
    }
}

#[allow(dead_code)]
pub struct SyncError<T: TransportTrait> {
    source: SyncErrorSrc<T>,
    location: &'static Location<'static>
}

impl<T: TransportTrait> Debug for SyncError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncError")
            .field("source", &self.source)
            .field("backtrace", &self.location)
            .finish()
    }
}

impl<T: TransportTrait> From<ConversionError> for SyncError<T> {
    fn from(value: ConversionError) -> Self {
        match value {
            ConversionError::Merge(merge_error) => SyncErrorSrc::Merge(merge_error),
            ConversionError::Json(error) => SyncErrorSrc::SerdeToString(error),
            ConversionError::Jiff(err) => SyncErrorSrc::Jiff(err),
        }
        .into()
    }
}

impl<T: TransportTrait> SyncError<T> {
    /// We don't derive `From<>` because a `String` might accidentally get converted (which is what
    /// `ewebsock::Error` really is).
    pub fn from_ewebsock(error: ewebsock::Error) -> Self {
        SyncErrorSrc::Ewebsock(error).into()
    }
}

pub type SyncResult<T, W> = Result<T, SyncError<W>>;

impl<W: TransportTrait, T> From<github_api::apis::Error<T>> for SyncError<W> {
    #[track_caller]
    fn from(value: github_api::apis::Error<T>) -> Self {
        SyncErrorSrc::Github(value.into()).into()
    }
}


impl<W: TransportTrait> From<OwnApiError> for SyncError<W> {
    #[track_caller]
    fn from(value: OwnApiError) -> Self {
        SyncErrorSrc::OwnApi(value).into()
    }
}

impl<W: TransportTrait> From<Error> for SyncError<W> {
    #[track_caller]
    fn from(value: Error) -> Self {
        SyncErrorSrc::Db(value).into()
    }
}

impl<W: TransportTrait> From<serde_json::Error> for SyncError<W> {
    #[track_caller]
    fn from(value: serde_json::Error) -> Self {
        SyncErrorSrc::SerdeToString(value).into()
    }
}
impl<W: TransportTrait> From<MergeError> for SyncError<W> {
    #[track_caller]
    fn from(value: MergeError) -> Self {
        SyncErrorSrc::Merge(value).into()
    }
}

impl<W: TransportTrait> From<NotAvailableError> for SyncError<W> {
    #[track_caller]
    fn from(value: NotAvailableError) -> Self {
        SyncErrorSrc::NotAvailable(value).into()
    }
}
