use leptos::prelude::*;
use zwang_router::set_pathname;

use crate::app::{flowbite::button::Button, routing::*};

#[component]
pub fn NewIssueButton(
    #[prop(into)] owner_name: Signal<String>,
    #[prop(into)] repo_name: Signal<String>,
) -> impl IntoView {
    move || {
        let on_click = move |_ev| {
            set_pathname(Root::OwnerName {
                owner_name: owner_name.get(),
                child: RootOwnerName::RepoName {
                    repo_name: repo_name.get(),
                    child: RootOwnerNameRepoName::Issues(RootOwnerNameRepoNameIssues::New(
                        RootOwnerNameRepoNameIssuesNew::Empty,
                    )),
                },
            });
        };
        view! { <Button
        on:click=on_click

        >New Issue</Button> }
    }
}
