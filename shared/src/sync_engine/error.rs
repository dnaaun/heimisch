use crate::{avail::MergeError, endpoints::endpoint_request::OwnApiError};

#[derive(Debug)]
pub enum SyncErrorSrc {
    OwnApi(OwnApiError),
    Github(github_api::simple_error::SimpleError),
    Db(idb::Error),
    SerdeToObject(typesafe_idb::serde_abstraction::Error),
    SerdeToString(serde_json::Error),
    Merge(MergeError),

    /// These are things like: the user that owns a repository in our db not existing in our db.
    DataModel(String),
}

impl From<SyncErrorSrc> for SyncError {
    fn from(value: SyncErrorSrc) -> Self {
        Self {
            source: value,
            backtrace: std::backtrace::Backtrace::force_capture(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SyncError {
    source: SyncErrorSrc,
    backtrace: std::backtrace::Backtrace,
}

pub type SyncResult<T> = Result<T, SyncError>;

impl<T> From<github_api::apis::Error<T>> for SyncError {
    fn from(value: github_api::apis::Error<T>) -> Self {
        SyncErrorSrc::Github(value.into()).into()
    }
}

impl From<OwnApiError> for SyncError {
    fn from(value: OwnApiError) -> Self {
        SyncErrorSrc::OwnApi(value).into()
    }
}

impl From<idb::Error> for SyncError {
    fn from(value: idb::Error) -> Self {
        SyncErrorSrc::Db(value).into()
    }
}

impl From<typesafe_idb::Error> for SyncError {
    fn from(value: typesafe_idb::Error) -> Self {
        match value {
            typesafe_idb::Error::Idb(error) => SyncErrorSrc::Db(error).into(),
            typesafe_idb::Error::SerdeToObject(error) => SyncErrorSrc::SerdeToObject(error).into(),
            typesafe_idb::Error::SerdeToString(error) => SyncErrorSrc::SerdeToString(error).into(),
        }
    }
}

impl From<MergeError> for SyncError {
    fn from(value: MergeError) -> Self {
        SyncErrorSrc::Merge(value).into()
    }
}
