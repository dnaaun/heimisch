use std::time::SystemTime;

use derive_more::derive::{AsRef, Deref, Display, From, Into};
use serde::{Deserialize, Serialize};

use super::user::UserId;

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
    Eq,
    Hash,
    Copy,
    Display,
    Default,
)]
pub struct InstallationId(i64);

#[derive(Serialize, Deserialize)]
pub struct Installation {
    pub id: InstallationId,
    pub created_at: SystemTime,
    pub github_user_id: UserId,
}
