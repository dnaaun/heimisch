use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InstallationId(u64);

#[derive(Serialize, Deserialize)]
struct RepositoryId(u64);

#[derive(Serialize, Deserialize)]
struct UserId(u64);

#[derive(macros::TypesafeIdb, Serialize, Deserialize)]
struct Repository {
    #[idb(id)]
    id: RepositoryId,

    name: String,

    #[idb(index)]
    installation_id: InstallationId,
}

#[derive(macros::TypesafeIdb, Serialize, Deserialize)]
struct User {
    #[idb(id)]
    id: UserId,

    name: String,

    #[idb(index)]
    repository_id: RepositoryId,
}

pub fn main() {
    async {
        let db = typesafe_idb::TypesafeDb::builder("just test".into())
            .with_store::<Repository>()
            .with_store::<User>()
            .build()
            .await
            .unwrap();

        let txn = db.txn().with_store::<User>().ro();

        // Should raise type error
        txn.object_store::<Repository>().unwrap();
    };
}
