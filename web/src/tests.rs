use std::rc::Rc;

use leptos::{
    prelude::*,
    task::{tick, Executor},
};
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
    _ = Executor::init_wasm_bindgen();

    let sync_engine = SyncEngine::new(
        Rc::new(BACKEND_API.with(|e| e.clone())),
        async |url| Transport::new(url).await,
        shared::github_api_trait::GithubApi.into(),
    )
    .await
    .unwrap();

    let num_times_updated = RwSignal::new(0);

    let signal = sync_engine.idb_signal(
        move |b| b.with_table::<IssueCommentsInitialSyncStatus>().build(),
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
        .build();

    txn.table::<IssueCommentsInitialSyncStatus>()
        .put(&Default::default())
        .await
        .unwrap();
    txn.commit().unwrap();

    tick().await;

    let _ = signal.await;
    assert_eq!(num_times_updated.get(), 2);
}
