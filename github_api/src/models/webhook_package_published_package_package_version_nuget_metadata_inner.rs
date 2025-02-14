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
pub struct WebhookPackagePublishedPackagePackageVersionNugetMetadataInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<models::ActionsGetWorkflowWorkflowIdParameter>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value:
        Option<Box<models::WebhookPackagePublishedPackagePackageVersionNugetMetadataInnerValue>>,
}

impl WebhookPackagePublishedPackagePackageVersionNugetMetadataInner {
    pub fn new() -> WebhookPackagePublishedPackagePackageVersionNugetMetadataInner {
        WebhookPackagePublishedPackagePackageVersionNugetMetadataInner {
            id: None,
            name: None,
            value: None,
        }
    }
}
