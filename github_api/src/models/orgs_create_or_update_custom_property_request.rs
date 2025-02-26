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
pub struct OrgsCreateOrUpdateCustomPropertyRequest {
    /// The type of the value for the property
    #[serde(rename = "value_type")]
    pub value_type: ValueType,
    /// Whether the property is required.
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(
        rename = "default_value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value:
        Option<Option<Box<models::OrgsCreateOrUpdateCustomPropertyRequestDefaultValue>>>,
    /// Short description of the property
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// An ordered list of the allowed values of the property. The property can have up to 200 allowed values.
    #[serde(
        rename = "allowed_values",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_values: Option<Option<Vec<String>>>,
}

impl OrgsCreateOrUpdateCustomPropertyRequest {
    pub fn new(value_type: ValueType) -> OrgsCreateOrUpdateCustomPropertyRequest {
        OrgsCreateOrUpdateCustomPropertyRequest {
            value_type,
            required: None,
            default_value: None,
            description: None,
            allowed_values: None,
        }
    }
}
/// The type of the value for the property
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "single_select")]
    SingleSelect,
    #[serde(rename = "multi_select")]
    MultiSelect,
    #[serde(rename = "true_false")]
    TrueFalse,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::String
    }
}
