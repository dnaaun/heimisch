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
pub struct SelectedActions {
    /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
    #[serde(
        rename = "github_owned_allowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub github_owned_allowed: Option<bool>,
    /// Whether actions from GitHub Marketplace verified creators are allowed. Set to `true` to allow all actions by GitHub Marketplace verified creators.
    #[serde(rename = "verified_allowed", skip_serializing_if = "Option::is_none")]
    pub verified_allowed: Option<bool>,
    /// Specifies a list of string-matching patterns to allow specific action(s) and reusable workflow(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/_*`.  > [!NOTE] > The `patterns_allowed` setting only applies to public repositories.
    #[serde(rename = "patterns_allowed", skip_serializing_if = "Option::is_none")]
    pub patterns_allowed: Option<Vec<String>>,
}

impl SelectedActions {
    pub fn new() -> SelectedActions {
        SelectedActions {
            github_owned_allowed: None,
            verified_allowed: None,
            patterns_allowed: None,
        }
    }
}
