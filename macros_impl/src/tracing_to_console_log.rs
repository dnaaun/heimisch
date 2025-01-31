use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn tracing_to_console_log(item: TokenStream) -> TokenStream {
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
            let buffer = ::std::sync::Arc::new(::std::sync::Mutex::new(Vec::new()));

            let writer_factory = ::wasm_testing_utils::tracing_to_console_log::MemoryWriterFactory {
                buffer: ::std::sync::Arc::clone(&buffer),
            };

            let subscriber = ::tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .with_writer(writer_factory)
                .with_max_level(::tracing::Level::TRACE)
                .finish();

            ::tracing::subscriber::set_global_default(subscriber)
                .expect("Unable to set global subscriber");

            // The body of the function
            #fn_body

            let logged_data = buffer.lock().unwrap();
            let logged_data_str = String::from_utf8(logged_data.clone()).unwrap();
            ::wasm_bindgen_test::console_log!("{}", logged_data_str);
        }
    };

    TokenStream::from(expanded)
}
