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
pub struct WebhookCodeScanningAlertAppearedInBranchAlertRule {
    /// A short description of the rule used to detect the alert.
    #[serde(rename = "description")]
    pub description: String,
    /// A unique identifier for the rule used to detect the alert.
    #[serde(rename = "id")]
    pub id: String,
    /// The severity of the alert.
    #[serde(rename = "severity", deserialize_with = "Option::deserialize")]
    pub severity: Option<Severity>,
}

impl WebhookCodeScanningAlertAppearedInBranchAlertRule {
    pub fn new(
        description: String,
        id: String,
        severity: Option<Severity>,
    ) -> WebhookCodeScanningAlertAppearedInBranchAlertRule {
        WebhookCodeScanningAlertAppearedInBranchAlertRule {
            description,
            id,
            severity,
        }
    }
}
/// The severity of the alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "null")]
    Null,
}

impl Default for Severity {
    fn default() -> Severity {
        Self::None
    }
}
