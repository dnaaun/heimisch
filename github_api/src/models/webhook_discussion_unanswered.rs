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
pub struct WebhookDiscussionUnanswered {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "discussion")]
    pub discussion: Box<models::Discussion>,
    #[serde(rename = "old_answer")]
    pub old_answer: Box<models::WebhooksAnswer>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUser>>,
}

impl WebhookDiscussionUnanswered {
    pub fn new(
        action: Action,
        discussion: models::Discussion,
        old_answer: models::WebhooksAnswer,
        repository: models::RepositoryWebhooks,
    ) -> WebhookDiscussionUnanswered {
        WebhookDiscussionUnanswered {
            action,
            discussion: Box::new(discussion),
            old_answer: Box::new(old_answer),
            organization: None,
            repository: Box::new(repository),
            sender: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "unanswered")]
    Unanswered,
}

impl Default for Action {
    fn default() -> Action {
        Self::Unanswered
    }
}
