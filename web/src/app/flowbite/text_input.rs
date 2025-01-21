use leptos::prelude::*;

/// Dude, this uses: https://flowbite.com/docs/forms/input-field/
#[component]
pub fn TextInput(
    #[prop(optional)] label: Option<impl IntoView + 'static>,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] placeholder: Signal<String>,
    #[prop(into, optional)] id: Signal<String>,
    #[prop(into, optional)] required: Signal<bool>,
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
            <input
                bind:value=value
                type="text"
                id
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder=placeholder
                required=required
            />
        </div>
    }
}
