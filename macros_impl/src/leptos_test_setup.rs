use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// NOTE: Should probably name this "leptos test setup"
pub fn leptos_test_setup(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_vis = &input.vis;
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let asyncness = &input.sig.asyncness;
    let input_args = &input.sig.inputs;
    let output = &input.sig.output;

    let expanded = quote! {
        #[::wasm_bindgen_test::wasm_bindgen_test]
        #fn_vis #asyncness fn #fn_name(#input_args) #output {
            {
                let buffer = ::std::sync::Arc::new(::std::sync::Mutex::new(Vec::new()));
                let writer_factory = ::leptos_testing_utils::leptos_test_setup::MemoryWriterFactory {
                    buffer: ::std::sync::Arc::clone(&buffer),
                };
                let subscriber = ::tracing_subscriber::fmt()
                    .without_time()
                    .with_writer(writer_factory)
                    .with_max_level(::tracing::Level::TRACE)
                    .finish();
                let _ = ::tracing::subscriber::set_global_default(subscriber);
            };

             #fn_body
        }
    };

    TokenStream::from(expanded)
}
