use std::{cell::RefCell, rc::Rc, sync::Arc};

use macros::tracing_to_console_log;
use typesafe_idb::Store;
use url::Url;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

use crate::{
    endpoints::endpoint_client::EndpointClient,
    sync_engine::{
        optimistic::db::{ReactivityTrackers, SerializedId},
        websocket_updates::tests::MockTypedTransport,
        DbSubscription, SyncEngine,
    },
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
#[tracing_to_console_log]
pub async fn get_by_index_reactivity() {
    let sync_engine = get_sync_engine().await;
    let mut original_reactivity_trackers = ReactivityTrackers::default();
    original_reactivity_trackers.add_modification(
        Repository::NAME,
        SerializedId::new_from_id::<Repository>(&4.into()),
    );
    let subscriber_hit_times = Rc::new(RefCell::new(0));
    let subscriber_hit_times2 = subscriber_hit_times.clone();

    let _ = sync_engine.db_subscriptions.add(DbSubscription {
        original_reactivity_trackers,
        func: Arc::new(move || {
            console_log!("HEYOO");
            *subscriber_hit_times2.borrow_mut() += 1;
        }),
    });
    console_log!("HEYOO outside");
    let txn = sync_engine.db.txn().with_store::<Repository>().build();
    let _ = txn
        .object_store::<Repository>()
        .unwrap()
        .index::<InstallationIdIndex>()
        .unwrap()
        .get(&4.into())
        .await;
    let _ = txn.commit();

    assert!(*subscriber_hit_times.borrow() == 1)
}
