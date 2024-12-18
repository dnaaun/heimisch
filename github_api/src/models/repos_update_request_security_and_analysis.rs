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

/// ReposUpdateRequestSecurityAndAnalysis : Specify which security and analysis features to enable or disable for the repository.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  For example, to enable GitHub Advanced Security, use this data in the body of the `PATCH` request: `{ \"security_and_analysis\": {\"advanced_security\": { \"status\": \"enabled\" } } }`.  You can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReposUpdateRequestSecurityAndAnalysis {
    #[serde(rename = "advanced_security", skip_serializing_if = "Option::is_none")]
    pub advanced_security:
        Option<Box<models::ReposUpdateRequestSecurityAndAnalysisAdvancedSecurity>>,
    #[serde(rename = "secret_scanning", skip_serializing_if = "Option::is_none")]
    pub secret_scanning: Option<Box<models::ReposUpdateRequestSecurityAndAnalysisSecretScanning>>,
    #[serde(
        rename = "secret_scanning_push_protection",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_push_protection:
        Option<Box<models::ReposUpdateRequestSecurityAndAnalysisSecretScanningPushProtection>>,
    #[serde(
        rename = "secret_scanning_ai_detection",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_ai_detection:
        Option<Box<models::ReposUpdateRequestSecurityAndAnalysisSecretScanningAiDetection>>,
    #[serde(
        rename = "secret_scanning_non_provider_patterns",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_non_provider_patterns:
        Option<Box<models::ReposUpdateRequestSecurityAndAnalysisSecretScanningNonProviderPatterns>>,
}

impl ReposUpdateRequestSecurityAndAnalysis {
    /// Specify which security and analysis features to enable or disable for the repository.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  For example, to enable GitHub Advanced Security, use this data in the body of the `PATCH` request: `{ \"security_and_analysis\": {\"advanced_security\": { \"status\": \"enabled\" } } }`.  You can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
    pub fn new() -> ReposUpdateRequestSecurityAndAnalysis {
        ReposUpdateRequestSecurityAndAnalysis {
            advanced_security: None,
            secret_scanning: None,
            secret_scanning_push_protection: None,
            secret_scanning_ai_detection: None,
            secret_scanning_non_provider_patterns: None,
        }
    }
}
