use serde::{Deserialize, Serialize};

use crate::{
    endpoints::endpoint::GetEndpoint,
    types::{
        installation::{Installation, InstallationId},
        installation_access_token_row::InstallationAccessToken,
    },
};

#[derive(Deserialize, Serialize)]
pub struct GetInstallationAccessTokenQueryParams {
    pub installation_id: InstallationId,
}

pub struct GetInstallationAccessTokenEndpoint;

impl GetEndpoint for GetInstallationAccessTokenEndpoint {
    type QueryParams = GetInstallationAccessTokenQueryParams;
    const PATH: &'static str = "/api/installations/get_token";
    type JsonResponse = InstallationAccessToken;
}

#[derive(Deserialize, Serialize)]
pub struct GetInstallationsResponse {
    pub installations: Vec<Installation>,
}

pub struct GetInstallationsEndpoint;

impl GetEndpoint for GetInstallationsEndpoint {
    type QueryParams = ();
    const PATH: &'static str = "/api/installations/";
    type JsonResponse = GetInstallationsResponse;
}
