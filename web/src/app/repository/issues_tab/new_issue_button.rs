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
        let on_click = move |_ev| {
            let url =
                zwang_url!("/owner_name={owner_name.get()}/repo_name={repo_name.get()}/issues/new");
            set_pathname(&url);
        };
        view! {
            <Button on:click=on_click>

                New Issue
            </Button>
        }
    }
}
