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

/// RepositoryInvitation : Repository invitations let you manage who you collaborate with.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryInvitation {
    /// Unique identifier of the repository invitation.
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "repository")]
    pub repository: Box<models::MinimalRepository>,
    #[serde(rename = "invitee", deserialize_with = "Option::deserialize")]
    pub invitee: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "inviter", deserialize_with = "Option::deserialize")]
    pub inviter: Option<Box<models::NullableSimpleUser>>,
    /// The permission associated with the invitation.
    #[serde(rename = "permissions")]
    pub permissions: Permissions,
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Whether or not the invitation has expired
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// URL for the repository invitation
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
}

impl RepositoryInvitation {
    /// Repository invitations let you manage who you collaborate with.
    pub fn new(
        id: i64,
        repository: models::MinimalRepository,
        invitee: Option<models::NullableSimpleUser>,
        inviter: Option<models::NullableSimpleUser>,
        permissions: Permissions,
        created_at: String,
        url: String,
        html_url: String,
        node_id: String,
    ) -> RepositoryInvitation {
        RepositoryInvitation {
            id,
            repository: Box::new(repository),
            invitee: if let Some(x) = invitee {
                Some(Box::new(x))
            } else {
                None
            },
            inviter: if let Some(x) = inviter {
                Some(Box::new(x))
            } else {
                None
            },
            permissions,
            created_at,
            expired: None,
            url,
            html_url,
            node_id,
        }
    }
}
/// The permission associated with the invitation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "maintain")]
    Maintain,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Self::Read
    }
}
