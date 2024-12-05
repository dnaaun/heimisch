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
    #[prop(into)] active_tab: Signal<Key>,
    set_active_tab: impl Fn(Key) + Send + Sync + 'static,
) -> impl IntoView
where
    Key: ToString + Send + Sync + 'static + Clone + Eq + Hash,
{
    let set_active_tab = Arc::new(set_active_tab);

    // let active2 = move || active_tab.read().to_string();
    // tracing::info!("{}", active2());

    let active_tab_content_el = move || {
        tabs.get()
            .iter()
            .find(|t| t.key == active_tab())
            .map(|t| (t.content_el)())
    };
    view! {
        <div>
        <div class="hyey text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
            <ul class="flex flex-wrap -mb-px">
            <For
                each=move || tabs.get()
                key=|tab| tab.key.clone()
                children={move |tab: Tab<Key>| {
                    let set_active_tab = set_active_tab.clone();
                    let key = tab.key.clone();
                    view! {
                        <li class="me-2">
                            <a
                            href="#"
                            on:click={move |_| set_active_tab(key.clone())}
                            class={move ||
                                if tab.key == active_tab.get() {
                                    "inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500" }
                                else {
                                    "inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300"
                                }
                            }
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
        <div class="flex items-center justify-center w-screen">
            <div class="m-5 max-w-screen-xl w-screen">
                {active_tab_content_el}
            </div>
        </div>
        </div>
    }
}
