use derive_more::derive::{AsRef, Deref, From, Into};
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

#[derive(From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LicenseId(String);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, AvailMerge)]
pub struct License {
    pub body: Avail<String>,
    pub conditions: Avail<Vec<String>>,
    pub description: Avail<String>,
    pub featured: Avail<bool>,
    pub html_url: Avail<String>,
    pub implementation: Avail<String>,

    #[idb(id)]
    pub key: LicenseId,

    pub limitations: Avail<Vec<String>>,
    pub name: String,
    pub node_id: String,
    pub permissions: Avail<Vec<String>>,
    pub spdx_id: Avail<String>,
    pub url: Avail<Option<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct InstallationId(u64);

#[derive(Serialize, Deserialize)]
pub struct RepositoryId(u64);

#[derive(Serialize, Deserialize)]
pub struct UserId(u64);

#[derive(macros::TypesafeIdb, Serialize, Deserialize)]
pub struct Repository {
    #[idb(id)]
    id: RepositoryId,

    name: String,

    #[idb(index)]
    installation_id: InstallationId,
}

#[derive(macros::TypesafeIdb, Serialize, Deserialize)]
pub struct User {
    #[idb(id)]
    id: UserId,

    name: String,

    #[idb(index)]
    repository_id: RepositoryId,
}

fn main() {
    #[allow(unused_must_use)]
    async {
        let db = typesafe_idb::TypesafeDb::builder("just test".into())
            .with_store::<Repository>()
            .build()
            .await
            .unwrap();

        let txn = db.txn().with_store::<Repository>().rw();
        let object_store = txn.object_store::<Repository>().unwrap();

        // Should raise type error
        object_store.delete(&RepositoryId(4)).await.unwrap();
    };
}
