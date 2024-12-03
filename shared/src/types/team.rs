use derive_more::derive::{AsRef, Deref, From, Into};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::team_organization::TeamOrganizationId;

#[derive(From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize)]
pub struct TeamId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    /// This is optional only because the `TeamSimple` returned
    /// sometimes doesn't contain it.
    pub created_at: Option<Timestamp>,
    pub description: Option<String>,
    pub html_url: String,
    #[doc = "Unique identifier of the team"]
    #[idb(id)]
    pub id: TeamId,
    #[doc = "Distinguished Name (DN) that team maps to within LDAP environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap_dn: Option<String>,
    /// This is optional only because the `TeamSimple` returned
    /// sometimes doesn't contain it.
    pub members_count: Option<i64>,
    pub members_url: String,
    #[doc = "Name of the team"]
    pub name: String,
    pub node_id: String,
    #[doc = "The notification setting the team has set"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<FullTeamNotificationSetting>,
    /// This is optional only because the `TeamSimple` returned
    /// sometimes doesn't contain it.
    pub organization_id: Option<TeamOrganizationId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<TeamId>,
    #[doc = "Permission that the team will have for its repositories"]
    pub permission: String,
    #[doc = "The level of privacy this team should have"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<FullTeamPrivacy>,
    /// This is optional only because the `TeamSimple` returned
    /// sometimes doesn't contain it.
    pub repos_count: Option<i64>,
    pub repositories_url: String,
    pub slug: String,
    /// This is optional only because the `TeamSimple` returned
    /// sometimes doesn't contain it.
    pub updated_at: Option<Timestamp>,
    #[doc = "URL for the team"]
    pub url: String,

    pub _done_a_full_fetch: bool,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FullTeamNotificationSetting {
    #[serde(rename = "notifications_enabled")]
    NotificationsEnabled,
    #[serde(rename = "notifications_disabled")]
    NotificationsDisabled,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FullTeamPrivacy {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
}
