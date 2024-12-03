use std::hash::Hash;
use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone)]
pub struct Tab<Key> {
    pub content_el: Arc<dyn Fn() -> AnyView + Send + Sync>,

    pub key: Key,
}

#[component]
pub fn Tabs<Key>(
    #[prop(into)] tabs: Signal<Vec<Tab<Key>>>,
    #[prop(into)] active: Signal<Key>,
) -> impl IntoView
where
    Key: ToString + Send + Sync + 'static + Clone + Eq + Hash,
{
    view! {
        <div class="text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
            <ul class="flex flex-wrap -mb-px">
            <For
                each=move || tabs.get()
                key=|tab| tab.key.clone()
                children={move |tab: Tab<Key>| {
                    view! {
                        <li class="me-2">
                            <a
                            href="#"
                            class=("inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500", tab.key == active.get())
                            class=("inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300", tab.key != active.get())
                            aria-current="page"
                            >
                            {tab.key.to_string()}
                        </a>
                            </li>
                    }.into_any()

                }}
            />
            </ul>
        </div>
    }
}
