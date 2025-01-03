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
pub struct WebhookDiscussionTransferredChanges {
    #[serde(rename = "new_discussion")]
    pub new_discussion: Box<models::Discussion>,
    #[serde(rename = "new_repository")]
    pub new_repository: Box<models::RepositoryWebhooks>,
}

impl WebhookDiscussionTransferredChanges {
    pub fn new(
        new_discussion: models::Discussion,
        new_repository: models::RepositoryWebhooks,
    ) -> WebhookDiscussionTransferredChanges {
        WebhookDiscussionTransferredChanges {
            new_discussion: Box::new(new_discussion),
            new_repository: Box::new(new_repository),
        }
    }
}
