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
pub struct WebhookPackageUpdatedPackagePackageVersion {
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::User2>>,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "body_html")]
    pub body_html: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "docker_metadata", skip_serializing_if = "Option::is_none")]
    pub docker_metadata:
        Option<Vec<models::WebhookPackagePublishedPackagePackageVersionDockerMetadataInner>>,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "installation_command")]
    pub installation_command: String,
    #[serde(rename = "manifest", skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Vec<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "package_files")]
    pub package_files: Vec<models::WebhookPackageUpdatedPackagePackageVersionPackageFilesInner>,
    #[serde(rename = "package_url", skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[serde(rename = "prerelease", skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(rename = "release", skip_serializing_if = "Option::is_none")]
    pub release: Option<Box<models::WebhookPackageUpdatedPackagePackageVersionRelease>>,
    #[serde(rename = "rubygems_metadata", skip_serializing_if = "Option::is_none")]
    pub rubygems_metadata: Option<Vec<models::WebhookRubygemsMetadata>>,
    #[serde(rename = "source_url", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    #[serde(rename = "target_oid")]
    pub target_oid: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl WebhookPackageUpdatedPackagePackageVersion {
    pub fn new(
        author: Option<models::User2>,
        body: String,
        body_html: String,
        created_at: String,
        description: String,
        html_url: String,
        id: i32,
        installation_command: String,
        metadata: Vec<std::collections::HashMap<String, serde_json::Value>>,
        name: String,
        package_files: Vec<models::WebhookPackageUpdatedPackagePackageVersionPackageFilesInner>,
        summary: String,
        target_commitish: String,
        target_oid: String,
        updated_at: String,
        version: String,
    ) -> WebhookPackageUpdatedPackagePackageVersion {
        WebhookPackageUpdatedPackagePackageVersion {
            author: if let Some(x) = author {
                Some(Box::new(x))
            } else {
                None
            },
            body,
            body_html,
            created_at,
            description,
            docker_metadata: None,
            draft: None,
            html_url,
            id,
            installation_command,
            manifest: None,
            metadata,
            name,
            package_files,
            package_url: None,
            prerelease: None,
            release: None,
            rubygems_metadata: None,
            source_url: None,
            summary,
            tag_name: None,
            target_commitish,
            target_oid,
            updated_at,
            version,
        }
    }
}
