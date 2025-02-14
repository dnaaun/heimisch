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

/// ProjectsV2SingleSelectOption : An option for a single select field
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectsV2SingleSelectOption {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "color",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<Option<String>>,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
}

impl ProjectsV2SingleSelectOption {
    /// An option for a single select field
    pub fn new(id: String, name: String) -> ProjectsV2SingleSelectOption {
        ProjectsV2SingleSelectOption {
            id,
            name,
            color: None,
            description: None,
        }
    }
}
