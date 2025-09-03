use std::{hash::Hash, sync::Arc};

use utils::JustSend;

#[derive(Debug, derive_more::Display, Clone)]
pub enum FrontendError {
    #[display("Indexeddb Error: {_0:?}")]
    Idb(Arc<JustSend<typed_db::idb_impl::Error>>),
}

/// I only implement this so that I can hash and memoize Result<_, FrontendError>,
/// so don't do things like creating a `HashSet<FrontendError>`.
impl Hash for FrontendError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state)
    }
}

impl PartialEq for FrontendError {
    fn eq(&self, _other: &Self) -> bool {
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

impl From<typed_db::idb_impl::Error> for FrontendError {
    fn from(value: typed_db::idb_impl::Error) -> Self {
        FrontendError::Idb(Arc::new(JustSend::new(value)))
    }
}
