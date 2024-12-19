use derive_more::derive::{AsRef, Deref, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize, Copy, Hash, PartialEq, Eq,
)]
pub struct LabelId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct Label {
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
    #[idb(id)]
    pub id: LabelId,
    pub name: String,
    pub node_id: String,
    pub url: String,
}
