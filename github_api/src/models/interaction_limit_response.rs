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

/// InteractionLimitResponse : Interaction limit settings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractionLimitResponse {
    #[serde(rename = "limit")]
    pub limit: models::InteractionGroup,
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "expires_at")]
    pub expires_at: String,
}

impl InteractionLimitResponse {
    /// Interaction limit settings.
    pub fn new(
        limit: models::InteractionGroup,
        origin: String,
        expires_at: String,
    ) -> InteractionLimitResponse {
        InteractionLimitResponse {
            limit,
            origin,
            expires_at,
        }
    }
}
