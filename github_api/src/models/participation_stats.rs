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
pub struct ParticipationStats {
    #[serde(rename = "all")]
    pub all: Vec<i32>,
    #[serde(rename = "owner")]
    pub owner: Vec<i32>,
}

impl ParticipationStats {
    pub fn new(all: Vec<i32>, owner: Vec<i32>) -> ParticipationStats {
        ParticipationStats { all, owner }
    }
}
