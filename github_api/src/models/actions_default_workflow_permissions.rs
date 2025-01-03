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

/// ActionsDefaultWorkflowPermissions : The default workflow permissions granted to the GITHUB_TOKEN when running workflows.
/// The default workflow permissions granted to the GITHUB_TOKEN when running workflows.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionsDefaultWorkflowPermissions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

impl std::fmt::Display for ActionsDefaultWorkflowPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
        }
    }
}

impl Default for ActionsDefaultWorkflowPermissions {
    fn default() -> ActionsDefaultWorkflowPermissions {
        Self::Read
    }
}
