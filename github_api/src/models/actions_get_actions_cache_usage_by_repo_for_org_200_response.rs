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
pub struct ActionsGetActionsCacheUsageByRepoForOrg200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "repository_cache_usages")]
    pub repository_cache_usages: Vec<models::ActionsCacheUsageByRepository>,
}

impl ActionsGetActionsCacheUsageByRepoForOrg200Response {
    pub fn new(
        total_count: i32,
        repository_cache_usages: Vec<models::ActionsCacheUsageByRepository>,
    ) -> ActionsGetActionsCacheUsageByRepoForOrg200Response {
        ActionsGetActionsCacheUsageByRepoForOrg200Response {
            total_count,
            repository_cache_usages,
        }
    }
}
