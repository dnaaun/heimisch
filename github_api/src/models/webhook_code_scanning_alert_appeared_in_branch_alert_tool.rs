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
pub struct WebhookCodeScanningAlertAppearedInBranchAlertTool {
    /// The name of the tool used to generate the code scanning analysis alert.
    #[serde(rename = "name")]
    pub name: String,
    /// The version of the tool used to detect the alert.
    #[serde(rename = "version", deserialize_with = "Option::deserialize")]
    pub version: Option<String>,
}

impl WebhookCodeScanningAlertAppearedInBranchAlertTool {
    pub fn new(
        name: String,
        version: Option<String>,
    ) -> WebhookCodeScanningAlertAppearedInBranchAlertTool {
        WebhookCodeScanningAlertAppearedInBranchAlertTool { name, version }
    }
}
