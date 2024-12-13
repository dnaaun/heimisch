use leptos::prelude::use_context;

#[allow(unused)]
pub fn use_unwrapped_context<T: Clone + 'static>() -> T {
    use_context().unwrap_or_else(|| panic!("Expected context to be defined"))
}
