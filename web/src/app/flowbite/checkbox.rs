use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(into)]
    checked: RwSignal<bool>,
    #[prop(into, optional)]
    label: Option<Signal<String>>
    ) -> impl IntoView {
    view! {
        <div class="flex items-center">
            <input
                bind:checked=checked
                type="checkbox"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
            />
            {move || {
                label
                    .get()
                    .map(|l| {
                        view! {
                            <label
                                for="default-checkbox"
                                class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                            >
                                {l}
                            </label>
                        }
                    })
            }}
        </div>
    }
}
