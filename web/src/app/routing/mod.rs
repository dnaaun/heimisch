use derive_more::Deref;
mod defns;

pub use defns::*;

use leptos::{prelude::*, tachys::html::class::IntoClass};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

trait RouteComponent<PassedFromParent> {
    type ToPassToChild;
    fn render(
        &self,
        passed_from_parent: PassedFromParent,
        child_component: Box<dyn Fn(Self::ToPassToChild) -> AnyView + Send + Sync>,
    ) -> AnyView;
}

struct PathnameManager {
    /// We're making this ArcMemo and not ArcRwSignal because sometimes, popstate gets emitted when
    /// set_pathname() is used, and sometimes it isn't. And I want to avoid doing double rerenders.
    pathname: ArcMemo<String>,
    set_pathname: ArcWriteSignal<String>,
}

impl PathnameManager {
    fn new() -> Self {
        let location = window().location();
        let (pathname, set_pathname) = ArcRwSignal::new(location.pathname().expect("")).split();

        let set_pathname2 = set_pathname.clone();
        let cb = move || {
            let new_pathname = window().location().pathname().expect("");
            set_pathname2(new_pathname);
        };
        let closure = Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();

        window()
            .add_event_listener_with_callback("popstate", closure.as_ref().dyn_ref().expect(""))
            .expect("");

        Self {
            pathname: ArcMemo::new(move |_| pathname.get()),
            set_pathname,
        }
    }
}

thread_local! {
    pub static PATHNAME_MANAGER: PathnameManager = PathnameManager::new();
}

pub fn use_pathname() -> Signal<String> {
    PATHNAME_MANAGER.with(|i| i.pathname.clone()).into()
}

pub fn set_pathname(path: impl ToString) {
    window()
        .history()
        .expect("")
        .push_state_with_url(&JsValue::NULL, "Some crazy title", Some(&path.to_string()))
        .expect("");
    PATHNAME_MANAGER.with(|i| (i.set_pathname)(window().location().pathname().expect("")));
}

#[component]
pub fn Routed() -> impl IntoView {
    let pathname = use_pathname();
    let top_level = Memo::new(move |_| pathname.read().parse::<TopLevel>());
    move || {
        view! {
            {match top_level.read().deref() {
                Ok(top_level) => top_level.render((), Box::new(|_| ().into_any())),
                Err(_) => todo!(),
            }}
        }
    }
}

#[component]
pub fn A(
    #[prop(into)] href: Option<String>,
    class: impl IntoClass,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            class=class
            href=href.clone()
            on:click=move |ev| {
                if let Some(href) = href.clone() {
                    set_pathname(href);
                }
                ev.prevent_default();
            }
        >
            {children()}
        </a>
    }
}
