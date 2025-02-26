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
pub struct WebhookWorkflowDispatch {
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "inputs", deserialize_with = "Option::deserialize")]
    pub inputs: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUser>,
    #[serde(rename = "workflow")]
    pub workflow: String,
}

impl WebhookWorkflowDispatch {
    pub fn new(
        inputs: Option<std::collections::HashMap<String, serde_json::Value>>,
        r#ref: String,
        repository: models::RepositoryWebhooks,
        sender: models::SimpleUser,
        workflow: String,
    ) -> WebhookWorkflowDispatch {
        WebhookWorkflowDispatch {
            enterprise: None,
            inputs,
            installation: None,
            organization: None,
            r#ref,
            repository: Box::new(repository),
            sender: Box::new(sender),
            workflow,
        }
    }
}
