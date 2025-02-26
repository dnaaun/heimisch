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
pub struct WebhookRepositoryEditedChangesTopics {
    #[serde(
        rename = "from",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub from: Option<Option<Vec<String>>>,
}

impl WebhookRepositoryEditedChangesTopics {
    pub fn new() -> WebhookRepositoryEditedChangesTopics {
        WebhookRepositoryEditedChangesTopics { from: None }
    }
}
