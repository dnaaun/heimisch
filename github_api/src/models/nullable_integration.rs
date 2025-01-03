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
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::app_10::Events;

/// NullableIntegration : GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NullableIntegration {
    /// Unique identifier of the GitHub app
    #[serde(rename = "id")]
    pub id: i32,
    /// The slug name of the GitHub app
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "owner", deserialize_with = "Option::deserialize")]
    pub owner: Option<Box<models::NullableSimpleUser>>,
    /// The name of the GitHub app
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "external_url")]
    pub external_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub created_at: Timestamp,
    #[serde(rename = "updated_at")]
    pub updated_at: Timestamp,
    #[serde(rename = "permissions")]
    pub permissions: models::IntegrationPermissions,

    /// The list of events for the GitHub app
    /// NOTE: I changed the type here.
    pub events: Vec<Events>,

    /// The number of installations associated with the GitHub app
    #[serde(
        rename = "installations_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub installations_count: Option<i32>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(
        rename = "webhook_secret",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_secret: Option<Option<String>>,
    #[serde(rename = "pem", skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
}
