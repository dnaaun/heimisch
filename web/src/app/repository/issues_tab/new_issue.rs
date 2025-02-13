use crate::app::repository::RepositoryPageContextInner;
use crate::app::routing::*;
use github_api::models::IssuesCreateRequestTitle;
use leptos::{prelude::*, task::spawn_local};
use macros::zwang_url;
use shared::{types::issue::Issue, utils::LogErr};
use zwang_router::{set_pathname, ArgFromParent};

use crate::app::{
    flowbite::{
        button::{Button, ButtonColor},
        text_area::TextArea,
        text_input::TextInput,
    },
    repository::RepositoryPageContext,
    sync_engine_provider::use_sync_engine,
    thirds::Thirds,
};

#[allow(non_snake_case)]
pub fn NewIssue(
    ArgFromParent(repository_page_context): ArgFromParent<RepositoryPageContext>,
) -> impl IntoView {
    let title = RwSignal::new(Default::default());
    let body = RwSignal::new(Default::default());
    move || {
        let sync_engine = use_sync_engine();
        let on_create = move |_| {
            let RepositoryPageContextInner {
                user: owner,
                repository,
            } = repository_page_context.get();
            let issues_create_request = github_api::models::IssuesCreateRequest {
                title: IssuesCreateRequestTitle::String(title.get()),
                body: Some(body.get()),
                ..Default::default()
            };
            let sync_engine = sync_engine.clone();
            let repo_name = repository.name.clone();
            spawn_local(async move {
                let issue_id = sync_engine
                    .create_issue(
                        &repository.installation_id,
                        &owner,
                        &repository,
                        issues_create_request,
                    )
                    .await
                    .log_err();

                if let Ok(issue_id) = issue_id {
                    let txn = sync_engine.db.txn().with_store::<Issue>().build();
                    let issue_number = txn
                        .object_store::<Issue>()
                        .unwrap()
                        .get(&issue_id)
                        .await
                        .unwrap()
                        .unwrap()
                        .number;
                    let owner_login = &repository_page_context.get().user.login;
                    let issue_number = issue_number.to_string();
                    set_pathname(zwang_url!("/owner_name={owner_login}/repo_name={repo_name}/issues/issue_number={issue_number}"));
                }
            });
        };
        view! {
            <Thirds
                two_thirds=view! {
                    <div class="flex flex-nowrap gap-2">
                        <div class="w-4"></div>
                        <div class="flex-grow flex flex-col gap-3">
                            <div class="font-semibold text-lg">Create new issue</div>
                            <TextInput
                                label={
                                    view! { <div>Add a title <span class="text-red-400">*</span></div> }
                                }
                                value=title
                                placeholder="Title"
                            />
                            <TextArea
                                label={
                                    view! { <div>Add a description</div> }
                                }
                                value=body
                                placeholder="Type your description here..."
                                rows=20
                            />
                            <div
                            class="flex gap-2 flex-end justify-end flex-end mb-4">
                            <Button color=ButtonColor::Light>Cancel</Button>
                            <Button on:click=on_create>Create</Button>
                            </div>
                        </div>
                    </div>
                }
                one_third=()
            />
        }
    }
}
