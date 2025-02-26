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
pub struct RepositoryRuleDetailedOneOf10 {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRuleCommitMessagePatternParameters>>,
    /// The type of source for the ruleset that includes this rule.
    #[serde(
        rename = "ruleset_source_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub ruleset_source_type: Option<RulesetSourceType>,
    /// The name of the source of the ruleset that includes this rule.
    #[serde(rename = "ruleset_source", skip_serializing_if = "Option::is_none")]
    pub ruleset_source: Option<String>,
    /// The ID of the ruleset that includes this rule.
    #[serde(rename = "ruleset_id", skip_serializing_if = "Option::is_none")]
    pub ruleset_id: Option<i32>,
}

impl RepositoryRuleDetailedOneOf10 {
    pub fn new(r#type: Type) -> RepositoryRuleDetailedOneOf10 {
        RepositoryRuleDetailedOneOf10 {
            r#type,
            parameters: None,
            ruleset_source_type: None,
            ruleset_source: None,
            ruleset_id: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "commit_message_pattern")]
    CommitMessagePattern,
}

impl Default for Type {
    fn default() -> Type {
        Self::CommitMessagePattern
    }
}
/// The type of source for the ruleset that includes this rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RulesetSourceType {
    #[serde(rename = "Repository")]
    Repository,
    #[serde(rename = "Organization")]
    Organization,
}

impl Default for RulesetSourceType {
    fn default() -> RulesetSourceType {
        Self::Repository
    }
}
