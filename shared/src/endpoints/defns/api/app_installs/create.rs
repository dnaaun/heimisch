use serde::{Deserialize, Serialize};

use crate::{endpoints::endpoint_client::MaybePageRedirect, types::installation::InstallationId};

use super::super::super::super::endpoint::{Endpoint, Method};

#[derive(Deserialize, Serialize)]
pub struct CreateAppInstallPayload {
    pub installation_id: InstallationId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreateAppInstallResponse {
    Failure { message: String },
    Success { installation_id: InstallationId },
}

pub struct CreateAppInstallEndpoint;

impl Endpoint for CreateAppInstallEndpoint {
    type QueryParams = ();

    const METHOD: Method = Method::Post;

    const PATH: &'static str = "/api/app_installs/";

    type JsonPayload = CreateAppInstallPayload;

    type JsonResponse = MaybePageRedirect<CreateAppInstallResponse>;
}
