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

/// CodeScanningVariantAnalysisRepository : Repository Identifier
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningVariantAnalysisRepository {
    /// A unique identifier of the repository.
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of the repository.
    #[serde(rename = "name")]
    pub name: String,
    /// The full, globally unique, name of the repository.
    #[serde(rename = "full_name")]
    pub full_name: String,
    /// Whether the repository is private.
    #[serde(rename = "private")]
    pub private: bool,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i32,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
}

impl CodeScanningVariantAnalysisRepository {
    /// Repository Identifier
    pub fn new(
        id: i32,
        name: String,
        full_name: String,
        private: bool,
        stargazers_count: i32,
        updated_at: Option<String>,
    ) -> CodeScanningVariantAnalysisRepository {
        CodeScanningVariantAnalysisRepository {
            id,
            name,
            full_name,
            private,
            stargazers_count,
            updated_at,
        }
    }
}
