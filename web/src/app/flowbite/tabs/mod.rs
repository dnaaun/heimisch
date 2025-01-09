use std::hash::Hash;

use leptos::prelude::*;

#[component]
pub fn Tabs<Key>(
    #[prop(into)] tabs: Signal<Vec<Key>>,
    #[prop(into)] active_tab: Signal<Key>,
    get_tab_label: impl Fn(&Key) -> String + Send + Sync + 'static,
    set_active_tab: impl Fn(Key) + Send + Sync + 'static,
) -> impl IntoView
where
    Key: 'static + Clone + Eq + Hash + Clone + Send + Sync,
{
    let set_active_tab = StoredValue::new(set_active_tab);
    let get_tab_label = StoredValue::new(get_tab_label);

    let for_children = move |key: Key| {
        let key = StoredValue::new(key);
        view! {
            <li class="me-2">
                <a
                    href="#"
                    on:click=move |ev| {
                        set_active_tab.read_value()(key.get_value());
                        ev.prevent_default();
                    }
                    class=move || {
                        if key.get_value() == active_tab.get() {
                            "inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500"
                        } else {
                            "inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300"
                        }
                    }
                    aria-current="page"
                >
                    {get_tab_label.read_value()(&key.read_value())}
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
