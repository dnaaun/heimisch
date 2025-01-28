#![feature(type_alias_impl_trait)]

use std::collections::{HashMap, HashSet};

use macros::TypesafeIdb;
use serde::{Deserialize, Serialize};
use typesafe_idb::{SerializedId, Store, StoreMarker, TypesafeDb};
use wasm_bindgen_test::wasm_bindgen_test;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InstallationId(u64);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct RepositoryId(u64);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct UserId(u64);

#[derive(TypesafeIdb, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Repository {
    #[idb(id)]
    id: RepositoryId,

    name: String,

    #[idb(index)]
    installation_id: InstallationId,
}

#[derive(TypesafeIdb, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct User {
    #[idb(id)]
    id: UserId,

    name: String,

    #[idb(index)]
    repository_id: RepositoryId,
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

type DbMarkers = impl StoreMarker<Repository> + StoreMarker<User>;

async fn get_db() -> TypesafeDb<DbMarkers> {
    TypesafeDb::builder("just test".into())
        .with_store::<User>()
        .with_store::<User>()
        .with_store::<Repository>()
        .build()
        .await
        .unwrap()
}

#[wasm_bindgen_test]
pub async fn get_by_index_reactivity() {
    let txn = get_db().await.txn().with_store::<Repository>().build();
    let _ = txn
        .object_store::<Repository>()
        .unwrap()
        .index::<InstallationIdIndex>()
        .unwrap()
        .get(&InstallationId(4))
        .await;
    let trackers = txn.commit().await.unwrap();
    assert_eq!(
        trackers.stores_accessed_in_bulk,
        HashSet::from_iter([Repository::NAME])
    );
    assert!(trackers.stores_accessed_by_id.is_empty())
}

#[wasm_bindgen_test]
pub async fn get_all_by_index_reactivity() {
    let txn = get_db().await.txn().with_store::<Repository>().build();
    let _ = txn
        .object_store::<Repository>()
        .unwrap()
        .index::<InstallationIdIndex>()
        .unwrap()
        .get_all(Some(&InstallationId(4)))
        .await;
    let trackers = txn.commit().await.unwrap();

    assert_eq!(
        trackers.stores_accessed_in_bulk,
        HashSet::from_iter([Repository::NAME])
    );
    assert!(trackers.stores_accessed_by_id.is_empty())
}

#[wasm_bindgen_test]
pub async fn get_by_id_reactivity() {
    let txn = get_db().await.txn().with_store::<Repository>().build();
    let _ = txn
        .object_store::<Repository>()
        .unwrap()
        .get(&RepositoryId(4))
        .await;
    let trackers = txn.commit().await.unwrap();

    assert!(trackers.stores_accessed_in_bulk.is_empty());

    assert_eq!(
        trackers.stores_accessed_by_id,
        HashMap::from_iter([(
            Repository::NAME,
            HashSet::from_iter([SerializedId::new_from_id::<Repository>(&RepositoryId(4))])
        )])
    );
}

#[wasm_bindgen_test]
pub async fn get_all_reactivity() {
    let txn = get_db().await.txn().with_store::<Repository>().build();
    let _ = txn.object_store::<Repository>().unwrap().get_all().await;
    let trackers = txn.commit().await.unwrap();

    assert_eq!(
        trackers.stores_accessed_in_bulk,
        HashSet::from_iter([Repository::NAME])
    );
    assert!(trackers.stores_accessed_by_id.is_empty())
}

#[wasm_bindgen_test]
pub async fn basic_read_and_write() {
    let txn = get_db()
        .await
        .txn()
        .with_store::<Repository>()
        // doubling causess no issues.
        .with_store::<User>()
        .with_store::<User>()
        .read_write()
        .build();

    let id = RepositoryId(4);
    let repository_object_store = txn.object_store::<Repository>().unwrap();
    let repository_object_store_installation_id_index = repository_object_store
        .index::<InstallationIdIndex>()
        .unwrap();
    assert_eq!(repository_object_store.get(&id).await.unwrap(), None);

    let installation_id = InstallationId(4243);
    assert!(repository_object_store_installation_id_index
        .get_all(Some(&installation_id))
        .await
        .unwrap()
        .is_empty());

    let repo = Repository {
        id: id.clone(),
        name: "Hey!".into(),
        installation_id: installation_id.clone(),
    };

    let repo2 = Repository {
        id: RepositoryId(repo.id.0 + 4),
        installation_id: InstallationId(repo.installation_id.0 + 1),
        name: "Heyyoo".into(),
    };
    repository_object_store.put(&repo).await.unwrap();
    repository_object_store.put(&repo2).await.unwrap();

    assert_eq!(
        repository_object_store.get(&id).await.unwrap(),
        Some(repo.clone())
    );

    assert_eq!(
        repository_object_store_installation_id_index
            .get_all(Some(&installation_id))
            .await
            .unwrap(),
        vec![repo.clone()]
    );

    repository_object_store.delete(&id).await.unwrap();
    assert_eq!(repository_object_store.get(&id).await.unwrap(), None);
}

#[test]
fn expect_compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
