use leptos::prelude::*;

use crate::app::{flowbite::{text_area::TextArea, text_input::TextInput}, thirds::Thirds};

#[component]
pub fn NewIssue() -> impl IntoView {
    let title = RwSignal::new(Default::default());
    let body = RwSignal::new(Default::default());
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
                    </div>
                </div>
            }
            one_third=()
        />
    }
}
