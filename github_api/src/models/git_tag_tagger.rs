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
pub struct GitTagTagger {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl GitTagTagger {
    pub fn new(date: String, email: String, name: String) -> GitTagTagger {
        GitTagTagger { date, email, name }
    }
}
