use std::ops::Deref;

use thaw::*;
use jiff::{fmt::strtime, Timestamp};
use leptos::prelude::*;
use shared::types::{
    issue::{Issue, RepositoryIdIndex}, issue_comment::{IssueComment, IssueIdIndex}, repository::Repository, user::User
};

use crate::{
    app::sync_engine_provider::use_sync_engine,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

#[component]
pub fn IssuesTab(repository: Repository) -> impl IntoView {
    let sync_engine = use_sync_engine();

    let issues = sync_engine.idb_signal(
        |db| db.txn().with_store::<Issue>().ro(),
        move |txn| async move {
            txn.object_store::<Issue>()
                .unwrap()
                .index::<RepositoryIdIndex>()
                .unwrap()
                .get_all(Some(&repository.id))
                .await
                .unwrap()
        },
    );
    let issues = move || issues.read().iter().flatten().cloned().collect::<Vec<_>>();

    let counts = move || {
        let issues = issues();
        let (open, closed): (Vec<Option<Timestamp>>, Vec<_>) = issues
            .iter()
            .filter_map(|i| i.closed_at.as_ref().to_option())
            .partition(|i| i.is_none());
        Some((open.len(), closed.len()))
    };

    move || {
        view! {
            <>
            <div class="bg-gray-100 border rounded-t-md p-3 flex flex-nowrap justify-between">
                <div class="flex flex-nowrap gap-x-2">
                    <div>Open {counts().map(|c| c.0)}</div>
                    <div>Closed {counts().map(|c| c.1)}</div>
                </div>
                <div>Author</div>
            </div>
            <For
                each={move || issues().into_iter().enumerate()}
                key={move |(_, i)|i.id}
                children={move |(i, issue)| view! { <IssueRow issue=issue is_last={i == issues().len() - 1} /> }
                }
            />

            </>
        }
    }
}

#[component]
pub fn IssueRow(issue: Issue, #[prop(optional)] is_last: bool) -> impl IntoView {
    let sync_engine = use_sync_engine();
    let user_id = issue.user_id.clone();
    let user = sync_engine.idb_signal(
        |db| db.txn().with_store::<User>().ro(),
        move |txn| {
            let user_id = user_id.clone();
            async move {
                let user_id = match user_id.to_option().flatten() {
                    Some(u) => u,
                    None => return None,
                };
                txn.object_store::<User>()
                    .unwrap()
                    .get(&user_id)
                    .await
                    .unwrap()
            }
        },
    );
    let issue_id = issue.id.clone();
    let comments_count = sync_engine.idb_signal(
        |db| db.txn().with_store::<IssueComment>().ro(),
        move |txn| {
            let issue_id = issue_id.clone();
            async move {
            txn.object_store::<IssueComment>()
                .unwrap()
                .index::<IssueIdIndex>()
                .unwrap()
                .get_all(Some(&Some(issue_id.clone())))
                .await
                .unwrap()
                .len()
        }});
    let comments_count = move || comments_count.read().deref().deref().clone();



    move || {
        let created_at = issue.created_at.clone();
        let closed_at = issue.closed_at.clone();
        let title = issue.title.clone();
        let number = issue.number.clone();

        let user = user.read();
        let user = user.as_ref();
        let user = user.flatten();
        let login = user.map(|u| u.login.clone());

        let opened_or_closed_text = closed_at
            .as_ref()
            .to_option()
            .flatten()
            .map(|i| format!("closed on {}", strtime::format("%b %d, %Y", *i).expect("")))
            .or_else(move || {
                created_at
                    .to_option()
                    .map(|i| format!("opened on {}", strtime::format("%b %d, %Y", i).expect("")))
            });


        view! {
            <div class="border-r border-l border-b p-3 flex justify-between items-center"
                class=("rounded-b", is_last)
                >
                <div>
                    <a class="mb-1.5 font-bold">{title.to_option()}</a>
                    <div class="flex gap-1.5 text-sm text-gray-500">
                        <div>{format!("#{number}")}</div>
                        <div>"·"</div>
                        <div>{login}</div>
                        <div>{opened_or_closed_text}</div>
                    </div>
                </div>
                <div>
                <div class="flex gap-1 items-center">
                    <Icon icon={icondata::AiCommentOutlined} />
                    {comments_count}
                </div>
                </div>
            </div>
        }
    }
}
