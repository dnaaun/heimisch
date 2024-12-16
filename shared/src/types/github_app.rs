use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::{app_10::Events, app_1_permissions::ReadOrWrite, App1Permissions};
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

use super::user::UserId;

#[derive(
    From,
    Into,
    Deref,
    AsRef,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
)]
pub struct GithubAppId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, AvailMerge, Default)]
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

#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
/// From `IssuesAssignedIssuePerformedViaGithubAppPermissions` in webhooks.
pub struct GitHubAppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_references: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussions: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<ReadOrWriteOrAdmin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_secrets: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_scanning_alert: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_file: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<ReadOrWrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<ReadOrWrite>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReadOrWriteOrAdmin {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
