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

/// LicenseContent : License Content
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseContent {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url", deserialize_with = "Option::deserialize")]
    pub html_url: Option<String>,
    #[serde(rename = "git_url", deserialize_with = "Option::deserialize")]
    pub git_url: Option<String>,
    #[serde(rename = "download_url", deserialize_with = "Option::deserialize")]
    pub download_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "encoding")]
    pub encoding: String,
    #[serde(rename = "_links")]
    pub _links: Box<models::ContentTreeEntriesInnerLinks>,
    #[serde(rename = "license", deserialize_with = "Option::deserialize")]
    pub license: Option<Box<models::NullableLicenseSimple>>,
}

impl LicenseContent {
    /// License Content
    pub fn new(
        name: String,
        path: String,
        sha: String,
        size: i32,
        url: String,
        html_url: Option<String>,
        git_url: Option<String>,
        download_url: Option<String>,
        r#type: String,
        content: String,
        encoding: String,
        _links: models::ContentTreeEntriesInnerLinks,
        license: Option<models::NullableLicenseSimple>,
    ) -> LicenseContent {
        LicenseContent {
            name,
            path,
            sha,
            size,
            url,
            html_url,
            git_url,
            download_url,
            r#type,
            content,
            encoding,
            _links: Box::new(_links),
            license: if let Some(x) = license {
                Some(Box::new(x))
            } else {
                None
            },
        }
    }
}
