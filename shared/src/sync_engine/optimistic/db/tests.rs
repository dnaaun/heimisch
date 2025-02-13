use std::{cell::RefCell, rc::Rc, sync::Arc};

use bon::builder;
use macros::tracing_to_console_log;
use typesafe_idb::{ReadOnly, ReadWrite};
use url::Url;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{
    endpoints::endpoint_client::EndpointClient,
    sync_engine::{tests::MockTransport, DbStoreMarkers, DbSubscription, SyncEngine},
    types::{
        issue::{Issue, RepositoryIdIndex},
        repository::Repository,
    },
};

use super::{TxnBuilderWithOptimisticChanges, TxnWithOptimisticChanges};

async fn get_sync_engine() -> SyncEngine<MockTransport, ()> {
    SyncEngine::<MockTransport, ()>::new(
        EndpointClient::new(|_| (), Url::parse("https://www.example.com/").unwrap()),
        |_| async { Ok(MockTransport::new().0) },
        (),
    )
    .await
    .unwrap()
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[builder]
async fn num_times_subscriber_called<Txn1Markers, Txn1Mode, Txn2Markers, Txn2Mode>(
    make_txn_1: impl for<'a> Fn(
        TxnBuilderWithOptimisticChanges<'a, DbStoreMarkers, (), ReadOnly>,
    ) -> TxnWithOptimisticChanges<Txn1Markers, Txn1Mode>,
    make_txn_2: impl for<'a> Fn(
        TxnBuilderWithOptimisticChanges<'a, DbStoreMarkers, (), ReadWrite>,
    ) -> TxnWithOptimisticChanges<Txn2Markers, Txn2Mode>,
    with_txn_1: impl AsyncFn(&TxnWithOptimisticChanges<Txn1Markers, Txn1Mode>),
    with_txn_2: impl AsyncFn(&TxnWithOptimisticChanges<Txn2Markers, Txn2Mode>),
    should_overlap: bool,
) {
    let subscriber_hit_times = Rc::new(RefCell::new(0));
    let subscriber_hit_times2 = subscriber_hit_times.clone();
    let sync_engine = get_sync_engine().await;
    let txn1 = make_txn_1(sync_engine.db.txn());
    with_txn_1(&txn1).await;

    let original_reactivity_trackers = txn1.commit().unwrap();

    let _ = sync_engine.db_subscriptions.add(DbSubscription {
        original_reactivity_trackers,
        func: Arc::new(move || {
            *subscriber_hit_times2.borrow_mut() += 1;
        }),
    });

    assert!(*subscriber_hit_times.borrow() == 0);

    let txn2 = make_txn_2(sync_engine.db.txn().read_write());
    with_txn_2(&txn2).await;
    txn2.commit().unwrap();

    let value = *subscriber_hit_times.borrow();

    if should_overlap {
        assert!(value >= 1);
    } else {
        assert_eq!(value, 0);
    }
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn index_get_no_optimisim_put_overlapping() {
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .index::<RepositoryIdIndex>()
                .unwrap()
                .get(&4.into())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Issue>()
                .unwrap()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(true)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn index_get_no_optimisim_put_non_overlapping() {
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .index::<RepositoryIdIndex>()
                .unwrap()
                .get(&4.into())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Repository>()
                .unwrap()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn get_no_optimisim_put_overlapping() {
    let some_issue_id = 4.into();
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .get(&some_issue_id)
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Issue>()
                .unwrap()
                .put(&Issue {
                    id: some_issue_id,
                    ..Default::default()
                })
                .await
                .unwrap();
        })
        .should_overlap(true)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn get_no_optimisim_put_non_overlapping() {
    let some_issue_id = 4.into();
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .get(&some_issue_id)
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Issue>()
                .unwrap()
                .put(&Issue {
                    id: (*some_issue_id + 1).into(),
                    ..Default::default()
                })
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;

    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .get(&some_issue_id)
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Repository>()
                .unwrap()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn get_all_no_optimisim_put_overlapping() {
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn.object_store::<Issue>().unwrap().get_all().await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Issue>()
                .unwrap()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(true)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn get_all_no_optimisim_put_non_overlapping() {
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .get(&Default::default())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Repository>()
                .unwrap()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn get_all_no_optimisim_create_overlapping() {
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn.object_store::<Issue>().unwrap().get_all().await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Issue>()
                .unwrap()
                .create(Default::default(), async { Ok(Default::default()) });
        })
        .should_overlap(true)
        .call()
        .await;
}

#[wasm_bindgen_test]
#[tracing_to_console_log]
pub async fn get_all_no_optimisim_create_non_overlapping() {
    num_times_subscriber_called()
        .make_txn_1(|txn| txn.with_store::<Issue>().build())
        .make_txn_2(|txn| txn.with_store::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .object_store::<Issue>()
                .unwrap()
                .get(&Default::default())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.object_store::<Repository>()
                .unwrap()
                .create(Default::default(), async { Ok(Default::default()) });
        })
        .should_overlap(false)
        .call()
        .await;
}
