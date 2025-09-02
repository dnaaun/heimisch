use std::sync::Arc;

use super::endpoint::{GetEndpoint, PostEndpoint};
use http::{HeaderName, StatusCode};
use reqwest::{Client, ClientBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
use utils::{ExecuteNicely, ReqwestSendError};

thread_local! {
    pub static CUSTOM_REDIRECT_STATUS_CODE: StatusCode= StatusCode::from_u16(399).expect("");
}
pub const CUSTOM_REDIRECT_HEADER_NAME: HeaderName = HeaderName::from_static("x-custom-location");

/// I had to have an error type here to make the `make_request` below work with
/// `leptos::Resource::new()`.
#[derive(Clone, Debug)]
pub enum OwnApiError {
    ReqwestError {
        error: Arc<dyn std::error::Error + Send + Sync>,
    },
    PageRedirect,
    UrlParamsEncode(serde_urlencoded::ser::Error),
    UrlParamsDecode(serde_urlencoded::de::Error),
}

impl From<serde_urlencoded::ser::Error> for OwnApiError {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::UrlParamsEncode(value)
    }
}

impl From<serde_urlencoded::de::Error> for OwnApiError {
    fn from(value: serde_urlencoded::de::Error) -> Self {
        Self::UrlParamsDecode(value)
    }
}

impl From<reqwest::Error> for OwnApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError {
            error: Arc::new(value),
        }
    }
}

impl From<ReqwestSendError> for OwnApiError {
    fn from(value: ReqwestSendError) -> Self {
        Self::ReqwestError {
            error: Arc::new(value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaybePageRedirect<T> {
    PageRedirectTo(Url),
    NoRedirect(T),
}

impl<T> From<T> for MaybePageRedirect<T> {
    fn from(value: T) -> Self {
        Self::NoRedirect(value)
    }
}

#[derive(Clone)]
pub struct EndpointClient {
    redirect_handler: Arc<dyn Fn(Url) + Send + Sync>,
    pub client: Client,
    pub domain: Url,
}

impl EndpointClient {
    pub fn new(redirect_handler: impl Fn(Url) + Send + Sync + 'static, domain: Url) -> Self {
        Self {
            redirect_handler: Arc::new(redirect_handler),
            client: ClientBuilder::new().build().expect(""),
            domain,
        }
    }

    // NOTE: Abstract away common functionality into a common function between make_get_request and
    // make_post_request.

    pub async fn make_get_request<T, E>(
        &self,
        _: E,
        query_params: <E as GetEndpoint>::QueryParams,
    ) -> Result<T, OwnApiError>
    where
        T: DeserializeOwned,
        E: GetEndpoint<JsonResponse = T>,
    {
        let mut url = self.domain.clone();
        url.set_path(E::PATH.to_string().as_str());
        url.set_query(Some(&serde_urlencoded::to_string(query_params)?));

        #[allow(unused_mut)]
        let mut request = self.client.get(url.clone());
        #[cfg(target_arch = "wasm32")]
        {
            request = request.fetch_credentials_include();
        }

        let response = self.client.execute_nicely(request.build().unwrap()).await?;

        if response.status() == CUSTOM_REDIRECT_STATUS_CODE.with(|i| *i) {
            let location = match response.headers().get(CUSTOM_REDIRECT_HEADER_NAME) {
                Some(l) => l,
                None => {
                    panic!(
                        "Redirect had no location header. Headers were: {:?}",
                        response.headers()
                    )
                }
            };
            let location = location
                .to_str()
                .expect("Redirect location was not a valid string")
                .parse()
                .expect("Redirect location not a valid URL.");
            (self.redirect_handler)(location);
            return Err(OwnApiError::PageRedirect);
        }

        Ok(response.json::<T>().await?)
    }

    pub async fn make_post_request<T, E>(
        &self,
        _: E,
        payload: <E as PostEndpoint>::JsonPayload,
        query_params: <E as PostEndpoint>::QueryParams,
    ) -> Result<T, OwnApiError>
    where
        T: DeserializeOwned,
        E: PostEndpoint<JsonResponse = T>,
    {
        let mut url = self.domain.clone();

        url.set_path(E::PATH.to_string().as_str());
        url.set_query(Some(&serde_urlencoded::to_string(query_params)?));

        let mut request = self.client.post(url.clone());
        request = request.json(&payload);
        #[cfg(target_arch = "wasm32")]
        {
            request = request.fetch_credentials_include();
        }

        let response = self.client.execute_nicely(request.build().unwrap()).await?;

        if response.status() == CUSTOM_REDIRECT_STATUS_CODE.with(|i| *i) {
            let location = match response.headers().get(CUSTOM_REDIRECT_HEADER_NAME) {
                Some(l) => l,
                None => {
                    panic!(
                        "Redirect had no location header. Headers were: {:?}",
                        response.headers()
                    )
                }
            };
            let location = location
                .to_str()
                .expect("Redirect location was not a valid string")
                .parse()
                .expect("Redirect location not a valid URL.");
            (self.redirect_handler)(location);
            return Err(OwnApiError::PageRedirect);
        }

        Ok(response.json::<T>().await?)
    }
}
