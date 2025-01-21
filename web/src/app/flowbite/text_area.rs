use leptos::prelude::*;

/// Dude, this uses: https://flowbite.com/docs/forms/textarea/
#[component]
pub fn TextArea(
    #[prop(optional)] label: Option<impl IntoView + 'static>,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] placeholder: Signal<String>,
    #[prop(into, optional)] id: Signal<String>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(optional, default = 4)] rows: u32,
) -> impl IntoView {
    view! {
        <div>
            {label
                .map(|label| {
                    view! {
                        <label
                            for=id
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            {label}
                        </label>
                    }
                })}
            <textarea
                bind:value=value
                id=id
                rows=rows
                class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder=placeholder
                required=required
            ></textarea>
        </div>
    }
}
