use github_api::models::IssuesCreateRequestTitle;
use leptos::{prelude::*, task::spawn_local};
use shared::{avail::Avail, utils::LogErr};
use zwang_router::ArgFromParent;

use crate::app::{
    flowbite::{
        button::{Button, ButtonColor},
        text_area::TextArea,
        text_input::TextInput,
    },
    repository::RepositoryPageWillPass,
    sync_engine_provider::use_sync_engine,
    thirds::Thirds,
};

#[allow(non_snake_case)]
pub fn NewIssue(ArgFromParent(repository): ArgFromParent<RepositoryPageWillPass>) -> impl IntoView {
    let title = RwSignal::new(Default::default());
    let body = RwSignal::new(Default::default());
    move || {
        let repository = repository.get();
        let sync_engine = use_sync_engine();
        let on_create = move |_| {
            if let Avail::Yes(owner_id) = repository.owner_id {
                let issues_create_request = github_api::models::IssuesCreateRequest {
                    title: IssuesCreateRequestTitle::String(title.get()),
                    body: Some(body.get()),
                    ..Default::default()
                };
                let sync_engine = sync_engine.clone();
                spawn_local(async move {
                    let _ = sync_engine
                        .create_issue(
                            &repository.installation_id,
                            &owner_id,
                            &repository.id,
                            issues_create_request,
                        )
                        .await
                        .log_err();
                });
            } else {
                tracing::info!(".owner_id not Avail-able");
            }
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
