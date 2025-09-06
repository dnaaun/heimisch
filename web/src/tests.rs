use std::sync::Arc;

use leptos::{prelude::*, task::tick};
use macros::leptos_test_setup;
use shared::{
    sync_engine::Transport,
    types::issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
};

use crate::{
    app::sync_engine_provider::SyncEngine, consts::BACKEND_API,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[leptos_test_setup]
pub async fn idb_signal_basic_reactivity() {
    let sync_engine = SyncEngine::builder()
        .backend_api(Arc::new(BACKEND_API.with(|e| e.clone())))
        .github_api(shared::github_api_trait::GithubApi.into())
        .db_name("heimisch".into())
        .make_transport(Arc::new(move |url| {
            Box::pin(async move { Transport::new(url).await })
        }))
        .build()
        .await
        .unwrap();

    let num_times_updated = RwSignal::new(0);

    let signal = sync_engine.idb_signal(
        async move |b| {
            b.with_table::<IssueCommentsInitialSyncStatus>()
                .build()
                .await
        },
        move |txn| async move {
            txn.table::<IssueCommentsInitialSyncStatus>()
                .get_all_optimistically()
                .await?;
            *num_times_updated.write() += 1;
            Ok(())
        },
    );
    signal.await.unwrap();

    assert_eq!(num_times_updated.get(), 1);

    // Clarify that another .await doesn't actually cause a recomputation.
    signal.await.unwrap();
    assert_eq!(num_times_updated.get(), 1);

    // But adding a new row should cause a recomputation
    let txn = sync_engine
        .db
        .txn()
        .with_table::<IssueCommentsInitialSyncStatus>()
        .read_write()
        .build()
        .await;

    txn.table::<IssueCommentsInitialSyncStatus>()
        .put(&Default::default())
        .await
        .unwrap();
    txn.commit().await.unwrap();

    tick().await;

    let _ = signal.await;
    assert_eq!(num_times_updated.get(), 2);
}
