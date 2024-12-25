use serde::{Deserialize, Serialize};

use crate::{
    endpoints::endpoint_client::MaybePageRedirect,
    types::{installation::{Installation, InstallationId}, installation_access_token_row::InstallationAccessToken},
};

use super::super::super::endpoint::{Endpoint, Method};

#[derive(Deserialize, Serialize)]
pub struct GetInstallationAccessTokenPayload {
    pub installation_id: InstallationId,
}

pub struct GetInstallationAccessTokenEndpoint;

impl Endpoint for GetInstallationAccessTokenEndpoint {
    type QueryParams = ();

    const METHOD: Method = Method::Post;

    const PATH: &'static str = "/api/installations/get_token";

    type JsonPayload = GetInstallationAccessTokenPayload;

    type JsonResponse = MaybePageRedirect<InstallationAccessToken>;
}

#[derive(Deserialize, Serialize)]
pub struct GetInstallationsResponse {
    pub installations: Vec<Installation>
}

pub struct GetInstallationsEndpoint;

impl Endpoint for GetInstallationsEndpoint {
    type QueryParams = ();

    const METHOD: Method = Method::Get;

    const PATH: &'static str = "/api/installations/";

    type JsonPayload = ();

    type JsonResponse = MaybePageRedirect<GetInstallationsResponse>;
}
