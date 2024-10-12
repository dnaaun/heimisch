#[derive(Debug)]
pub enum Error {
    Idb(idb::Error),
    SerdeToObject(crate::serde_abstraction::Error),
    SerdeToString(serde_json::Error)
}

impl From<idb::Error> for Error {
    fn from(err: idb::Error) -> Self {
        Error::Idb(err)
    }
}

// Implement `From` trait for `serde::de::Error`
impl From<crate::serde_abstraction::Error> for Error {
    fn from(err: crate::serde_abstraction::Error) -> Self {
        Error::SerdeToObject(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeToString(err)
    }
}
