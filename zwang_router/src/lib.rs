mod slashed_and_segmented;

pub use slashed_and_segmented::*;

use leptos::{prelude::*, tachys::html::class::IntoClass};
use serde::de::DeserializeOwned;
use std::{fmt::Display, ops::Deref, sync::Arc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub use macros::{zwang_url, zwang_routes};

pub trait MemoExt<T>
where
    T: Send + Sync + 'static,
{
    fn unwrap(self) -> Memo<T>;
}

impl<T> MemoExt<T> for Memo<Option<T>>
where
    T: Clone + Send + Sync + 'static + PartialEq,
{
    fn unwrap(self) -> Memo<T> {
        Memo::new(move |_| {
            let option: Option<T> = self.get();
            option.unwrap()
        })
    }
}

struct PathnameManager {
    /// We're making this ArcMemo and not ArcRwSignal because sometimes, popstate gets emitted when
    /// set_pathname() is used, and sometimes it isn't. And I want to avoid doing double rerenders.
    pathname: ArcMemo<String>,
    search: ArcMemo<String>,
    set_pathname: ArcWriteSignal<String>,
    set_search: ArcWriteSignal<String>,
}

impl PathnameManager {
    fn new() -> Self {
        let location = window().location();
        let (pathname, set_pathname) = ArcRwSignal::new(location.pathname().expect("")).split();
        let (search, set_search) = ArcRwSignal::new(location.search().expect("")).split();

        let set_pathname2 = set_pathname.clone();
        let set_search2 = set_search.clone();
        let cb = move || {
            let new_location = window().location();
            let new_pathname = new_location.pathname().expect("");
            let new_search = new_location.search().expect("");
            set_pathname2(new_pathname);
            set_search2(new_search);
        };
        let closure = Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();

        window()
            .add_event_listener_with_callback("popstate", closure.as_ref().dyn_ref().expect(""))
            .expect("");

        Self {
            pathname: ArcMemo::new(move |_| pathname.get()),
            search: ArcMemo::new(move |_| search.get()),
            set_pathname,
            set_search,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ArgFromParent<T>(pub T);

impl<A> Deref for ArgFromParent<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Outlet<A = (), V = AnyView>(Arc<dyn Fn(A) -> V + Send + Sync + 'static>);

impl<A, V> Outlet<A, V> {
    pub fn call(&self, a: A) -> V {
        (self.0)(a)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RouteParams<T>(pub T);

impl<T> Deref for RouteParams<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RoutingInfoForComponent<ArgFromParentInner, OutletInner, ParamsInner>
where
    ArgFromParentInner: Clone,
    OutletInner: Clone,
    ParamsInner: Clone,
{
    pub arg_from_parent: ArgFromParentInner,
    pub outlet: OutletInner,
    pub params: ParamsInner,
}

impl<A, C, P> From<&RoutingInfoForComponent<A, C, P>> for ArgFromParent<A>
where
    A: Clone,
    C: Clone,
    P: Clone,
{
    fn from(value: &RoutingInfoForComponent<A, C, P>) -> Self {
        Self(value.arg_from_parent.clone())
    }
}

impl<A, CA, CV, P> From<&RoutingInfoForComponent<A, Arc<dyn Fn(CA) -> CV + Send + Sync>, P>>
    for Outlet<CA, CV>
where
    A: Clone,
    CA: Clone,
    P: Clone,
{
    fn from(value: &RoutingInfoForComponent<A, Arc<dyn Fn(CA) -> CV + Send + Sync>, P>) -> Self {
        Self(value.outlet.clone())
    }
}

impl<A, C, P> From<&RoutingInfoForComponent<A, C, P>> for RouteParams<P>
where
    A: Clone,
    C: Clone,
    P: Clone,
{
    fn from(value: &RoutingInfoForComponent<A, C, P>) -> Self {
        Self(value.params.clone())
    }
}

pub trait RoutableComponent<ArgFromParent, ChildComp, Param, ArgsTuple>
where
    ArgFromParent: Clone,
    ChildComp: Clone,
    Param: Clone,
{
    fn into_view_with_route_info(
        self,
        info: RoutingInfoForComponent<ArgFromParent, ChildComp, Param>,
    ) -> impl IntoView;
}

impl<ArgFromParent, ChildComp, Param, F, V> RoutableComponent<ArgFromParent, ChildComp, Param, ()>
    for F
where
    F: Fn() -> V,
    V: IntoView,
    ArgFromParent: Clone,
    ChildComp: Clone,
    Param: Clone,
{
    fn into_view_with_route_info(
        self,
        _info: RoutingInfoForComponent<ArgFromParent, ChildComp, Param>,
    ) -> impl IntoView {
        self()
    }
}

impl<ArgFromParent, ChildComp, Param, F, V, A1>
    RoutableComponent<ArgFromParent, ChildComp, Param, (A1,)> for F
where
    F: Fn(A1) -> V,
    V: IntoView,
    ArgFromParent: Clone,
    ChildComp: Clone,
    Param: Clone,
    A1: for<'a> From<&'a RoutingInfoForComponent<ArgFromParent, ChildComp, Param>>,
{
    fn into_view_with_route_info(
        self,
        info: RoutingInfoForComponent<ArgFromParent, ChildComp, Param>,
    ) -> impl IntoView {
        self((&info).into())
    }
}

impl<ArgFromParent, ChildComp, Param, F, V, A1, A2>
    RoutableComponent<ArgFromParent, ChildComp, Param, (A1, A2)> for F
where
    F: Fn(A1, A2) -> V,
    V: IntoView,
    ArgFromParent: Clone,
    ChildComp: Clone,
    Param: Clone,
    A1: for<'a> From<&'a RoutingInfoForComponent<ArgFromParent, ChildComp, Param>>,
    A2: for<'a> From<&'a RoutingInfoForComponent<ArgFromParent, ChildComp, Param>>,
{
    fn into_view_with_route_info(
        self,
        info: RoutingInfoForComponent<ArgFromParent, ChildComp, Param>,
    ) -> impl IntoView {
        self((&info).into(), (&info).into())
    }
}

impl<ArgFromParent, ChildComp, Param, F, V, A1, A2, A3>
    RoutableComponent<ArgFromParent, ChildComp, Param, (A1, A2, A3)> for F
where
    F: Fn(A1, A2, A3) -> V,
    V: IntoView,
    ArgFromParent: Clone,
    ChildComp: Clone,
    Param: Clone,
    A1: for<'a> From<&'a RoutingInfoForComponent<ArgFromParent, ChildComp, Param>>,
    A2: for<'a> From<&'a RoutingInfoForComponent<ArgFromParent, ChildComp, Param>>,
    A3: for<'a> From<&'a RoutingInfoForComponent<ArgFromParent, ChildComp, Param>>,
{
    fn into_view_with_route_info(
        self,
        info: RoutingInfoForComponent<ArgFromParent, ChildComp, Param>,
    ) -> impl IntoView {
        self((&info).into(), (&info).into(), (&info).into())
    }
}

thread_local! {
    pub static PATHNAME_MANAGER: PathnameManager = PathnameManager::new();
}

pub fn use_pathname() -> ArcMemo<String> {
    PATHNAME_MANAGER.with(|i| i.pathname.clone())
}

pub fn set_pathname(path: impl ToString) {
    window()
        .history()
        .expect("")
        .push_state_with_url(&JsValue::NULL, "", Some(&path.to_string()))
        .expect("");
    PATHNAME_MANAGER.with(|i| (i.set_pathname)(window().location().pathname().expect("")));
}

pub fn set_search(search: impl ToString) {
    let search = search.to_string();
    window().location().set_search(&search).expect("");
    PATHNAME_MANAGER.with(|i| (i.set_search)(window().location().search().expect("")));
}

pub fn use_search() -> ArcMemo<String> {
    PATHNAME_MANAGER.with(|i| i.search.clone())
}

pub fn use_serde_search<T>() -> Signal<Result<T, serde::de::value::Error>>
where
    T: Send + Sync + DeserializeOwned + 'static,
{
    let search = use_search();
    Signal::derive(move || serde_urlencoded::from_str(search.get().strip_prefix("?").unwrap_or("")))
}

#[component]
pub fn A(
    #[prop(into)] href: Signal<String>,
    class: impl IntoClass,
    children: Children,
) -> impl IntoView {
    let href = href.get();
    view! {
        <a
            class=class
            href=href.clone()
            on:click=move |ev| {
                set_pathname(href.clone());
                ev.prevent_default();
            }
        >
            {children()}
        </a>
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoPart;

impl<'a> TryFrom<Slashed<'a>> for NoPart {
    type Error = String;

    fn try_from(value: Slashed<'a>) -> std::result::Result<Self, Self::Error> {
        if value.non_slash_len() == 0 {
            Ok(Self)
        } else {
            Err(format!("non slash length is not 0 in '{value}'"))
        }
    }
}

// Ensure `NoPart` has a `Display` implementation if it is a custom type
impl std::fmt::Display for NoPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement according to its actual structure and desired string representation.
        write!(f, "")
    }
}

pub fn empty_component<ArgFromParent>(_: ArgFromParent) -> impl IntoView {
}

pub fn passthrough_component<A, V>(
    ArgFromParent(arg_from_parent): ArgFromParent<A>,
    outlet: Outlet<A, V>,
) -> impl IntoView
where
    V: IntoView,
{
    outlet.call(arg_from_parent)
}

pub fn not_found_compnent() -> impl IntoView {
    "Zwang router: route not found."
}

#[derive(Clone, Debug, Copy)]
pub struct ParsedPath<T: Sync + Send + 'static>(pub ::leptos::prelude::Memo<Result<T, String>>);

impl<T: Sync + Send + 'static> std::ops::Deref for ParsedPath<T> {
    type Target = ::leptos::prelude::Memo<Result<T, String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct PathAndRest<T: ToString> {
    pub path: T,
    pub rest: String,
}

impl<T: ToString> Display for PathAndRest<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.path.to_string(), self.rest)
    }
}
