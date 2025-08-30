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

#[proc_macro_derive(Table, attributes(db))]
pub fn derive_table(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    macros_impl::derive_table(input)
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

#[proc_macro_attribute]
pub fn leptos_test_setup(_: TokenStream, input: TokenStream) -> TokenStream {
    macros_impl::leptos_test_setup::leptos_test_setup(input)
}
