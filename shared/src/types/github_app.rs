use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::{app_10::Events, App1Permissions};
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

use super::user::UserId;

#[derive(
    From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Hash,
)]
pub struct GithubAppId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, AvailMerge)]
pub struct GithubApp {
    pub client_id: Avail<Option<String>>,
    pub client_secret: Avail<Option<String>>,
    pub created_at: Avail<Timestamp>,
    pub description: Avail<Option<String>>,
    #[doc = "The list of events for the GitHub app"]
    pub events: Avail<Vec<Events>>,
    pub external_url: Avail<String>,
    pub html_url: Avail<String>,

    #[doc = "Unique identifier of the GitHub app"]
    #[idb(id)]
    pub id: GithubAppId,

    #[doc = "The number of installations associated with the GitHub app"]
    pub installations_count: Avail<i64>,
    #[doc = "The name of the GitHub app"]
    pub name: Avail<String>,
    pub node_id: Avail<String>,
    pub owner_id: Avail<UserId>,
    pub pem: Avail<Option<String>>,
    pub permissions: Avail<App1Permissions>,
    #[doc = "The slug name of the GitHub app"]
    pub slug: Avail<Option<String>>,
    pub updated_at: Avail<Timestamp>,
    pub webhook_secret: Avail<Option<String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GitHubAppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<String, String>,
}
