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
pub struct ActionsListRepoWorkflows200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "workflows")]
    pub workflows: Vec<models::Workflow>,
}

impl ActionsListRepoWorkflows200Response {
    pub fn new(
        total_count: i32,
        workflows: Vec<models::Workflow>,
    ) -> ActionsListRepoWorkflows200Response {
        ActionsListRepoWorkflows200Response {
            total_count,
            workflows,
        }
    }
}
