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

/// EnterpriseWebhooks : An enterprise on GitHub. Webhook payloads contain the `enterprise` property when the webhook is configured on an enterprise account or an organization that's part of an enterprise account. For more information, see \"[About enterprise accounts](https://docs.github.com/admin/overview/about-enterprise-accounts).\"
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseWebhooks {
    /// A short description of the enterprise.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// The enterprise's website URL.
    #[serde(
        rename = "website_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub website_url: Option<Option<String>>,
    /// Unique identifier of the enterprise
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The name of the enterprise.
    #[serde(rename = "name")]
    pub name: String,
    /// The slug url identifier for the enterprise.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
}

impl EnterpriseWebhooks {
    /// An enterprise on GitHub. Webhook payloads contain the `enterprise` property when the webhook is configured on an enterprise account or an organization that's part of an enterprise account. For more information, see \"[About enterprise accounts](https://docs.github.com/admin/overview/about-enterprise-accounts).\"
    pub fn new(
        html_url: String,
        id: i32,
        node_id: String,
        name: String,
        slug: String,
        created_at: Option<String>,
        updated_at: Option<String>,
        avatar_url: String,
    ) -> EnterpriseWebhooks {
        EnterpriseWebhooks {
            description: None,
            html_url,
            website_url: None,
            id,
            node_id,
            name,
            slug,
            created_at,
            updated_at,
            avatar_url,
        }
    }
}
