use itertools::multiunzip;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn derive_avail_merge(input: proc_macro::TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    // Extract the name of the struct
    let name = &input.ident;

    // Check the input is a struct with named fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Merge can only be derived for structs with named fields"),
        },
        _ => panic!("Merge can only be derived for structs"),
    };

    // Generate the merge implementation for each field
    let (checks, with_merged_fields, merge_fields): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(
        fields.iter().map(|field| {
        let field_name = &field.ident;
        // Check if the field has the `#[id]` attribute
        let is_not_avail_field = match &field.ty {
            syn::Type::Path(type_path) => type_path.path.segments.last().map(|segment|
                segment.ident != "Avail"
            ).unwrap_or(true),
            _ => true
        };


        if is_not_avail_field {
            (
                Some(quote! {
                if self.#field_name != other.#field_name {
                    return Err(crate::avail::MergeError::NonMatchingAttr(stringify!(#field_name)));
                };
            }),
            Some(quote! { #field_name: other.#field_name }),
            Some(quote! { self.#field_name = other.#field_name; }),
            )
        } else {
            (
                None,
            Some(quote! { #field_name: self.#field_name.with_merged(other.#field_name)? }),
            Some(quote! { self.#field_name.merge(other.#field_name)?; }),
            )
        }
    })
    .collect::<Vec<_>>()
    );

    let checks = checks.into_iter().flatten().collect::<Vec<_>>();

    // Expand into a complete implementation
    let expanded = quote! {
        impl #name {
            pub fn with_merged(self, other: Self) -> std::result::Result<Self, crate::avail::MergeError> {
                #(#checks)*

                Ok(Self {
                    #(#with_merged_fields),*
                })
            }

            pub fn merge(&mut self, other: Self) -> std::result::Result<(), crate::avail::MergeError> {
                #(#checks)*

                #(#merge_fields)*

                Ok(())
            }
        }
    };

    expanded
}
