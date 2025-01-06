use crate::endpoints::endpoint::PostEndpoint;

use derive_more::derive::{AsRef, Deref, Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthFinishPayload {
    pub state: String,
    pub code: String,
}

#[cfg_attr(feature = "ssr", derive(diesel_derive_newtype::DieselNewType))]
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone, Into, From, AsRef, Deref, Display,
)]
pub struct GithubAccessToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthFinishResponse {
    Failure { message: String },
    Success(GithubAccessToken),
}

pub struct AuthFinishEndpoint;

impl PostEndpoint for AuthFinishEndpoint {
    type QueryParams = ();
    const PATH: &'static str = "/api/auth/finish";
    type JsonPayload = AuthFinishPayload;
    type JsonResponse = AuthFinishResponse;
}
