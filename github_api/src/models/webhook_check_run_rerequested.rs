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
pub struct WebhookCheckRunRerequested {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "check_run")]
    pub check_run: Box<models::CheckRunWithSimpleCheckSuite>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
}

impl WebhookCheckRunRerequested {
    pub fn new(
        check_run: models::CheckRunWithSimpleCheckSuite,
        repository: models::RepositoryWebhooks,
        sender: models::SimpleUser,
    ) -> WebhookCheckRunRerequested {
        WebhookCheckRunRerequested {
            action: None,
            check_run: Box::new(check_run),
            installation: None,
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "rerequested")]
    Rerequested,
}

impl Default for Action {
    fn default() -> Action {
        Self::Rerequested
    }
}
