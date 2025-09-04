use jiff::{fmt::strtime, Timestamp};
use leptos::prelude::*;
use macros::zwang_url;
use shared::{
    sync_engine::optimistic::db::MaybeOptimistic,
    types::{
        issue::{Issue, RepositoryIdIndex},
        issue_comment::{IssueComment, IssueIdIndex},
        user::User,
    },
};

use itertools::Itertools;
use zwang_router::{ArgFromParent, RouteParams, A};

use crate::{
    app::{
        repository::{issues_tab::new_issue_button::NewIssueButton, RepositoryPageContext},
        routing::*,
        sync_engine_provider::use_sync_engine,
    },
    frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

#[allow(non_snake_case)]
pub fn IssuesList(
    RouteParams(ParamsOwnerNameRepoName {
        owner_name,
        repo_name,
    }): RouteParams<ParamsOwnerNameRepoName>,
    ArgFromParent(repository_page_context): ArgFromParent<RepositoryPageContext>,
) -> impl IntoView {
    let sync_engine = use_sync_engine();

    let issues = sync_engine.idb_signal(
        async |builder| builder.with_table::<Issue>().build().await,
        move |txn| async move {
            let issues = txn
                .table::<Issue>()
                .index::<RepositoryIdIndex>()
                .get_all_optimistically(Some(&repository_page_context.read().repository.id))
                .await?;
            Ok(issues)
        },
    );

    let issues = Signal::derive(move || {
        let mut issues = issues
            .get()
            .transpose()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        issues.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok::<_, FrontendError>(issues)
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
        <div>
            <div class="mb-2 flex flex-row-reverse justify-start">
                <NewIssueButton owner_name repo_name />
            </div>
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
                    >(
                        view! {
                            <IssueRow
                                owner_name
                                repo_name
                                issue=issue.clone()
                                is_last=i == issues_len - 1
                            />
                        },
                    )
                }
            />
        </div>
    }
}

#[component]
fn IssueRow(
    issue: MaybeOptimistic<Issue>,
    #[prop(into)] owner_name: Signal<String>,
    #[prop(into)] repo_name: Signal<String>,
    #[prop(optional)] is_last: bool,
) -> impl IntoView {
    let sync_engine = use_sync_engine();
    let user_id = issue.user_id.clone();
    let user = sync_engine.idb_signal(
        async |builder| builder.with_table::<User>().build().await,
        move |txn| {
            let user_id = user_id.clone();
            async move {
                let user_id = match user_id.to_option().flatten() {
                    Some(u) => u,
                    None => return Ok(None),
                };
                Ok(txn.table::<User>().get_optimistically(&user_id).await?)
            }
        },
    );
    let id = issue.id;
    let comments_count = sync_engine.idb_signal(
        async |builder| builder.with_table::<IssueComment>().build().await,
        move |txn| async move {
            Ok(txn
                .table::<IssueComment>()
                .index::<IssueIdIndex>()
                .get_all_optimistically(Some(&Some(id)))
                .await?
                .len())
        },
    );
    let comments_count = Memo::new(move |_| comments_count.get());

    let created_at = issue.created_at.clone();
    let closed_at = issue.closed_at.clone();
    let title = issue.title.clone();
    let number = issue.number;

    let login = move || -> Result<_, FrontendError> {
        let user = user.get().transpose()?.flatten();
        Ok(user.map(|u| u.login.clone()))
    };

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

    let href = Signal::derive(move || {
        // TODO: Figure out why doing `.get()` makes leptos warn that we're
        // reading these `owner_name` and `repo_name` memos in an "unreactive"
        // context. Can't think of anything that should be more reactive than
        // the body of a Signal::derive.
        let owner_name = owner_name.get_untracked();
        let repo_name = repo_name.get_untracked();
        zwang_url!("/owner_name={owner_name}/repo_name={repo_name}/issues/issue_number={number.to_string()}")
            .to_string()
    });
    Ok::<_, FrontendError>(view! {
        <div
            class="border-r border-l border-b p-3 flex justify-between items-center"
            class=("rounded-b", is_last)
        >
            <div>
                <A class="mb-1.5 font-bold" href=href>
                    {title.to_option()}
                </A>
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
