use derive_more::derive::{AsRef, Deref, From, Into};
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

#[derive(From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LicenseId(String);

#[derive(macros::TypesafeIdb)]
#[derive(Deserialize, Serialize, Clone, Debug, AvailMerge)]
pub struct License {
    pub body: Avail<String>,
    pub conditions: Avail<Vec<String>>,
    pub description: Avail<String>,
    pub featured: Avail<bool>,
    pub html_url: Avail<String>,
    pub implementation: Avail<String>,

    #[idb(id)]
    pub key: LicenseId,

    pub limitations: Avail<Vec<String>>,
    pub name: String,
    pub node_id: String,
    pub permissions: Avail<Vec<String>>,
    pub spdx_id: Avail<String>,
    pub url: Avail<Option<String>>,
}
