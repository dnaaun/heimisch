use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::installation::InstallationId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallationAccessToken {
    pub token: String,

    pub expires_at: Timestamp,
}

/// When storing this in the db, we need to store the installation id.
#[derive(macros::TypesafeIdb)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallationAccessTokenRow {
    #[idb(id)]
    pub installation_id: InstallationId,

    #[serde(flatten)]
    pub token: InstallationAccessToken,
}
