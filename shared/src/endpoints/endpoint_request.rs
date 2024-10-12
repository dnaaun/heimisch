use super::endpoint::{Endpoint, Method, QueryParams};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

/// I had to have an error type here to make the `make_request` below work with
/// `leptos::Resource::new()`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OwnApiError {
    ReqwestError { summary: String },
}

impl From<reqwest::Error> for OwnApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError {
            summary: value.to_string(),
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait EndpointRequest
where
    Self: Endpoint,
{
    async fn make_request(
        domain: &Url,
        client: &Client,
        payload: <Self as Endpoint>::JsonPayload,
        query_params: <Self as Endpoint>::QueryParams,
    ) -> Result<Self::JsonResponse, OwnApiError> {
        let mut url = domain.clone();

        url.set_path(Self::PATH.to_string().as_str());

        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in query_params.get_pairs() {
                pairs.append_pair(key, value);
            }
        }

        let request = match Self::METHOD {
            Method::Get => client.get(url),
            Method::Post => client.post(url),
        };

        let response = client
            .execute(request.json(&payload).build().unwrap())
            .await?;

        let json_response = response.json::<Self::JsonResponse>().await?;

        Ok(json_response)
    }
}

impl<E: Endpoint> EndpointRequest for E {}
