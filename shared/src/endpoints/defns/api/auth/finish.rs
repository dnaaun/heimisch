use super::super::super::super::endpoint::{Endpoint, Method};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthFinishPayload {
    pub state: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthFinishResponse {
    Failure { message: String },
    Success { user_access_token: String },
}

pub struct AuthFinishEndpoint;

impl Endpoint for AuthFinishEndpoint {
    type QueryParams = ();

    const METHOD: Method = Method::Post;

    const PATH: &'static str = "/api/auth/finish";

    type JsonPayload = AuthFinishPayload;

    type JsonResponse = AuthFinishResponse;
}
