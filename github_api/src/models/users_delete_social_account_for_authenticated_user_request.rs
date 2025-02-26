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
pub struct UsersDeleteSocialAccountForAuthenticatedUserRequest {
    /// Full URLs for the social media profiles to delete.
    #[serde(rename = "account_urls")]
    pub account_urls: Vec<String>,
}

impl UsersDeleteSocialAccountForAuthenticatedUserRequest {
    pub fn new(account_urls: Vec<String>) -> UsersDeleteSocialAccountForAuthenticatedUserRequest {
        UsersDeleteSocialAccountForAuthenticatedUserRequest { account_urls }
    }
}
