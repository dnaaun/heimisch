use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
struct InstallationId(u64);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
struct RepositoryId(u64);

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Hash, PartialOrd, Ord, Clone)]
struct UserId(u64);

#[derive(Debug, macros::TypesafeIdb, Serialize, Deserialize, Clone)]
struct Repository {
    #[idb(id)]
    id: RepositoryId,

    name: String,

    #[idb(index)]
    installation_id: InstallationId,
}

#[derive(macros::TypesafeIdb, Serialize, Deserialize, Clone, Debug)]
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

        let txn = db.txn().with_store::<User>().read_only();

        // Should raise type error
        txn.build().object_store::<Repository>().unwrap();
    };
}
