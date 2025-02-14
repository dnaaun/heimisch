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
pub struct BranchRestrictionPolicyTeamsInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(
        rename = "notification_setting",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_setting: Option<String>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "members_url", skip_serializing_if = "Option::is_none")]
    pub members_url: Option<String>,
    #[serde(rename = "repositories_url", skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(
        rename = "parent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<Option<String>>,
}

impl BranchRestrictionPolicyTeamsInner {
    pub fn new() -> BranchRestrictionPolicyTeamsInner {
        BranchRestrictionPolicyTeamsInner {
            id: None,
            node_id: None,
            url: None,
            html_url: None,
            name: None,
            slug: None,
            description: None,
            privacy: None,
            notification_setting: None,
            permission: None,
            members_url: None,
            repositories_url: None,
            parent: None,
        }
    }
}
