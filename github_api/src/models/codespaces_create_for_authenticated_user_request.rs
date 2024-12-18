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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodespacesCreateForAuthenticatedUserRequest {
    CodespacesCreateForAuthenticatedUserRequestOneOf(
        Box<models::CodespacesCreateForAuthenticatedUserRequestOneOf>,
    ),
    CodespacesCreateForAuthenticatedUserRequestOneOf1(
        Box<models::CodespacesCreateForAuthenticatedUserRequestOneOf1>,
    ),
}

impl Default for CodespacesCreateForAuthenticatedUserRequest {
    fn default() -> Self {
        Self::CodespacesCreateForAuthenticatedUserRequestOneOf(Default::default())
    }
}
/// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is closing down.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Geo {
    #[serde(rename = "EuropeWest")]
    EuropeWest,
    #[serde(rename = "SoutheastAsia")]
    SoutheastAsia,
    #[serde(rename = "UsEast")]
    UsEast,
    #[serde(rename = "UsWest")]
    UsWest,
}

impl Default for Geo {
    fn default() -> Geo {
        Self::EuropeWest
    }
}
