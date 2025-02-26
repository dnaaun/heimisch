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
pub struct SigstoreBundle0VerificationMaterialTlogEntriesInnerInclusionPromise {
    #[serde(
        rename = "signedEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_entry_timestamp: Option<String>,
}

impl SigstoreBundle0VerificationMaterialTlogEntriesInnerInclusionPromise {
    pub fn new() -> SigstoreBundle0VerificationMaterialTlogEntriesInnerInclusionPromise {
        SigstoreBundle0VerificationMaterialTlogEntriesInnerInclusionPromise {
            signed_entry_timestamp: None,
        }
    }
}
