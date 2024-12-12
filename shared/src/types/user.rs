use derive_more::derive::{AsRef, Deref, Display, From, Into};
use github_api::models::user::Type;
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

#[cfg_attr(feature = "ssr", derive(diesel_derive_newtype::DieselNewType))]
#[derive(
    From,
    Into,
    Deref,
    AsRef,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Copy,
    Hash,
    Display,
    Default,
)]
pub struct UserId(i64);

#[derive(macros::TypesafeIdb, Serialize, Deserialize, Clone, Debug, AvailMerge)]
pub struct User {
    pub avatar_url: Avail<String>,
    pub bio: Avail<Option<String>>,
    pub blog: Avail<Option<String>>,
    pub business_plus: Avail<Option<bool>>,
    pub collaborators: Avail<i64>,
    pub company: Avail<Option<String>>,
    pub created_at: Avail<Option<Timestamp>>,
    pub disk_usage: Avail<i64>,
    pub deleted: Avail<bool>,
    pub email: Avail<Option<String>>,
    pub events_url: Avail<String>,
    pub followers: Avail<Option<i64>>,
    pub followers_url: Avail<String>,
    pub following: Avail<i64>,
    pub following_url: Avail<String>,
    pub gists_url: Avail<String>,
    pub gravatar_id: Avail<Option<String>>,
    pub hireable: Avail<Option<bool>>,
    pub html_url: Avail<String>,

    #[idb(id)]
    pub id: UserId,
    pub ldap_dn: Avail<Option<String>>,
    pub location: Avail<Option<String>>,

    #[idb(index)]
    pub login: String,

    pub name: Avail<Option<String>>,
    pub node_id: Avail<String>,
    pub notification_email: Avail<Option<String>>,
    pub organizations_url: Avail<String>,
    pub owned_private_repos: Avail<i64>,

    // IGNORED field .
    // pub plan: Option<PrivateUserPlan>,
    pub private_gists: Avail<i64>,
    pub public_gists: Avail<i64>,
    pub public_repos: Avail<i64>,
    pub received_events_url: Avail<String>,
    pub repos_url: Avail<String>,
    pub site_admin: Avail<bool>,
    pub starred_url: Avail<String>,
    pub subscriptions_url: Avail<String>,
    pub total_private_repos: Avail<i64>,
    pub twitter_username: Avail<Option<String>>,
    pub two_factor_authentication: Avail<bool>,
    pub r#type: Avail<Type>,
    pub updated_at: Avail<Timestamp>,
    pub url: Avail<String>,
    pub user_view_type: Avail<Option<String>>,

    pub starred_at: Avail<Option<Timestamp>>,
}
