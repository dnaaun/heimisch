use github_api::models::milestone::OpenOrClosed;
use leptos::prelude::*;
use shared::types::{
    issue::{self, Issue},
    repository::RepositoryId,
};
use zwang_router::{ArgFromParent, RouteParams};

use crate::{
    app::{
        flowbite::{button::{Button, ButtonColor}, pill_badge::{PillBadge, PillBadgeColor}},
        not_found::NotFound,
        routing::ParamsIssueNumberOwnerNameRepoName,
        sync_engine_provider::use_sync_engine,
    },
    frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

#[allow(non_snake_case)]
pub fn OneIssue(
    ArgFromParent(repository_id): ArgFromParent<Signal<RepositoryId>>,
    RouteParams(ParamsIssueNumberOwnerNameRepoName { issue_number, .. }): RouteParams<
        ParamsIssueNumberOwnerNameRepoName,
    >,
) -> impl IntoView {
    let issue_number = move || issue_number.get().parse::<i64>();
    return move || {
        let sync_engine = use_sync_engine();
        let issue_number = match issue_number() {
            Ok(i) => i,
            Err(_) => return view! { <NotFound /> }.into_any(),
        };
        let issue = sync_engine.idb_signal(
            move |txn| txn.with_store::<Issue>().build(),
            move |txn| async move {
                Ok(txn
                    .object_store::<Issue>()?
                    .index::<issue::RepositoryIdIndex>()?
                    .get_all(Some(&repository_id.read()))
                    .await?
                    .into_iter()
                    .filter(move |i| i.number == issue_number)
                    .next())
            },
        );

        

        (move || {
            issue.get().map(|issue| {
                let issue = match issue? {
                    Some(i) => i,
                    None => return Ok(view! { <NotFound /> }.into_any()),
                };
                Ok::<_, FrontendError>(
                    view! {
                        <div>
                            <div class="flex justify-between">
                            <div class="flex items-center gap-2">
                            <div class="text-4xl font-extrabold dark:text-white">{issue.title.to_option()}</div>
                            <div class="text-4xl font-extrabold dark:text-gray-600 text-gray-400">#{issue.number}</div>
                            
                            </div>
                            <div class="flex gap-2">
                            <Button color=ButtonColor::Light>Edit</Button>
                            <Button>New Issue</Button>
                            </div>

                            </div>
                            <div class="flex gap-2">
                            {
                                issue.state.to_option().map(|state| 
                                    {
                                        let (text, color) = match state {
                                        OpenOrClosed::Open => ("Open", PillBadgeColor::Default),
                                        OpenOrClosed::Closed => ("Closed", PillBadgeColor::Indigo),
                                    };

                            view! { <PillBadge color>{text}</PillBadge> }
                                    }
                                )
                            }
                            </div>
                            </div>
                            <div class="mt-3 border-b border-gray-200 border-solid"></div>
                    }
                    .into_any(),
                )
            })
        })
        .into_any()
    };
}
