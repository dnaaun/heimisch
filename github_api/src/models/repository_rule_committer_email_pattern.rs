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

/// RepositoryRuleCommitterEmailPattern : Parameters to be used for the committer_email_pattern rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleCommitterEmailPattern {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRuleCommitMessagePatternParameters>>,
}

impl RepositoryRuleCommitterEmailPattern {
    /// Parameters to be used for the committer_email_pattern rule
    pub fn new(r#type: Type) -> RepositoryRuleCommitterEmailPattern {
        RepositoryRuleCommitterEmailPattern {
            r#type,
            parameters: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "committer_email_pattern")]
    CommitterEmailPattern,
}

impl Default for Type {
    fn default() -> Type {
        Self::CommitterEmailPattern
    }
}
