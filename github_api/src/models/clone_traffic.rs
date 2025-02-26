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

/// CloneTraffic : Clone Traffic
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneTraffic {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "uniques")]
    pub uniques: i32,
    #[serde(rename = "clones")]
    pub clones: Vec<models::Traffic>,
}

impl CloneTraffic {
    /// Clone Traffic
    pub fn new(count: i32, uniques: i32, clones: Vec<models::Traffic>) -> CloneTraffic {
        CloneTraffic {
            count,
            uniques,
            clones,
        }
    }
}
