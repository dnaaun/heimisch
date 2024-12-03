use leptos_reactive::use_context;

pub fn use_unwrapped_context<T: Clone + 'static>() -> T {
    use_context().expect("Expected context to be defined")
}
