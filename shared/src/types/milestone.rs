use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::milestone;
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

use super::user::UserId;

#[derive(
    From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize, Copy, Hash, PartialEq, Eq,
)]
pub struct MilestoneId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, AvailMerge)]
pub struct Milestone {
    pub closed_at: Avail<Option<Timestamp>>,
    pub closed_issues: Avail<i64>,
    pub created_at: Avail<Timestamp>,
    pub creator_id: Avail<Option<UserId>>,
    pub description: Avail<Option<String>>,
    pub due_on: Avail<Option<Timestamp>>,
    pub html_url: Avail<String>,
    #[idb(id)]
    pub id: MilestoneId,
    pub labels_url: Avail<String>,
    pub node_id: Avail<String>,
    #[doc = "The number of the milestone."]
    pub number: Avail<i64>,
    pub open_issues: Avail<i64>,
    #[doc = "The state of the milestone."]
    pub state: Avail<milestone::State>,
    #[doc = "The title of the milestone."]
    pub title: Avail<String>,
    pub updated_at: Avail<Timestamp>,
    pub url: Avail<String>,
}
