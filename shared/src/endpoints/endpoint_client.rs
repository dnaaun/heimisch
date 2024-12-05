use std::sync::Arc;

use super::endpoint::{Endpoint, Method, QueryParams};
use http::{header::LOCATION, StatusCode};
use reqwest::{Client, ClientBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

thread_local! {
    pub static CUSTOM_REDIRECT_STATUS_CODE: StatusCode= StatusCode::from_u16(399).expect("");
}

/// I had to have an error type here to make the `make_request` below work with
/// `leptos::Resource::new()`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OwnApiError {
    ReqwestError { summary: String },
    PageRedirect,
}

impl From<reqwest::Error> for OwnApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError {
            summary: value.to_string(),
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

    pub async fn make_request<T, E>(
        &self,
        _: E,
        payload: <E as Endpoint>::JsonPayload,
        query_params: <E as Endpoint>::QueryParams,
    ) -> Result<T, OwnApiError>
    where
        T: DeserializeOwned,
        E: Endpoint<JsonResponse = MaybePageRedirect<T>>,
    {
        let mut url = self.domain.clone();

        url.set_path(E::PATH.to_string().as_str());

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in query_params.get_pairs() {
                pairs.append_pair(key, value);
            }
        }

        let request = match E::METHOD {
            Method::Get => self.client.get(url.clone()),
            Method::Post => self.client.post(url.clone()),
        };

        let response = self
            .client
            .execute(request.json(&payload).build().unwrap())
            .await?;

        if response.status() == CUSTOM_REDIRECT_STATUS_CODE.with(|i| *i) {
            let location: Url = self
                .domain
                .join(
                    response
                        .headers()
                        .get(LOCATION)
                        .expect("Redirect had no location header")
                        .to_str()
                        .expect("Redirect header not valid str"),
                )
                .expect("redirect header not valid URL");
            (self.redirect_handler)(location);
            return Err(OwnApiError::PageRedirect);
        }

        let json_response = response.json::<MaybePageRedirect<T>>().await?;

        match json_response {
            MaybePageRedirect::PageRedirectTo(url) => {
                (self.redirect_handler)(url);
                Err(OwnApiError::PageRedirect)
            }
            MaybePageRedirect::NoRedirect(resp) => Ok(resp),
        }
    }
}
