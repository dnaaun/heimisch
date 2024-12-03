use crate::endpoints::{endpoint::No, endpoint_client::MaybePageRedirect};

use super::super::super::super::endpoint::{Endpoint, Method};
use derive_more::derive::{AsRef, Deref, Into};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthFinishPayload {
    pub state: String,
    pub code: String,
}

#[cfg_attr(feature = "ssr", derive(diesel_derive_newtype::DieselNewType))]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone, Into, AsRef, Deref)]
pub struct GithubAccessToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthFinishResponse {
    Failure { message: String },
    Success(GithubAccessToken),
}

pub struct AuthFinishEndpoint;

impl Endpoint for AuthFinishEndpoint {
    type QueryParams = ();

    const METHOD: Method = Method::Post;

    const PATH: &'static str = "/api/auth/finish";

    type JsonPayload = AuthFinishPayload;

    type JsonResponse = MaybePageRedirect<AuthFinishResponse>;

    type AuthRequired = No;
}
