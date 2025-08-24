use std::panic::Location;

#[derive(Debug)]
pub struct Error {
    #[allow(unused)]
    inner: typesafe_idb::Error,
    #[allow(unused)]
    txn_location: &'static Location<'static>,
}

impl Error {
    pub fn new(inner: typesafe_idb::Error, txn_location: &'static Location<'static>) -> Self {
        Self {
            inner,
            txn_location,
        }
    }
}
