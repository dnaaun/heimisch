use std::{hash::Hash, sync::Arc};

use send_wrapper::SendWrapper;

#[derive(Debug, derive_more::Display, Clone)]
pub enum FrontendError {
    #[display("Indexeddb Error: {_0:?}")]
    Idb(Arc<SendWrapper<typesafe_idb::Error>>),
}

impl Hash for FrontendError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}

impl PartialEq for FrontendError {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Eq for FrontendError {}

impl std::error::Error for FrontendError {}

impl From<&FrontendError> for FrontendError {
    fn from(value: &FrontendError) -> Self {
        value.clone()
    }
}

impl From<typesafe_idb::Error> for FrontendError {
    fn from(value: typesafe_idb::Error) -> Self {
        FrontendError::Idb(Arc::new(SendWrapper::new(value)))
    }
}
