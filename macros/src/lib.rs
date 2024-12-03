extern crate proc_macro;

use proc_macro::TokenStream;

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
