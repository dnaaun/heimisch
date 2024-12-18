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
pub struct CvssSeverities {
    #[serde(
        rename = "cvss_v3",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cvss_v3: Option<Option<Box<models::CvssSeveritiesCvssV3>>>,
    #[serde(
        rename = "cvss_v4",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cvss_v4: Option<Option<Box<models::CvssSeveritiesCvssV4>>>,
}

impl CvssSeverities {
    pub fn new() -> CvssSeverities {
        CvssSeverities {
            cvss_v3: None,
            cvss_v4: None,
        }
    }
}
