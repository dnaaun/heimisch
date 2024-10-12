pub enum Error {
    DieselError(diesel::result::Error),
    DeadpoolError(deadpool_diesel::InteractError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::DieselError(err)
    }
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Error::DeadpoolError(err)
    }
}


