use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::installation::InstallationId;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InstallationAccessToken {
    pub token: String,

    pub expires_at: Timestamp,
}

/// When storing this in the db, we need to store the installation id.
#[derive(macros::TypesafeIdb, Debug, Serialize, Deserialize, Clone)]
pub struct InstallationAccessTokenRow {
    #[idb(id)]
    pub installation_id: InstallationId,

    #[serde(flatten)]
    pub token: InstallationAccessToken,
}

impl typed_db::Table for InstallationAccessTokenRow {
    const NAME: &'static str = "installation_access_token_rows";
    type Marker = ();
    type Id = InstallationId;

    fn id(&self) -> &Self::Id {
        &self.installation_id
    }

    fn index_names() -> &'static [&'static str] {
        &["installation_id"]
    }
}
