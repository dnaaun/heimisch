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

/// CommitActivity : Commit Activity
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitActivity {
    #[serde(rename = "days")]
    pub days: Vec<i32>,
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "week")]
    pub week: i32,
}

impl CommitActivity {
    /// Commit Activity
    pub fn new(days: Vec<i32>, total: i32, week: i32) -> CommitActivity {
        CommitActivity { days, total, week }
    }
}
