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
pub struct PagesHttpsCertificate {
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "description")]
    pub description: String,
    /// Array of the domain set and its alternate name (if it is configured)
    #[serde(rename = "domains")]
    pub domains: Vec<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl PagesHttpsCertificate {
    pub fn new(state: State, description: String, domains: Vec<String>) -> PagesHttpsCertificate {
        PagesHttpsCertificate {
            state,
            description,
            domains,
            expires_at: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "authorization_created")]
    AuthorizationCreated,
    #[serde(rename = "authorization_pending")]
    AuthorizationPending,
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "authorization_revoked")]
    AuthorizationRevoked,
    #[serde(rename = "issued")]
    Issued,
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "errored")]
    Errored,
    #[serde(rename = "bad_authz")]
    BadAuthz,
    #[serde(rename = "destroy_pending")]
    DestroyPending,
    #[serde(rename = "dns_changed")]
    DnsChanged,
}

impl Default for State {
    fn default() -> State {
        Self::New
    }
}
