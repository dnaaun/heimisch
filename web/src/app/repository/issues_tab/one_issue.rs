use futures::future::{join_all, OptionFuture};
use github_api::models::milestone::OpenOrClosed;
use jiff::Timestamp;
use leptos::prelude::*;
use shared::{
    avail::Avail,
    types::{
        issue::{self, Issue},
        issue_comment::{IssueComment, IssueIdIndex},
        user::User,
    },
};
use zwang_router::{ArgFromParent, RouteParams};

use crate::{
    app::{
        flowbite::{
            button::{Button, ButtonColor},
            pill_badge::{PillBadge, PillBadgeColor},
        },
        not_found::NotFound,
        repository::RepositoryPageWillPass,
        routing::ParamsIssueNumberOwnerNameRepoName,
        sync_engine_provider::use_sync_engine,
        thirds::Thirds,
    },
    frontend_error::FrontendError,
    idb_signal::IdbSignal,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

use super::new_issue_button::NewIssueButton;

#[allow(non_snake_case)]
pub fn OneIssue(
    ArgFromParent(repo): ArgFromParent<RepositoryPageWillPass>,
    RouteParams(ParamsIssueNumberOwnerNameRepoName {
        issue_number,
        owner_name,
        repo_name,
    }): RouteParams<ParamsIssueNumberOwnerNameRepoName>,
) -> impl IntoView {
    let issue_number = move || issue_number.get().parse::<i64>();
    move || {
        let sync_engine = use_sync_engine();
        let issue_number = match issue_number() {
            Ok(i) => i,
            Err(_) => return view! { <NotFound /> }.into_any(),
        };
        let issue_and_user: IdbSignal<Result<Option<_>, _>> = sync_engine.idb_signal(
            move |txn| txn.with_store::<User>().with_store::<Issue>().build(),
            move |txn| async move {
                let issue = txn
                    .object_store::<Issue>()?
                    .index::<issue::RepositoryIdIndex>()?
                    .get_all(Some(&repo.read().id))
                    .await?
                    .into_iter()
                    .find(move |i| i.number == issue_number);

                Ok(if let Some(issue) = issue {
                    let user = if let Avail::Yes(Some(user_id)) = issue.user_id {
                        txn.object_store::<User>()?.get(&user_id).await?
                    } else {
                        None
                    };
                    Some((issue, user))
                } else {
                    None
                })
            },
        );

        (move || {
            issue_and_user.get().map(|issue_and_user| {
                let sync_engine = use_sync_engine();
                let (issue, user) = match issue_and_user? {
                    Some((issue, user)) => (StoredValue::new(issue), StoredValue::<_>::new(user)),
                    None => return Ok::<_, FrontendError>(view! { <NotFound /> }.into_any()),
                };

                let issue_id = issue.get_value().id;
                let issue_comment_and_users = sync_engine.idb_signal(|builder| {
                    builder.with_store::<IssueComment>()
                        .with_store::<User>()
                        .build()
                }, move |txn| async move {
                    let mut issue_comments = txn.object_store::<IssueComment>()?.index::<IssueIdIndex>()?.get_all(Some(&Some(issue_id))).await?;
                    issue_comments.sort_by_key(|c| c.created_at.clone());
                    let user_store = txn.object_store::<User>() ?;
                    let users = join_all(issue_comments.iter().map(async |ic| {
                        OptionFuture::from(
                        ic.user_id.clone().to_option().flatten().map(async |user_id| {
                            user_store.get(&user_id).await
                        })).await

                    })).await
                    .into_iter()
                        .map(|u| u.transpose())
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(Option::flatten)
                        .collect::<Vec<_>>();

                    Ok(issue_comments.into_iter().zip(users).collect::<Vec<_>>())
                });



                Ok(
                view! {
                    <div>
                        <div class="flex justify-between">
                            <div class="flex items-center gap-2">
                                <div class="text-4xl font-extrabold dark:text-white">
                                    {issue.read_value().title.as_ref().to_option().cloned()}
                                </div>
                                <div class="text-4xl font-extrabold dark:text-gray-600 text-gray-400">
                                    #{issue.read_value().number}
                                </div>

                            </div>
                            <div class="flex gap-2">
                                <Button color=ButtonColor::Light>Edit</Button>
                                <NewIssueButton owner_name repo_name />
                            </div>

                        </div>
                        <div class="flex gap-2">
                            {issue
                                .read_value()
                                .state
                                .as_ref()
                                .to_option()
                                .map(|state| {
                                    let (text, color) = match state {
                                        OpenOrClosed::Open => ("Open", PillBadgeColor::Default),
                                        OpenOrClosed::Closed => ("Closed", PillBadgeColor::Indigo),
                                    };

                                    view! { <PillBadge color>{text}</PillBadge> }
                                })}
                        </div>
                        <div class="my-3 border-b border-gray-200 border-solid"></div>
                        <Thirds
                            two_thirds=view! {
                                <div class="flex flex-col gap-y-8 flex-grow">
                                    <IssueCommentBox
                                        body=issue.read_value().body.clone().to_option().flatten()
                                        login=user.read_value().as_ref().map(|u| u.login.clone())
                                        created_at=issue.read_value().created_at.clone().to_option()
                                    />
                                    {move || {
                                        Ok::<
                                            _,
                                            FrontendError,
                                        >(
                                            issue_comment_and_users
                                                .get()
                                                .transpose()?
                                                .map(|issue_comment_and_users| {

                                                    view! {
                                                        <For
                                                            each=move || issue_comment_and_users.clone()
                                                            key=|(ic, _)| ic.id
                                                            children=|(issue_comment, user)| {
                                                                view! {
                                                                    <IssueCommentBox
                                                                        body=issue_comment.body.to_option()
                                                                        created_at=issue_comment.created_at.to_option()
                                                                        login=user.as_ref().map(|u| u.login.clone())
                                                                    />
                                                                }
                                                            }
                                                        />
                                                    }
                                                }),
                                        )
                                    }}
                                </div>
                            }
                            one_third=()
                        />
                    </div>
                }.into_any())

            })
        })
        .into_any()
    }
}

#[component]
pub fn IssueCommentBox(
    #[prop(into)] login: Signal<Option<String>>,
    #[prop(into)] body: Signal<Option<String>>,
    #[prop(into)] created_at: Signal<Option<Timestamp>>,
) -> impl IntoView {
    let ago = move || {
        created_at.get().map(|c| {
            let formatter = timeago::Formatter::default();
            formatter.convert(c.duration_until(Timestamp::now()).unsigned_abs())
        })
    };
    view! {
        <div>
            <div class="bg-blue-50 border-t border-l border-r border-blue-200 flex flex-between p-2 rounded-t-lg">
                <div class="flex gap-1">
                    <div class="font-semibold">{login}</div>
                    <div class="text-gray-700">{ago}</div>
                </div>
                <div></div>
            </div>
            <div class="rounded-b-lg border-l border-r border-b border-blue-200 p-4 min-h-10">
                {body}
            </div>
        </div>
    }
}
