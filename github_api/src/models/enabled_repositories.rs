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

/// EnabledRepositories : The policy that controls the repositories in the organization that are allowed to run GitHub Actions.
/// The policy that controls the repositories in the organization that are allowed to run GitHub Actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnabledRepositories {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "selected")]
    Selected,
}

impl std::fmt::Display for EnabledRepositories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::Selected => write!(f, "selected"),
        }
    }
}

impl Default for EnabledRepositories {
    fn default() -> EnabledRepositories {
        Self::All
    }
}
