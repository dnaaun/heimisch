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

/// CodeOfConductSimple : Code of Conduct Simple
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeOfConductSimple {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "html_url", deserialize_with = "Option::deserialize")]
    pub html_url: Option<String>,
}

impl CodeOfConductSimple {
    /// Code of Conduct Simple
    pub fn new(
        url: String,
        key: String,
        name: String,
        html_url: Option<String>,
    ) -> CodeOfConductSimple {
        CodeOfConductSimple {
            url,
            key,
            name,
            html_url,
        }
    }
}
