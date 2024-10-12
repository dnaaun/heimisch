use leptos::prelude::*;
use shared::types::{
    issue::{Issue, RepositoryIdIndex},
    repository::Repository,
};

use crate::{
    app::sync_engine_provider::use_sync_engine,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

#[component]
pub fn IssuesTab(repository: Repository) -> impl IntoView {
    let sync_engine = use_sync_engine();

    let open_and_close_issues_count = sync_engine.idb_signal(
        |db| db.txn().with_store::<Issue>().ro(),
        move |txn| {
            Box::pin(async move {
                let (open, closed): (Vec<_>, Vec<_>) = txn
                    .object_store::<Issue>()
                    .unwrap()
                    .index::<RepositoryIdIndex>()
                    .unwrap()
                    .get_all(Some(&repository.id))
                    .await
                    .unwrap()
                    .into_iter()
                    .filter_map(|i| i.closed_at.to_option())
                    .partition(|i| i.is_none());
                (open.len(), closed.len())
            })
        },
    );

    let closed = move || open_and_close_issues_count.read().map(|i| i.1);
    let open = move || open_and_close_issues_count.read().map(|i| i.0);

    view! {
        <div class="bg-gray-100 border rounded-t-md p-3 flex flex-nowrap justify-between">
            <div class="flex flex-nowrap gap-x-2">
                <div>Open {open}</div>
                <div>Closed {closed}</div>
            </div>
            <div>Author</div>
        </div>
    }
}
