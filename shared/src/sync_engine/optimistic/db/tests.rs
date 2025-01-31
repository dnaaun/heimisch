use std::collections::HashSet;

use typesafe_idb::Store;
use url::Url;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{
    endpoints::endpoint_client::EndpointClient,
    sync_engine::{websocket_updates::tests::MockTypedTransport, SyncEngine},
    types::repository::{InstallationIdIndex, Repository},
};

async fn get_sync_engine() -> SyncEngine<MockTypedTransport> {
    SyncEngine::new(EndpointClient::new(
        |_| (),
        Url::parse("https://www.example.com/").unwrap(),
    ))
    .await
    .unwrap()
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn get_by_index_reactivity() {
    let txn = get_sync_engine()
        .await
        .db
        .txn()
        .with_store::<Repository>()
        .build();
    let _ = txn
        .object_store::<Repository>()
        .unwrap()
        .index::<InstallationIdIndex>()
        .unwrap()
        .get(&4.into())
        .await;
    let trackers = txn.commit().unwrap();
    assert_eq!(
        trackers.stores_read_in_bulk,
        HashSet::from_iter([Repository::NAME])
    );
    assert!(trackers.stores_read_by_id.is_empty())
}
