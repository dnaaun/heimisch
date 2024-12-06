use serde::{Deserialize, Serialize};

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

        let txn = db.txn().with_store::<Repository>().ro();
        let object_store = txn.object_store::<Repository>().unwrap();

        // Should raise type error
        object_store.delete(&RepositoryId(4)).await.unwrap();
    };
}
