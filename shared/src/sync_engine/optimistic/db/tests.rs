use std::sync::Arc;

use any_spawner::Executor;
use bon::builder;
use parking_lot::Mutex;
use typed_db::{sqlite_impl::SqliteDatabase, RawDbTrait, ReadOnly, ReadWrite};
use url::Url;

use crate::{
    backend_api_trait::BackendApi,
    endpoints::endpoint_client::EndpointClient,
    sync_engine::{
        websocket_updates::transport::tests::MockTransport, DbSubscription, DbTableMarkers,
        SyncEngine,
    },
    types::{
        issue::{Issue, RepositoryIdIndex},
        repository::Repository,
    },
};

use super::{TxnBuilderWithOptimisticChanges, TxnWithOptimisticChanges};

async fn get_sync_engine<RawDb: RawDbTrait>() -> SyncEngine<RawDb, BackendApi, MockTransport, ()> {
    SyncEngine::<RawDb, BackendApi, MockTransport, ()>::new(
        Arc::new(BackendApi::new(EndpointClient::new(
            |_| (),
            Url::parse("https://www.example.com/").unwrap(),
        ))),
        Arc::new(()),
        ":memory:".into(),
    )
    .await
    .unwrap()
}

#[builder]
async fn num_times_subscriber_called<
    RawDb: RawDbTrait,
    Txn1Markers,
    Txn1Mode,
    Txn2Markers,
    Txn2Mode,
>(
    make_txn_1: impl for<'a> Fn(
        TxnBuilderWithOptimisticChanges<'a, RawDb, DbTableMarkers, (), ReadOnly>,
    ) -> TxnWithOptimisticChanges<RawDb, Txn1Markers, Txn1Mode>,
    make_txn_2: impl for<'a> Fn(
        TxnBuilderWithOptimisticChanges<'a, RawDb, DbTableMarkers, (), ReadWrite>,
    ) -> TxnWithOptimisticChanges<RawDb, Txn2Markers, Txn2Mode>,
    with_txn_1: impl AsyncFn(&TxnWithOptimisticChanges<RawDb, Txn1Markers, Txn1Mode>),
    with_txn_2: impl AsyncFn(&TxnWithOptimisticChanges<RawDb, Txn2Markers, Txn2Mode>),
    should_overlap: bool,
) {
    let subscriber_hit_times = Arc::new(Mutex::new(0));
    let subscriber_hit_times2 = subscriber_hit_times.clone();
    let sync_engine = get_sync_engine().await;
    let txn1 = make_txn_1(sync_engine.db.txn());
    with_txn_1(&txn1).await;

    let original_reactivity_trackers = txn1.commit().unwrap();

    let _ = sync_engine.db_subscriptions.add(DbSubscription {
        original_reactivity_trackers,
        func: Arc::new(move || {
            *subscriber_hit_times2.lock() += 1;
        }),
    });

    assert!(*subscriber_hit_times.lock() == 0);

    let txn2 = make_txn_2(sync_engine.db.txn().read_write());
    with_txn_2(&txn2).await;
    let _ = txn2.commit().unwrap();

    let value = *subscriber_hit_times.lock();

    if should_overlap {
        assert!(value >= 1);
    } else {
        assert_eq!(value, 0);
    }
}

pub async fn index_get_no_optimisim_put_overlapping() {
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .index::<RepositoryIdIndex>()
                .get_optimistically(&4.into())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Issue>().put(&Default::default()).await.unwrap();
        })
        .should_overlap(true)
        .call()
        .await;
}

#[tokio::test]
pub async fn index_get_no_optimisim_put_non_overlapping() {
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .index::<RepositoryIdIndex>()
                .get_optimistically(&4.into())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Repository>()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;
}

#[tokio::test]
pub async fn get_no_optimisim_put_overlapping() {
    let some_issue_id = 4.into();
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .get_optimistically(&some_issue_id)
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Issue>()
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

#[tokio::test]
pub async fn get_no_optimisim_put_non_overlapping() {
    let some_issue_id = 4.into();
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .get_optimistically(&some_issue_id)
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Issue>()
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

    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .get_optimistically(&some_issue_id)
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Repository>()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;
}

pub fn init_executor() {
    #[cfg(feature = "ssr")]
    let _ = Executor::init_futures_executor(); // ignore AlreadySet error

    #[cfg(feature = "hydrate")]
    let _ = Executor::init_wasm_bindgen(); // ignore AlreadySet error
}

#[tokio::test]
pub async fn get_all_no_optimisim_put_overlapping() {
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn.table::<Issue>().get_all_optimistically().await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Issue>().put(&Default::default()).await.unwrap();
        })
        .should_overlap(true)
        .call()
        .await;
}

#[tokio::test]
pub async fn get_all_no_optimisim_put_non_overlapping() {
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .get_optimistically(&Default::default())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Repository>()
                .put(&Default::default())
                .await
                .unwrap();
        })
        .should_overlap(false)
        .call()
        .await;
}

#[tokio::test]
pub async fn get_all_no_optimisim_create_overlapping() {
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Issue>().build())
        .with_txn_1(async |txn| {
            let _ = txn.table::<Issue>().get_all_optimistically().await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Issue>()
                .create_optimistically(Default::default(), async { Ok(Default::default()) });
        })
        .should_overlap(true)
        .call()
        .await;
}

#[tokio::test]
pub async fn get_all_no_optimisim_create_non_overlapping() {
    init_executor();
    num_times_subscriber_called::<SqliteDatabase, _, _, _, _, _, _, _, _>()
        .make_txn_1(|txn| txn.with_table::<Issue>().build())
        .make_txn_2(|txn| txn.with_table::<Repository>().build())
        .with_txn_1(async |txn| {
            let _ = txn
                .table::<Issue>()
                .get_optimistically(&Default::default())
                .await;
        })
        .with_txn_2(async |txn| {
            txn.table::<Repository>()
                .create_optimistically(Default::default(), async { Ok(Default::default()) });
        })
        .should_overlap(false)
        .call()
        .await;
}
