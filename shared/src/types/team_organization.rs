use derive_more::derive::{AsRef, Deref, From, Into};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize)]
pub struct TeamOrganizationId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug)]
pub struct TeamOrganization {
    pub archived_at: Option<Timestamp>,
    pub avatar_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blog: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_repository_permission: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub events_url: String,
    pub followers: i64,
    pub following: i64,
    pub has_organization_projects: bool,
    pub has_repository_projects: bool,
    pub hooks_url: String,
    pub html_url: String,
    #[idb(id)]
    pub id: TeamOrganizationId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    pub issues_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_fork_private_repositories: Option<bool>,
    pub members_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TeamOrganizationPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<i64>,
    pub public_gists: i64,
    pub public_members_url: String,
    pub public_repos: i64,
    pub repos_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_factor_requirement_enabled: Option<bool>,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: Timestamp,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TeamOrganizationPlan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filled_seats: Option<i64>,
    pub name: String,
    pub private_repos: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seats: Option<i64>,
    pub space: i64,
}
