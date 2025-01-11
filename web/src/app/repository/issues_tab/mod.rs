pub mod list;

use itertools::Itertools;
use jiff::{fmt::strtime, Timestamp};
use leptos::prelude::*;
use shared::types::{
    issue::{Issue, RepositoryIdIndex},
    issue_comment::{IssueComment, IssueIdIndex},
    repository::RepositoryId,
    user::User,
};

use crate::{
    app::sync_engine_provider::use_sync_engine, frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

use super::{
    Part1OwnerNamePart2RepoNamePart3EmptyCaptures, Part1OwnerNamePart2RepoNamePart3IssuesCaptures,
};

#[allow(non_snake_case)]
pub fn IssuesTabEmpty(
    #[allow(unused_variables)] child_component: impl Fn(()) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1OwnerNamePart2RepoNamePart3EmptyCaptures>,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    IssuesTab(repository_id)
}
pub fn IssuesTabWithIssueId(
    #[allow(unused_variables)] child_component: impl Fn(()) -> AnyView + Send + Sync,
    #[allow(unused_variables)] captures: Memo<Part1OwnerNamePart2RepoNamePart3IssuesCaptures>,
    #[allow(unused_variables)] repository_id: Signal<RepositoryId>,
) -> impl IntoView {
    IssuesTab(repository_id)
}

pub fn IssuesTab(#[allow(unused_variables)] repository_id: Signal<RepositoryId>) -> impl IntoView {
    let sync_engine = use_sync_engine();

    let issues = sync_engine.idb_signal(
        |builder| builder.with_store::<Issue>().build(),
        move |txn| async move {
            Ok(txn
                .object_store::<Issue>()?
                .index::<RepositoryIdIndex>()?
                .get_all(Some(&repository_id.read()))
                .await?)
        },
    );

    let issues = Signal::derive(move || {
        Ok::<_, FrontendError>(
            issues
                .get()
                .transpose()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        )
    });

    let issues_len = Memo::new(move |_| {
        issues
            .read()
            .as_ref()
            .map(|i| i.len())
            .map_err(|i| i.clone())
    });

    let counts = Memo::new(move |_| {
        let issues = issues()?;
        let (open, closed): (Vec<Option<Timestamp>>, Vec<_>) = issues
            .iter()
            .filter_map(|i| i.closed_at.as_ref().to_option())
            .partition(|i| i.is_none());
        Ok::<_, FrontendError>((open.len(), closed.len()))
    });

    view! {
        <>
            <div class="bg-gray-100 border rounded-t-md p-3 flex flex-nowrap justify-between">
                <div class="flex flex-nowrap gap-x-2">
                    <div>Open {move || counts().map(|c| c.0)}</div>
                    <div>Closed {move || counts().map(|c| c.1)}</div>
                </div>
                <div>Author</div>
            </div>
            <For
                each=move || issues.read().iter().flatten().cloned().enumerate().collect_vec()
                key=move |(_, i)| i.id
                children=move |(i, issue)| {
                    let issues_len = (*issues_len.read()).clone()?;
                    Ok::<
                        _,
                        FrontendError,
                    >(view! { <IssueRow issue=issue.clone() is_last=i == issues_len - 1 /> })
                }
            />
        </>
    }
    .into_any()
}

#[component]
pub fn IssueRow(issue: Issue, #[prop(optional)] is_last: bool) -> impl IntoView {
    let sync_engine = use_sync_engine();
    let user_id = issue.user_id.clone();
    let user = sync_engine.idb_signal(
        |builder| builder.with_store::<User>().build(),
        move |txn| {
            let user_id = user_id.clone();
            async move {
                let user_id = match user_id.to_option().flatten() {
                    Some(u) => u,
                    None => return Ok(None),
                };
                Ok(txn.object_store::<User>()?.get(&user_id).await?)
            }
        },
    );
    let issue_id = issue.id;
    let comments_count = sync_engine.idb_signal(
        |builder| builder.with_store::<IssueComment>().build(),
        move |txn| {
            let issue_id = issue_id;
            async move {
                Ok(txn
                    .object_store::<IssueComment>()?
                    .index::<IssueIdIndex>()?
                    .get_all(Some(&Some(issue_id)))
                    .await?
                    .len())
            }
        },
    );
    let comments_count = Memo::new(move |_| comments_count.get());

    let created_at = issue.created_at.clone();
    let closed_at = issue.closed_at.clone();
    let title = issue.title.clone();
    let number = issue.number;

    let login = user.get().transpose()?.flatten().map(|u| u.login.clone());

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

    Ok::<_, FrontendError>(view! {
        <div
            class="border-r border-l border-b p-3 flex justify-between items-center"
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
                <div class="flex gap-2 items-center">
                    <div>comments</div>
                    {comments_count}
                </div>
            </div>
        </div>
    })
}
