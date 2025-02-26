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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookTeamDeleted {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::Repository15>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUser>>,
    #[serde(rename = "team")]
    pub team: Box<models::WebhooksTeam1>,
}

impl WebhookTeamDeleted {
    pub fn new(
        action: Action,
        organization: models::OrganizationSimpleWebhooks,
        team: models::WebhooksTeam1,
    ) -> WebhookTeamDeleted {
        WebhookTeamDeleted {
            action,
            enterprise: None,
            installation: None,
            organization: Box::new(organization),
            repository: None,
            sender: None,
            team: Box::new(team),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for Action {
    fn default() -> Action {
        Self::Deleted
    }
}
