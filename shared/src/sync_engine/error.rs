use crate::{avail::MergeError, endpoints::endpoint_client::OwnApiError};
use std::fmt::Debug;

use super::{
    conversions::conversion_error::ConversionError,
    typed_transport::{TypedTransportError, TypedTransportTrait},
};

pub enum SyncErrorSrc<T: TypedTransportTrait> {
    OwnApi(OwnApiError),
    Github(github_api::simple_error::SimpleError),
    Db(idb::Error),
    SerdeToObject(typesafe_idb::serde_abstraction::Error),
    SerdeToString(serde_json::Error),
    Jiff(jiff::Error),
    Merge(MergeError),
    Ewebsock(ewebsock::Error),
    /// These are things like: the user that owns a repository in our db not existing in our db.
    DataModel(String),
    WebSocket(TypedTransportError<T::ConnError>),
}

impl<T: TypedTransportTrait> Debug for SyncErrorSrc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: TypedTransportTrait> From<SyncErrorSrc<T>> for SyncError<T> {
    fn from(value: SyncErrorSrc<T>) -> Self {
        Self {
            source: value,
            backtrace: std::backtrace::Backtrace::force_capture(),
        }
    }
}

#[allow(dead_code)]
pub struct SyncError<T: TypedTransportTrait> {
    source: SyncErrorSrc<T>,
    backtrace: std::backtrace::Backtrace,
}

impl<T: TypedTransportTrait> Debug for SyncError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: TypedTransportTrait> From<ConversionError> for SyncError<T> {
    fn from(value: ConversionError) -> Self {
        match value {
            ConversionError::Merge(merge_error) => SyncErrorSrc::Merge(merge_error),
            ConversionError::Json(error) => SyncErrorSrc::SerdeToString(error),
            ConversionError::Jiff(err) => SyncErrorSrc::Jiff(err),
        }
        .into()
    }
}

impl<T: TypedTransportTrait> SyncError<T> {
    /// We don't derive `From<>` because a `String` might accidentally get converted (which is what
    /// `ewebsock::Error` really is).
    pub fn from_ewebsock(error: ewebsock::Error) -> Self {
        SyncErrorSrc::Ewebsock(error).into()
    }
}

pub type SyncResult<T, W> = Result<T, SyncError<W>>;

impl<W: TypedTransportTrait, T> From<github_api::apis::Error<T>> for SyncError<W> {
    fn from(value: github_api::apis::Error<T>) -> Self {
        SyncErrorSrc::Github(value.into()).into()
    }
}

impl<W: TypedTransportTrait> From<TypedTransportError<W::ConnError>> for SyncError<W> {
    fn from(value: TypedTransportError<W::ConnError>) -> Self {
        SyncErrorSrc::WebSocket(value).into()
    }
}

impl<W: TypedTransportTrait> From<OwnApiError> for SyncError<W> {
    fn from(value: OwnApiError) -> Self {
        SyncErrorSrc::OwnApi(value).into()
    }
}

impl<W: TypedTransportTrait> From<idb::Error> for SyncError<W> {
    fn from(value: idb::Error) -> Self {
        SyncErrorSrc::Db(value).into()
    }
}

impl<W: TypedTransportTrait> From<serde_json::Error> for SyncError<W> {
    fn from(value: serde_json::Error) -> Self {
        SyncErrorSrc::SerdeToString(value).into()
    }
}

impl<W: TypedTransportTrait> From<typesafe_idb::Error> for SyncError<W> {
    fn from(value: typesafe_idb::Error) -> Self {
        match value {
            typesafe_idb::Error::Idb(error) => SyncErrorSrc::Db(error).into(),
            typesafe_idb::Error::SerdeToObject(error) => SyncErrorSrc::SerdeToObject(error).into(),
            typesafe_idb::Error::SerdeToString(error) => SyncErrorSrc::SerdeToString(error).into(),
        }
    }
}

impl<W: TypedTransportTrait> From<MergeError> for SyncError<W> {
    fn from(value: MergeError) -> Self {
        SyncErrorSrc::Merge(value).into()
    }
}
