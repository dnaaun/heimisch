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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesAddLabelsRequest {
    IssuesAddLabelsRequestOneOf(Box<models::IssuesAddLabelsRequestOneOf>),
    Array(Vec<String>),
    IssuesSetLabelsRequestOneOf1(Box<models::IssuesSetLabelsRequestOneOf1>),
    IssuesSetLabelsRequestOneOf1LabelsInner1(Vec<models::IssuesSetLabelsRequestOneOf1LabelsInner>),
    String(String),
}

impl Default for IssuesAddLabelsRequest {
    fn default() -> Self {
        Self::IssuesAddLabelsRequestOneOf(Default::default())
    }
}
