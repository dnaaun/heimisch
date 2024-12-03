use derive_more::derive::{AsRef, Deref, From, Into};
use serde::{Deserialize, Serialize};

#[derive(From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestLabelsItemId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug)]
pub struct PullRequestLabelsItem {
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
    #[idb(id)]
    pub id: PullRequestLabelsItemId,
    pub name: String,
    pub node_id: String,
    pub url: String,
}
