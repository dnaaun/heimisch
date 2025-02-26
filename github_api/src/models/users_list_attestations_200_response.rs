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
pub struct UsersListAttestations200Response {
    #[serde(rename = "attestations", skip_serializing_if = "Option::is_none")]
    pub attestations: Option<Vec<models::UsersListAttestations200ResponseAttestationsInner>>,
}

impl UsersListAttestations200Response {
    pub fn new() -> UsersListAttestations200Response {
        UsersListAttestations200Response { attestations: None }
    }
}
