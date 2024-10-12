/// Only difference with the `Error` from `github_api::apis::Error` is that it's not
/// generic over `T`. (and so doesn't contain a parsed value of `T`).
///
/// I am doing this to avoid the pain of converting
#[derive(Debug)]
pub enum SimpleError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent),
}

#[derive(Debug, Clone)]
pub struct ResponseContent {
    pub status: reqwest::StatusCode,
    pub content: String,
}

impl<T> From<crate::apis::Error<T>> for SimpleError {
    fn from(value: crate::apis::Error<T>) -> Self {
        match value {
            crate::apis::Error::Reqwest(error) => Self::Reqwest(error),
            crate::apis::Error::Serde(error) => Self::Serde(error),
            crate::apis::Error::Io(error) => Self::Io(error),
            crate::apis::Error::ResponseError(response_content) => {
                Self::ResponseError(ResponseContent {
                    status: response_content.status,
                    content: response_content.content,
                })
            }
        }
    }
}
