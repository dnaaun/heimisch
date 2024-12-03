use crate::endpoints::{endpoint::No, endpoint_client::MaybePageRedirect};

use super::super::super::super::endpoint::{Endpoint, Method};

pub struct AuthInitiateEndpoint;

/// Not really used right now except for the PATH
impl Endpoint for AuthInitiateEndpoint {
    type QueryParams = ();

    const METHOD: Method = Method::Get;

    const PATH: &'static str = "/api/auth/initiate";

    type JsonPayload = ();
    type JsonResponse = MaybePageRedirect<()>;

    type AuthRequired = No;
}
