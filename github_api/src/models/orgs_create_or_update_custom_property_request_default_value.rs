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

/// OrgsCreateOrUpdateCustomPropertyRequestDefaultValue : Default value of the property
/// Default value of the property
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgsCreateOrUpdateCustomPropertyRequestDefaultValue {
    String(String),
    Array(Vec<String>),
}

impl Default for OrgsCreateOrUpdateCustomPropertyRequestDefaultValue {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
