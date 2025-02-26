/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WebhooksTeam1 : Groups of organization members that gives permissions on specified repositories.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksTeam1 {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Description of the team
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// Unique identifier of the team
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "members_url", skip_serializing_if = "Option::is_none")]
    pub members_url: Option<String>,
    /// Name of the team
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(
        rename = "parent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<Option<Box<models::WebhooksTeamParent>>>,
    /// Permission that the team will have for its repositories
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    /// Whether team members will receive notifications when their team is @mentioned
    #[serde(
        rename = "notification_setting",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_setting: Option<NotificationSetting>,
    #[serde(rename = "repositories_url", skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// URL for the team
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WebhooksTeam1 {
    /// Groups of organization members that gives permissions on specified repositories.
    pub fn new(id: i32, name: String) -> WebhooksTeam1 {
        WebhooksTeam1 {
            deleted: None,
            description: None,
            html_url: None,
            id,
            members_url: None,
            name,
            node_id: None,
            parent: None,
            permission: None,
            privacy: None,
            notification_setting: None,
            repositories_url: None,
            slug: None,
            url: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
}

impl Default for Privacy {
    fn default() -> Privacy {
        Self::Open
    }
}
/// Whether team members will receive notifications when their team is @mentioned
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationSetting {
    #[serde(rename = "notifications_enabled")]
    Enabled,
    #[serde(rename = "notifications_disabled")]
    Disabled,
}

impl Default for NotificationSetting {
    fn default() -> NotificationSetting {
        Self::Enabled
    }
}
