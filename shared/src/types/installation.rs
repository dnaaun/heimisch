use derive_more::derive::{AsRef, Deref, Display, From, Into};
use serde::{Deserialize, Serialize};

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
