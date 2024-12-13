use leptos::prelude::*;

use crate::app::{flowbite::Spinner, icon::Icon};

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="h-screen w-screen flex justify-center items-center">
            <Spinner />
        </div>
    }
}
