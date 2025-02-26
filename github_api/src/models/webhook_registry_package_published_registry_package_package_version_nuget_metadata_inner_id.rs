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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInnerId {
    String(String),
    Object(serde_json::Value),
    Integer(i32),
}

impl Default for WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInnerId {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
