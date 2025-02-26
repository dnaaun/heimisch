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
pub struct WebhookOrganizationRenamed {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Box<models::WebhookOrganizationRenamedChanges>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "membership", skip_serializing_if = "Option::is_none")]
    pub membership: Option<Box<models::WebhooksMembership>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookOrganizationRenamed {
    pub fn new(
        action: Action,
        organization: models::OrganizationSimpleWebhooks,
        sender: models::SimpleUser,
    ) -> WebhookOrganizationRenamed {
        WebhookOrganizationRenamed {
            action,
            changes: None,
            enterprise: None,
            installation: None,
            membership: None,
            organization: Box::new(organization),
            repository: None,
            sender: Box::new(sender),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "renamed")]
    Renamed,
}

impl Default for Action {
    fn default() -> Action {
        Self::Renamed
    }
}
