#[derive(Debug)]
pub enum ConversionError {
    Merge(crate::avail::MergeError),
    Json(serde_json::Error),
    Jiff(jiff::Error),
}

impl From<crate::avail::MergeError> for ConversionError {
    fn from(value: crate::avail::MergeError) -> Self {
        Self::Merge(value)
    }
}

impl From<serde_json::Error> for ConversionError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<jiff::Error> for ConversionError {
    fn from(value: jiff::Error) -> Self {
        Self::Jiff(value)
    }
}
