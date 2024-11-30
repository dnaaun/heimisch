use axum::{http::StatusCode, response::IntoResponse};
use backtrace::Backtrace;
use github_webhook_body::WebhookBody;
use shared::types::installation::InstallationId;
use utils::{ReqwestJsonError, ReqwestSendError};
use uuid::Uuid;

use crate::axum_helpers::extractors::HeaderError;

#[derive(Debug)]
pub enum DbIntegrityError {
    SessionsDataIsNotMap { session_id: Uuid, session_data: serde_json::Value },
}

impl From<DbIntegrityError> for Error {
    fn from(value: DbIntegrityError) -> Self {
        ErrorSource::DbIntegrity(value).into()
    }
}

#[derive(Debug)]
pub enum ErrorSource {
    DieselError(diesel::result::Error),
    DeadpoolInteractError(deadpool_diesel::InteractError),
    DeadpoolPoolError(deadpool_diesel::PoolError),
    Github(github_api::simple_error::SimpleError),
    ReqwestSendError(ReqwestSendError),
    ReqwestJsonError(ReqwestJsonError),
    GithubIdOutOfI64Bounds,
    GithubUserDetailsNotFound,
    HeaderError(HeaderError),
    InstallationIdNotFound(InstallationId),
    GithubWebhookNoInstallationId { body: WebhookBody },
    GithubWebhookHeaderError { message: String },
    GithubWebhookBodyDeser(serde_json::Error),
    // Db integrity errors
    DbIntegrity(DbIntegrityError),
}

#[derive(Debug)]
pub struct Error {
    source: ErrorSource,
    backtrace: Backtrace,
}

impl Error {
    pub fn new(source: ErrorSource) -> Self {
        Self {
            source,
            backtrace: Backtrace::new(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error {
            source: ErrorSource::DieselError(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Error {
            source: ErrorSource::DeadpoolInteractError(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<deadpool_diesel::PoolError> for Error {
    fn from(err: deadpool_diesel::PoolError) -> Self {
        Error {
            source: ErrorSource::DeadpoolPoolError(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl<T> From<github_api::apis::Error<T>> for Error {
    fn from(err: github_api::apis::Error<T>) -> Self {
        ErrorSource::Github(err.into()).into()
    }
}

impl From<ReqwestSendError> for Error {
    fn from(err: ReqwestSendError) -> Self {
        Error {
            source: ErrorSource::ReqwestSendError(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<ReqwestJsonError> for Error {
    fn from(err: ReqwestJsonError) -> Self {
        Error {
            source: ErrorSource::ReqwestJsonError(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<HeaderError> for Error {
    fn from(err: HeaderError) -> Self {
        Error {
            source: ErrorSource::HeaderError(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<ErrorSource> for Error {
    fn from(source: ErrorSource) -> Self {
        Error {
            source,
            backtrace: Backtrace::new(),
        }
    }
}

// TODO: Implement the prod/dev distinction.
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (code, error_msg) = match &self.source {
            // These GithubWebhook* errors will be emitted only for the webhook endpoint, and we don't return non
            // 2xx values there, because that could cause re-deliveries.
            ErrorSource::GithubWebhookHeaderError { message } => {
                let error_msg = format!("Error deserializing webhook headers: {message}");
                (StatusCode::OK, error_msg)
            }
            ErrorSource::GithubWebhookBodyDeser(err) => {
                let error_msg = format!("Error deserializing webhook body: {err:#?}");
                (StatusCode::OK, error_msg)
            }
            ErrorSource::GithubWebhookNoInstallationId { body } => {
                let error_msg = format!(
                    "Webhook body has no installation id: {}",
                    serde_json::to_string_pretty(&body).expect("")
                );
                (StatusCode::OK, error_msg)
            }
            ErrorSource::InstallationIdNotFound(id) => {
                let error_msg = format!("installation id not found: {id}");
                (StatusCode::UNAUTHORIZED, error_msg)
            }
            ErrorSource::HeaderError(header_error) => {
                let error_msg = match header_error {
                    HeaderError::Utf8(_) => "Header values were not utf8".to_owned(),
                    HeaderError::SerdeJson(err) => {
                        format!("Couldn't deserialize headers: {err:#?}")
                    }
                };

                (StatusCode::BAD_REQUEST, error_msg)
            }
            ErrorSource::ReqwestSendError(err) => {
                let error_msg = format!(
                    "Reqwest send error:
Url: {}
Payload: {:#?}
Reqwest reported error: {:?}
Backtrace:
{:?}
",
                    err.url, err.payload, err.request_error, self.backtrace
                );

                (StatusCode::INTERNAL_SERVER_ERROR, error_msg)
            }
            ErrorSource::ReqwestJsonError(err) => {
                let error_msg = format!(
                    "Reqwest json error:
Body: {:#?}
Reqwest error: {:?}
Deserialization error: {:?}
Backtrace:
{}
",
                    err.body,
                    err.reqwest_error,
                    err.serde_error,
                    print_backtrace_nicely(self.backtrace)
                );

                (StatusCode::INTERNAL_SERVER_ERROR, error_msg)
            }
            &ErrorSource::DieselError(_)
            | &ErrorSource::DeadpoolPoolError(_)
            | &ErrorSource::DeadpoolInteractError(_)
            | &ErrorSource::Github(_)
            | &ErrorSource::GithubIdOutOfI64Bounds
            | &ErrorSource::DbIntegrity(_)
            | &ErrorSource::GithubUserDetailsNotFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "{:?}
{}",
                    self.source,
                    print_backtrace_nicely(self.backtrace)
                ),
            ),
        };

        tracing::error!("{}", error_msg);

        (code, error_msg).into_response()
    }
}

/// Filters backtrace frames to those in our codebase.
fn print_backtrace_nicely(backtrace: Backtrace) -> String {
    let frames = backtrace
        .frames()
        .iter()
        .filter(|frame| {
            frame.symbols().iter().any(|symbol| {
                symbol
                    .filename()
                    .map(|filename| filename.to_str())
                    .flatten()
                    .map(|filename| filename.contains("heimisch")) // TODO: change this to be more robust?
                    .unwrap_or(false)
            })
        })
        .skip(1) // The first item of the backtrace is going to be `Backtrace::new()`
        .map(|i| i.clone())
        .collect::<Vec<_>>();

    let shorter_bt = Backtrace::from(frames);

    format!("{:?}", shorter_bt)
}
