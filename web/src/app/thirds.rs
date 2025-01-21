use leptos::prelude::*;

#[component]
pub fn Thirds(
     two_thirds: impl IntoView + 'static,
     one_third: impl IntoView + 'static,
) -> impl IntoView {
    view! {
        <div class="flex gap-6">
            <div class="flex-grow">{two_thirds}</div>
            <div class="w-72">{one_third}</div>
        </div>
    }
}
