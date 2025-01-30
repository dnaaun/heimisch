use std::sync::Arc;

use leptos::{
    prelude::*,
    task::{tick, Executor},
};
use parking_lot::Mutex;
use shared::types::issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus;
use tracing::Level;
use tracing_subscriber::fmt::MakeWriter;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

use crate::{
    app::sync_engine_provider::SyncEngine, consts::ENDPOINT_CLIENT,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

struct InMemoryWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl std::io::Write for InMemoryWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock();
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// NOTE: I should clean this up into a tracing logger for testing.
struct MemoryWriterFactory {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl<'a> MakeWriter<'a> for MemoryWriterFactory {
    type Writer = InMemoryWriter;

    fn make_writer(&'a self) -> Self::Writer {
        InMemoryWriter {
            buffer: Arc::clone(&self.buffer),
        }
    }
}

/// Note the scaffolding to get tracing output to show up in log output? Maybe abstract that out
/// into it's own crate?
#[wasm_bindgen_test]
#[allow(dead_code)]
pub async fn idb_signal_basic_reactivity() {
    // Create shared buffer
    let buffer = Arc::new(Mutex::new(Vec::new()));

    // Create a writer factory for tracing
    let writer_factory = MemoryWriterFactory {
        buffer: Arc::clone(&buffer),
    };

    // Set the subscriber with the custom writer factory
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_writer(writer_factory)
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    _ = Executor::init_wasm_bindgen();

    let sync_engine = SyncEngine::new(ENDPOINT_CLIENT.with(|e| e.clone()))
        .await
        .unwrap();

    let num_times_updated = RwSignal::new(0);

    let signal = sync_engine.idb_signal(
        move |b| b.with_store::<IssueCommentsInitialSyncStatus>().build(),
        move |txn| async move {
            txn.object_store::<IssueCommentsInitialSyncStatus>()?
                .get_all()
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
        .with_store::<IssueCommentsInitialSyncStatus>()
        .read_write()
        .build();

    txn.object_store::<IssueCommentsInitialSyncStatus>()
        .unwrap()
        .no_optimism_put(&Default::default())
        .await
        .unwrap();
    txn.commit().unwrap();

    tick().await;

    let _ = signal.await;
    assert_eq!(num_times_updated.get(), 2);

    let logged_data = buffer.lock();
    let logged_data_str = String::from_utf8(logged_data.clone()).unwrap();
    console_log!("{}", logged_data_str);
}
