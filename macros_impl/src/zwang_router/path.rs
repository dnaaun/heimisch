use quote::quote;
use std::{iter::once, str::FromStr};

use convert_case::Casing;
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use syn::*;

use super::routes::main_model::Pascal;

pub fn zwang_url(input: LitStr) -> proc_macro2::TokenStream {
    let url = input.value();

    if let Some('/') = url.chars().next() {
    } else {
        return syn::Error::new(Span::call_site(), "Must start with a slash (`/`).")
            .to_compile_error();
    }

    let path_end = url.find(&['?', '#'][..]).unwrap_or(url.len());

    let parts: Result<Vec<_>> = url[..path_end]
        .split('/')
        .skip(1) // Skip the leading empty entry from leading '/'
        .map(|part_str| {
            let part_str = if part_str.is_empty() {
                "Empty"
            } else {
                part_str
            };
            let equals_idx = part_str.find("=");
            Ok(if let Some(equals_idx) = equals_idx {
                let name = part_str[..equals_idx].into();
                let remaining = &part_str[(equals_idx + 1)..];

                let value = if remaining.len() >= 2
                    && remaining.starts_with('{')
                    && remaining.ends_with('}')
                {
                    let expr = &part_str[(equals_idx + 2)..(part_str.len() - 1)];
                    let expr = TokenStream::from_str(expr);
                    match expr {
                        Ok(e) => e,
                        Err(_) => {
                            return Err(syn::Error::new(
                                Span::call_site(),
                                format!("Invalid expression for `{}`.", name),
                            ))
                        }
                    }
                } else {
                    quote! { #remaining }
                };
                Part::Dynamic { name, value }
            } else {
                Part::Static(part_str.into())
            })
        })
        .collect();

    let parts = match parts {
        Ok(p) => p,
        Err(e) => return e.to_compile_error(),
    };

    let rest = url[path_end..].to_string();

    let typed_path = construct_typed_path(&parts, 0);

    quote! {
        ::zwang_router::PathAndRest {
            path: #typed_path,
            rest: #rest.into()
        }
    }
}

enum Part {
    Static(Pascal),
    Dynamic { name: Pascal, value: TokenStream },
}

impl Part {
    fn name(&self) -> Pascal {
        match self {
            Part::Static(s) => s.clone(),
            Part::Dynamic { name, value: _ } => name.clone(),
        }
    }
}

/// `parts` should not include the `Root` part.
fn construct_typed_path(parts: &[Part], cur_idx: usize) -> proc_macro2::TokenStream {
    debug_assert!(!parts.is_empty());
    debug_assert!(cur_idx <= parts.len());

    let enum_name = Ident::new(
        &once("Root".into())
            .chain(parts[..(cur_idx)].iter().map(Part::name))
            .join(""),
        Span::call_site(),
    );

    if cur_idx == parts.len() {
        quote! { #enum_name::Empty }
    } else {
        let variant_part = &parts[cur_idx];
        let variant_name = variant_part.name();
        let variant_name = Ident::new(&variant_name, Span::call_site());
        let next_construction = construct_typed_path(parts, cur_idx + 1);
        match variant_part {
            Part::Static(_) => {
                quote! { #enum_name::#variant_name(#next_construction) }
            }
            Part::Dynamic { name, value } => {
                let var_name =
                    Ident::new(&name.to_case(convert_case::Case::Snake), Span::call_site());
                quote! {
                    #enum_name::#variant_name {
                        #var_name: #value.into(),
                        child: #next_construction
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zwang_url;

    #[test]
    fn basic_url_construction() {
        let expr: syn::LitStr = parse_quote! { "/owner_name=hi/repo_name=hello/issues/new" };
        let url = zwang_url(expr);
        println!("{}", url);
    }
}
