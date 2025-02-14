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

/// GitCreateCommitRequestCommitter : Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitCreateCommitRequestCommitter {
    /// The name of the author (or committer) of the commit
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email of the author (or committer) of the commit
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl GitCreateCommitRequestCommitter {
    /// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
    pub fn new() -> GitCreateCommitRequestCommitter {
        GitCreateCommitRequestCommitter {
            name: None,
            email: None,
            date: None,
        }
    }
}
