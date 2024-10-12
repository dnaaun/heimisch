use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField};
use proc_macro2::Span;
use quote::{quote, IdentFragment};
use syn::{self, parse_macro_input, DeriveInput, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(idb), supports(struct_named))]
struct IdbRepositoryOpts {
    ident: syn::Ident,
    data: darling::ast::Data<(), IdbField>,
}

#[derive(FromField)]
#[darling(attributes(idb))]
struct IdbField {
    ident: Option<syn::Ident>,
    id: Option<bool>,
    ty: syn::Type,
    index: Option<bool>,
}

pub fn derive_typesafe_idb(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = IdbRepositoryOpts::from_derive_input(&input).expect("Invalid input");

    let struct_name = &opts.ident;
    let struct_name_lowercase_pluralized = Ident::new(
        &(struct_name.to_string().to_lowercase() + "s"),
        struct_name.span(),
    );
    let store_marker_name = quote::format_ident!("{}StoreMarker", struct_name);

    let fields: Vec<_> = match opts.data {
        darling::ast::Data::Struct(ref fields) => fields.fields.iter().collect(),
        _ => panic!("IdbRepository can only be derived for structs"),
    };

    let id_field = fields
        .iter()
        .find(|f| f.id.is_some())
        .expect("There must be exactly one `#[idb(id)]` field");

    let id_field_name = id_field.ident.as_ref().expect("Fields must be named");
    let id_field_type = &id_field.ty;

    let index_structs: Vec<_> = fields
        .iter()
        .filter(|f| f.index.is_some())
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap().to_string();
            let field_type = &f.ty;
            let index_name = Ident::new(
                &(field_name.to_case(Case::Pascal) + "Index"),
                field_name.span().unwrap_or(Span::call_site()),
            );

            quote! {
                pub struct #index_name { }

                impl typesafe_idb::IndexSpec for #index_name {
                    type Store = #struct_name;
                    const NAME: &'static str = #field_name;
                    type Type = #field_type;
                }
            }
        })
        .collect();

    let index_adds: Vec<_> = fields
        .iter()
        .filter(|f| f.index.is_some())
        .map(|f| {
            let field_name_str = f.ident.as_ref().unwrap().to_string();
            quote! {
                .add_index(idb::builder::IndexBuilder::new(
                    #field_name_str.into(),
                    idb::KeyPath::Single(#field_name_str.into()),
                ))
            }
        })
        .collect();

    let output = quote! {
        pub struct #store_marker_name {}

        impl typesafe_idb::StoreMarker<#struct_name> for #store_marker_name {}
        impl typesafe_idb::Store for #struct_name {
            const NAME: &'static str = stringify!(#struct_name_lowercase_pluralized);
            type Marker = #store_marker_name;
            type Id = #id_field_type;

            fn id(&self) -> &Self::Id {
                &self.#id_field_name
            }

            fn object_store_builder() -> idb::builder::ObjectStoreBuilder {
                idb::builder::ObjectStoreBuilder::new(Self::NAME)
                    .key_path(Some(idb::KeyPath::new_single(stringify!(#id_field_name))))
                    #(#index_adds)*
            }
        }
        #(#index_structs)*
    };

    output.into()
}
