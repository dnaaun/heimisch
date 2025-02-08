use std::panic::Location;

#[derive(Debug)]
pub struct Error {
    inner: typesafe_idb::Error,
    txn_location: &'static Location<'static>,
}

impl Error {
    pub fn new(inner: typesafe_idb::Error, txn_location: &'static Location<'static>) -> Self {
        Self { inner, txn_location }
    }
}