use serde::{Deserialize, Serialize};

use crate::{endpoints::endpoint::PostEndpoint, types::installation::InstallationId};

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

impl PostEndpoint for CreateAppInstallEndpoint {
    type QueryParams = ();

    const PATH: &'static str = "/api/app_installs/";

    type JsonPayload = CreateAppInstallPayload;

    type JsonResponse = CreateAppInstallResponse;
}
