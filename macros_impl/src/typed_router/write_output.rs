use std::iter::once;

use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::*;

use super::main_model::{self, ROOT};

pub fn write_output(parts: main_model::Parts) -> Result<TokenStream> {
    let (param_sub_parts, non_param_sub_parts): (Vec<_>, Vec<_>) =
        parts.top_parts.into_iter().partition(|p| p.is_param_itself);

    let mut param_sub_parts = param_sub_parts.into_iter();
    let param_sub_part = param_sub_parts.next().map(Box::new);
    if param_sub_parts.next().is_some() {
        return Err(Error::new(
            Span::call_site(),
            "top level contains more than one parameterized part.",
        ));
    }

    let root_part = main_model::Part {
        name: ROOT.into(),
        short_name: ROOT.into(),

        non_param_sub_parts,
        param_sub_part,

        arg_to_sub_parts: parse_quote!(()),
        span: Span::call_site(),
        path_span: Span::call_site(),

        // Won't (or really, _shouldn't_) be used. I add the parenthetical because it's definitely
        // possible to enforce this by restructing the code such that "invalid states are
        // unrepresentable." But it ain't worth it.
        path: Default::default(),
        view: Default::default(),
        arg_from_parent_type: parse_quote!(()),
        params_from_higher_levels: Default::default(),
        is_param_itself: false,
    };

    let all_parts_output = flatten_parts(Some(root_part))
        .iter()
        .filter(|x| x.len_sub_parts() > 0)
        .map(|x| {
            [write_part_defn(x), write_from_slashed_impl(x)]
                .into_iter()
                .collect::<TokenStream>()
        })
        .collect_vec();
    Ok(quote! {
        #(#all_parts_output)*
    })
}

/// NOTE: There's a lot of cloning here, and it feels like that shouldn't be necessary.
fn flatten_parts(parts: impl IntoIterator<Item = main_model::Part>) -> Vec<main_model::Part> {
    parts
        .into_iter()
        .map(|part| {
            let non_param_flat = flatten_parts(part.non_param_sub_parts.clone());
            let param_flat = flatten_parts(part.param_sub_part.clone().map(|x| *x));
            once(part).chain(non_param_flat).chain(param_flat)
        })
        .flatten()
        .collect()
}

trait PartExt {
    fn as_variant_ident(&self) -> Ident;
    fn as_ident(&self) -> Ident;
    fn as_param_name_ident(&self) -> Option<Ident>;
    fn as_path_part_literal(&self) -> Literal;
}
impl PartExt for main_model::Part {
    fn as_variant_ident(&self) -> Ident {
        Ident::new(&self.short_name, self.path_span)
    }

    fn as_ident(&self) -> Ident {
        Ident::new(&self.name, self.path_span)
    }

    /// Will be None if is_param_itself is false
    fn as_param_name_ident(&self) -> Option<Ident> {
        if self.is_param_itself {
            Some(Ident::new(
                &self.short_name.to_case(Case::Snake),
                self.path_span,
            ))
        } else {
            None
        }
    }

    fn as_path_part_literal(&self) -> Literal {
        let mut path_part = Literal::string(&self.path);
        path_part.set_span(self.path_span);
        path_part
    }
}

fn write_part_defn(part: &main_model::Part) -> TokenStream {
    debug_assert!(part.len_sub_parts() > 0);

    let ident = part.as_ident();

    let mut variants = part
        .non_param_sub_parts
        .iter()
        .map(|non_param_child| {
            let variant_ident = non_param_child.as_variant_ident();
            let field_ident = non_param_child.as_ident();

            if non_param_child.len_sub_parts() > 0 {
                quote! { #variant_ident(#field_ident) }
            } else {
                quote! { #variant_ident }
            }
        })
        .collect_vec();

    if let Some(param_sub_part) = &part.param_sub_part {
        let variant_ident = param_sub_part.as_variant_ident();
        let param_name_ident = param_sub_part.as_param_name_ident();
        let child_field = if param_sub_part.len_sub_parts() > 0 {
            let child_field_type_ident = param_sub_part.as_ident();
            Some(quote! { child: #child_field_type_ident, })
        } else {
            None
        };

        variants.push(quote! { #variant_ident {
            #param_name_ident: String,
            #child_field
        }
        });
    }

    quote! {
        pub enum #ident {
            #(#variants),*
        }
    }
}

fn write_from_slashed_impl(part: &main_model::Part) -> TokenStream {
    let mut matches = part
        .non_param_sub_parts
        .iter()
        .map(|sub_part| {
            let path_part = sub_part.as_path_part_literal();
            let variant_name_ident = sub_part.as_variant_ident();

            if sub_part.len_sub_parts() == 0 {
                quote! {
                    #path_part => {
                        ::zwang_router::NoPart::try_from(tail)?;
                        Ok(Self::#variant_name_ident)
                    }
                }
            } else {
                quote! {
                    #path_part => Ok(Self::#variant_name_ident(tail.try_into()?)),
                }
            }
        })
        .collect_vec();

    matches.push(match &part.param_sub_part {
        Some(sub_part) => {
            let variant_name_ident = sub_part.as_variant_ident();
            let param_name_ident = sub_part.as_param_name_ident();

            let child_field = if sub_part.len_sub_parts() > 0 {
                Some(quote! { child: tail.try_into()? })
            } else {
                None
            };

            quote! {
                #param_name_ident @ _ => Ok(Self::#variant_name_ident {
                #param_name_ident: #param_name_ident.to_owned(),
                #child_field
            })
            }
        }
        None => {
            quote! {
                other => {
                    Err(format!("Unrecognized path segment: '{}'", other))
                }
            }
        }
    });

    let ident = part.as_ident();

    quote! {
        impl<'a> TryFrom<::zwang_router::Slashed<'a>> for #ident {
            type Error = String;

            fn try_from(value: ::zwang_router::Slashed<'a>) -> std::result::Result<Self, Self::Error> {
                let (head, tail) = ::zwang_router::split_slashed(value);

                match head.non_slash() {
                    #(#matches)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use main_model::Parts;

    use crate::typed_router::{parsing, TEST_STR};

    use super::*;

    #[test]
    fn test_writing_output() -> Result<()> {
        let parsed: parsing::Parts = parse_str(TEST_STR).expect("Unable to parse routes");
        let main_model_parts = Parts::try_from(parsed)?;
        let output = write_output(main_model_parts).unwrap().to_string();
        println!(
            "\n\n{}\n\n",
            prettyplease::unparse(&syn::parse_file(&output).unwrap())
        );
        Ok(())
    }
}
