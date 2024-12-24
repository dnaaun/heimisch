use std::hash::Hash;
use std::sync::Arc;

use leptos::prelude::*;

#[component]
pub fn Tabs<Key>(
    #[prop(into)] tabs: Signal<Vec<Key>>,
    #[prop(into)] active_tab: Signal<Key>,
    set_active_tab: impl Fn(Key) + Send + Sync + 'static,
) -> impl IntoView
where
    Key: ToString + Send + Sync + 'static + Clone + Eq + Hash + Clone,
{
    let set_active_tab = Arc::new(set_active_tab);

    let for_children = move |key: Key| {
        let set_active_tab = set_active_tab.clone();
        let key2 = key.clone();
        view! {
            <li class="me-2">
                <a
                    href="#"
                    on:click=move |_| set_active_tab(key2.clone())
                    class=move || {
                        if key.clone() == active_tab.get() {
                            "inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500"
                        } else {
                            "inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300"
                        }
                    }
                    aria-current="page"
                >
                    {key.to_string()}
                </a>
            </li>
        }
        .into_any()
    };
    view! {
        <div class="hyey text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
            <ul class="flex flex-wrap -mb-px">
                <For each=tabs key=|k| k.clone() children=for_children />
            </ul>
        </div>
    }
}
