use std::sync::Arc;

use send_wrapper::SendWrapper;

#[derive(Debug, derive_more::Display, Clone)]
pub enum FrontendError {
    #[display("Indexeddb Error: {_0:?}")]
    Idb(Arc<SendWrapper<typesafe_idb::Error>>),
}

impl std::error::Error for FrontendError {}

impl From<typesafe_idb::Error> for FrontendError {
    fn from(value: typesafe_idb::Error) -> Self {
        FrontendError::Idb(Arc::new(SendWrapper::new(value)))
    }
}
