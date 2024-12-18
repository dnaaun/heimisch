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
pub struct WebhooksSecurityAdvisoryCvss {
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "vector_string", deserialize_with = "Option::deserialize")]
    pub vector_string: Option<String>,
}

impl WebhooksSecurityAdvisoryCvss {
    pub fn new(score: f64, vector_string: Option<String>) -> WebhooksSecurityAdvisoryCvss {
        WebhooksSecurityAdvisoryCvss {
            score,
            vector_string,
        }
    }
}
