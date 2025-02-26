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

/// CopilotSeatDetailsAssigningTeam : The team through which the assignee is granted access to GitHub Copilot, if applicable.
/// The team through which the assignee is granted access to GitHub Copilot, if applicable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopilotSeatDetailsAssigningTeam {
    Team(Box<models::Team>),
    EnterpriseTeam(Box<models::EnterpriseTeam>),
}

impl Default for CopilotSeatDetailsAssigningTeam {
    fn default() -> Self {
        Self::Team(Default::default())
    }
}
