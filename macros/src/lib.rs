extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

#[proc_macro_derive(AvailMerge, attributes(id))]
pub fn derive_avail_merge(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    macros_impl::derive_avail_merge(input).into()
}

#[proc_macro_derive(TypesafeIdb, attributes(idb))]
pub fn derive_typesafe_idb(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    macros_impl::derive_typesafe_idb(input)
}

#[proc_macro]
pub fn zwang_routes(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    macros_impl::zwang_routes(input)
}

#[proc_macro]
pub fn zwang_url(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    macros_impl::zwang_url(input).into()
}

/// NOTE: Should probably name this "leptos test setup".
#[proc_macro_attribute]
pub fn tracing_to_console_log(_: TokenStream, input: TokenStream) -> TokenStream {
    macros_impl::tracing_to_console_log::tracing_to_console_log(input)
}
