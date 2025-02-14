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

/// DependabotAlertWithRepositoryDependency : Details for the vulnerable dependency.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DependabotAlertWithRepositoryDependency {
    #[serde(rename = "package", skip_serializing_if = "Option::is_none")]
    pub package: Option<Box<models::DependabotAlertPackage>>,
    /// The full path to the dependency manifest file, relative to the root of the repository.
    #[serde(rename = "manifest_path", skip_serializing_if = "Option::is_none")]
    pub manifest_path: Option<String>,
    /// The execution scope of the vulnerable dependency.
    #[serde(
        rename = "scope",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope: Option<Option<Scope>>,
}

impl DependabotAlertWithRepositoryDependency {
    /// Details for the vulnerable dependency.
    pub fn new() -> DependabotAlertWithRepositoryDependency {
        DependabotAlertWithRepositoryDependency {
            package: None,
            manifest_path: None,
            scope: None,
        }
    }
}
/// The execution scope of the vulnerable dependency.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "runtime")]
    Runtime,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Development
    }
}
