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
pub struct CopilotDotcomChatModelsInner {
    /// Name of the language used for Copilot code completion suggestions, for the given editor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Indicates whether a model is custom or default.
    #[serde(rename = "is_custom_model", skip_serializing_if = "Option::is_none")]
    pub is_custom_model: Option<bool>,
    /// The training date for the custom model (if applicable).
    #[serde(
        rename = "custom_model_training_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_model_training_date: Option<Option<String>>,
    /// Total number of users who prompted Copilot Chat on github.com at least once for each model.
    #[serde(
        rename = "total_engaged_users",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_engaged_users: Option<i32>,
    /// Total number of chats initiated by users on github.com.
    #[serde(rename = "total_chats", skip_serializing_if = "Option::is_none")]
    pub total_chats: Option<i32>,
}

impl CopilotDotcomChatModelsInner {
    pub fn new() -> CopilotDotcomChatModelsInner {
        CopilotDotcomChatModelsInner {
            name: None,
            is_custom_model: None,
            custom_model_training_date: None,
            total_engaged_users: None,
            total_chats: None,
        }
    }
}
