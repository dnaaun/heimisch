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
pub struct CodeSecurityDefaultConfigurationsInner {
    /// The visibility of newly created repositories for which the code security configuration will be applied to by default
    #[serde(
        rename = "default_for_new_repos",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_for_new_repos: Option<DefaultForNewRepos>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<models::CodeSecurityConfiguration>>,
}

impl CodeSecurityDefaultConfigurationsInner {
    pub fn new() -> CodeSecurityDefaultConfigurationsInner {
        CodeSecurityDefaultConfigurationsInner {
            default_for_new_repos: None,
            configuration: None,
        }
    }
}
/// The visibility of newly created repositories for which the code security configuration will be applied to by default
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultForNewRepos {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private_and_internal")]
    PrivateAndInternal,
    #[serde(rename = "all")]
    All,
}

impl Default for DefaultForNewRepos {
    fn default() -> DefaultForNewRepos {
        Self::Public
    }
}
