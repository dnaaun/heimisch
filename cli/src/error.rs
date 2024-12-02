use std::{backtrace::Backtrace, fmt::Display};

#[derive(Debug)]
pub struct Error {
    source: ErrorSource,
    backtrace: Backtrace,
}

#[derive(Debug)]
pub enum ErrorSource {
    Diesel(diesel::result::Error),
    DeadpoolInteract(deadpool_diesel::InteractError),
    DeadpoolPool(deadpool_diesel::PoolError),
    BackendApi(reqwest::Error),
}

impl Error {
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error {
            source: ErrorSource::Diesel(err),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Error {
            source: ErrorSource::DeadpoolInteract(err),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<deadpool_diesel::PoolError> for Error {
    fn from(err: deadpool_diesel::PoolError) -> Self {
        Error {
            source: ErrorSource::DeadpoolPool(err),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            source: ErrorSource::BackendApi(err),
            backtrace: Backtrace::capture(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)
    }
}

impl Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSource::Diesel(err) => err.fmt(f),
            ErrorSource::DeadpoolInteract(err) => err.fmt(f),
            ErrorSource::DeadpoolPool(err) => err.fmt(f),
            ErrorSource::BackendApi(err) => {
                let mut fmt_result = err.fmt(f);
                if let Some(url) = err.url() {
                    fmt_result = fmt_result.and(
                        f.write_fmt(format_args!("Error has to do with url: {}", url.as_str())),
                    )
                }
                fmt_result
            }
        }
    }
}
