use leptos::prelude::*;
use macros::zwang_url;
use zwang_router::set_pathname;

use crate::app::{flowbite::button::Button, routing::*};

#[component]
pub fn NewIssueButton(
    #[prop(into)] owner_name: Signal<String>,
    #[prop(into)] repo_name: Signal<String>,
) -> impl IntoView {
    move || {
        let owner_name = owner_name.get();
        let repo_name = repo_name.get();
        let on_click = move |_ev| {
            let owner_name = owner_name.clone();
            let repo_name = repo_name.clone();
            let url = zwang_url!("/owner_name={owner_name}/repo_name={repo_name}/issues/new");
            set_pathname(&url);
        };
        view! {
            <Button on:click=on_click>

                New Issue
            </Button>
        }
    }
}
