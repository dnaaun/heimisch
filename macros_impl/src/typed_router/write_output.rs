use std::{collections::HashSet, iter::once};

use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::*;

use super::main_model::{self};

pub fn write_output(parts: main_model::Parts) -> Result<TokenStream> {
    let (all_parts, all_params) = flatten_parts_and_params(Some(parts.top_part))?;
    let all_parts_output = all_parts
        .iter()
        .filter(|x| x.non_leaf_details.is_some())
        .map(|x| {
            Ok([
                write_part_defn(x),
                write_from_slashed_impl(x),
                write_display_impl(x),
                write_only_struct(x),
                write_get_only_impl(x),
                write_route_to_view_impl(x)?,
            ]
            .into_iter()
            .collect::<TokenStream>())
        })
        .collect::<Result<Vec<_>>>()?;

    let all_params_output = all_params
        .iter()
        .sorted_by_key(|a| a.len())
        .map(write_params_struct);

    let non_varying_stuff_written = write_non_varying_stuff();
    Ok(quote! {
        #non_varying_stuff_written
        #(#all_parts_output)*
        #(#all_params_output)*
    })
}

/// NOTE: There's a lot of cloning here, and it feels like that shouldn't be necessary.
fn flatten_parts_and_params(
    parts: impl IntoIterator<Item = main_model::Part>,
) -> Result<(Vec<main_model::Part>, HashSet<main_model::ParamsSet>)> {
    let (parts, params): (Vec<_>, Vec<_>) = parts
        .into_iter()
        .map(|part| {
            let (non_param_parts, params1) =
                if let Some(non_leaf_details) = part.non_leaf_details.as_ref() {
                    flatten_parts_and_params(non_leaf_details.non_param_sub_parts.to_vec())?
                } else {
                    (Default::default(), Default::default())
                };
            let (param_parts, params2) = if let Some(Some(p)) = part
                .non_leaf_details
                .as_ref()
                .map(|n| n.param_sub_part.clone())
            {
                flatten_parts_and_params(Some(*p))?
            } else {
                (Default::default(), Default::default())
            };
            let available_params = part.params_available_at_this_level()?;
            Ok((
                once(part).chain(non_param_parts).chain(param_parts),
                params1
                    .union(&params2)
                    .cloned()
                    .chain(if available_params.len() > 0 {
                        Some(available_params)
                    } else {
                        None
                    })
                    .collect::<HashSet<_>>(),
            ))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip();
    Ok((
        parts.into_iter().flatten().collect(),
        params
            .into_iter()
            .fold(Default::default(), |a, b| a.union(&b).cloned().collect()),
    ))
}

trait PartExt {
    fn as_variant_ident(&self) -> Ident;
    fn as_ident(&self) -> Ident;
    fn as_path_part_literal(&self) -> Literal;
    fn as_only_ident(&self) -> Ident;
    fn params_available_at_this_level(&self) -> Result<main_model::ParamsSet>;
    fn as_param_var_name_ident(&self) -> Option<Ident>;
    fn as_child_memo_var_name_ident(&self) -> Ident;

    fn should_create_empty_variant(&self) -> bool;
}

impl PartExt for main_model::Part {
    fn as_variant_ident(&self) -> Ident {
        Ident::new(&self.short_name, self.path_span)
    }

    fn as_ident(&self) -> Ident {
        Ident::new(&self.name, self.path_span)
    }

    fn as_path_part_literal(&self) -> Literal {
        let mut path_part = Literal::string(&self.path);
        path_part.set_span(self.path_span);
        path_part
    }

    fn as_only_ident(&self) -> Ident {
        Ident::new(&((**self.name).to_owned() + "Only"), self.path_span)
    }

    fn params_available_at_this_level(&self) -> Result<main_model::ParamsSet> {
        Ok(if let Some(param) = &self.param_at_this_level {
            self.params_from_higher_levels.with_added(param.clone())?
        } else {
            self.params_from_higher_levels.clone()
        })
    }

    fn as_param_var_name_ident(&self) -> Option<Ident> {
        self.param_at_this_level
            .as_ref()
            .map(|p| Ident::new(&format!("param_memo_{p}"), p.span()))
    }

    fn as_child_memo_var_name_ident(&self) -> Ident {
        Ident::new(
            &format!("child_memo_{}", self.short_name.to_case(Case::Snake)),
            self.path_span,
        )
    }

    fn should_create_empty_variant(&self) -> bool {
        self.non_leaf_details.is_some() && self.view.is_some()
    }
}

trait ParamsSetExt {
    fn as_ident(&self) -> Ident;
}

impl ParamsSetExt for main_model::ParamsSet {
    fn as_ident(&self) -> Ident {
        Ident::new(
            &("Params".to_owned()
                + &self
                    .iter()
                    .map(|p| p.to_string().to_case(Case::Pascal))
                    .reduce(|a, b| a + &b)
                    .expect("Unexpected empty set of params")),
            Span::call_site(),
        )
    }
}

fn write_part_defn(part: &main_model::Part) -> TokenStream {
    let non_leaf_details = part.non_leaf_details.as_ref().expect("");

    let ident = part.as_ident();

    let mut variants = non_leaf_details
        .non_param_sub_parts
        .iter()
        .map(|non_param_child| {
            let variant_ident = non_param_child.as_variant_ident();
            let field_ident = non_param_child.as_ident();

            if non_param_child.non_leaf_details.is_some() {
                quote! { #variant_ident(#field_ident) }
            } else {
                quote! { #variant_ident }
            }
        })
        .collect_vec();

    if let Some(param_sub_part) = &non_leaf_details.param_sub_part {
        let variant_ident = param_sub_part.as_variant_ident();
        let param_name_ident = param_sub_part.param_at_this_level.clone().expect("");
        let child_field = if param_sub_part.non_leaf_details.is_some() {
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

    if part.should_create_empty_variant() {
        variants.push(quote! { Empty })
    }

    quote! {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum #ident {
            #(#variants),*
        }
    }
}

fn write_from_slashed_impl(part: &main_model::Part) -> TokenStream {
    let non_leaf_details = part.non_leaf_details.as_ref().expect("");
    let mut matches = non_leaf_details
        .non_param_sub_parts
        .iter()
        .map(|sub_part| {
            let path_part = sub_part.as_path_part_literal();
            let variant_name_ident = sub_part.as_variant_ident();

            if !sub_part.non_leaf_details.is_some() {
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

    if part.should_create_empty_variant() {
        matches.push(quote! {
            "" => {
                ::zwang_router::NoPart::try_from(tail)?;
                Ok(Self::Empty)
            }
        })
    }

    matches.push(match &non_leaf_details.param_sub_part {
        Some(sub_part) => {
            let variant_name_ident = sub_part.as_variant_ident();
            let param_name_ident = sub_part.param_at_this_level.as_ref();

            let child_field = if sub_part.non_leaf_details.is_some() {
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

fn write_display_impl(part: &main_model::Part) -> TokenStream {
    let non_leaf_details = part.non_leaf_details.as_ref().expect("");
    let mut matches = non_leaf_details
        .non_param_sub_parts
        .iter()
        .map(|p| {
            let variant_ident = p.as_variant_ident();
            if p.non_leaf_details.is_some() {
                let mut interpolated_str_literal = Literal::string(&format!("/{}{{}}", p.path));
                interpolated_str_literal.set_span(p.path_span);
                quote! {
                Self::#variant_ident(child) => {
                    write!(f, #interpolated_str_literal, child)
                }

                }
            } else {
                let mut interpolated_str_literal = Literal::string(&format!("/{}", p.path));
                interpolated_str_literal.set_span(p.path_span);
                quote! {
                    Self::#variant_ident => {
                        write!(f, #interpolated_str_literal)
                    }
                }
            }
        })
        .collect_vec();

    if part.should_create_empty_variant() {
        matches.push(quote! {
            Self::Empty => {
                write!(f, "/")
            }
        })
    }

    if let Some(p) = &non_leaf_details.param_sub_part {
        let variant_ident = p.as_variant_ident();
        let param_name_ident = p.param_at_this_level.as_ref().expect("");
        matches.push(if p.non_leaf_details.is_some() {
            quote! {
                Self::#variant_ident { #param_name_ident, child } => {
                    write!(f, "/{}{}", #param_name_ident, child)
                }
            }
        } else {
            quote! {
                    Self::#variant_ident { #param_name_ident } => {
                    write!(f, "/{}", #param_name_ident)
                }

            }
        });
    }
    let ident = part.as_ident();

    quote! {
            impl std::fmt::Display for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#matches)*
                    }
                }
            }
    }
}

fn write_only_struct(part: &main_model::Part) -> TokenStream {
    let non_leaf_details = part.non_leaf_details.as_ref().expect("");
    let mut variants = non_leaf_details
        .non_param_sub_parts
        .iter()
        .map(|p| p.as_variant_ident())
        .collect_vec();
    if let Some(p) = &non_leaf_details.param_sub_part {
        variants.push(p.as_variant_ident())
    }
    if part.should_create_empty_variant() {
        variants.push(Ident::new("Empty", part.span))
    }

    let only_ident = Ident::new(&((**part.name).to_owned() + "Only"), part.path_span);

    quote! {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        enum #only_ident {
            #(#variants),*
        }
    }
}

fn write_get_only_impl(part: &main_model::Part) -> TokenStream {
    let non_leaf_details = part.non_leaf_details.as_ref().expect("");
    let only_ident = part.as_only_ident();
    let mut variants = non_leaf_details
        .non_param_sub_parts
        .iter()
        .map(|p| {
            let variant_ident = p.as_variant_ident();
            if p.non_leaf_details.is_some() {
                quote! { Self::#variant_ident(..) => #only_ident::#variant_ident }
            } else {
                quote! { Self::#variant_ident => #only_ident::#variant_ident }
            }
        })
        .collect_vec();
    if let Some(p) = &non_leaf_details.param_sub_part {
        let variant_ident = p.as_variant_ident();

        variants.push(quote! { Self::#variant_ident { .. } => #only_ident::#variant_ident })
    }

    if part.should_create_empty_variant() {
        variants.push(quote! { Self::Empty => #only_ident::Empty })
    }

    let ident = part.as_ident();

    quote! {
        impl #ident {
            fn get_only(&self) -> #only_ident {
                match self {
                    #(#variants),*
                }
            }
        }
    }
}

fn write_params_struct(params: &main_model::ParamsSet) -> TokenStream {
    let ident = params.as_ident();
    let fields = params
        .iter()
        .map(|p| quote! { pub #p: ::leptos::prelude::Memo<String> });
    quote! {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct #ident {
            #(#fields),*
        }
    }
}

fn write_non_varying_stuff() -> TokenStream {
    quote! {
        trait RouteToView {
            type PrevParams: Sync + Send + 'static;
            type ArgFromParent;
            fn render(
                self,
                arg_from_parent: Self::ArgFromParent,
                prev_params: Self::PrevParams,
            ) -> impl ::leptos::prelude::IntoView;
        }

        impl RouteToView for ::leptos::prelude::Memo<Result<Root, String>> {
            type PrevParams = ();

            type ArgFromParent = ();

            fn render(
                self,
                arg_from_parent: Self::ArgFromParent,
                prev_params: Self::PrevParams,
            ) -> impl ::leptos::prelude::IntoView {
                let ok_memo =
                    ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).ok());
                let this_part_only = ::leptos::prelude::Memo::new(move |_| {
                    ::leptos::prelude::Get::get(&self).map(|i| i.get_only())
                });
                move || {
                    let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I onlt don't .track(), this sometimes,
                                                                          // doesn't work.
                    match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                        Ok(_) => ::leptos::prelude::IntoAny::into_any(
                            ::leptos::prelude::Memo::new(move |_| {
                                ::leptos::prelude::Get::get(&ok_memo).unwrap()
                            })
                            .render(arg_from_parent, prev_params),
                        ),
                        Err(_) => {
                            ::leptos::prelude::IntoAny::into_any(::leptos::prelude::view! { <NotFound /> })
                        }
                    }
                }
            }
        }

        #[::leptos::prelude::component]
        pub fn Routed() -> impl ::leptos::prelude::IntoView {
            let pathname = ::zwang_router::use_pathname();
            let root = ::leptos::prelude::Memo::new(move |_| {
                let pathname = ::leptos::prelude::Get::get(&pathname);
                let slashed = ::zwang_router::Slashed::new(&pathname)
                    .expect("pathname doesn't start with a slash is weird");
                Root::try_from(slashed)
            });
            ::leptos::prelude::provide_context(::zwang_router::ParsedPath(root));
            root.render((), ())
        }
    }
}

fn write_route_to_view_impl(part: &main_model::Part) -> Result<TokenStream> {
    let non_leaf_details = part.non_leaf_details.as_ref().expect("");
    let ident = part.as_ident();
    let catch_all = if (non_leaf_details.param_sub_part.iter().count()
        + non_leaf_details.non_param_sub_parts.len())
        + if part.should_create_empty_variant() {
            1
        } else {
            0
        }
        > 1
    {
        Some(quote! { _ => None, })
    } else {
        None
    };
    let param_memo_written = non_leaf_details.param_sub_part.as_ref().map(|sub_part| {
        let param_ident = sub_part.param_at_this_level.as_ref().expect("");
        let var_name_ident = sub_part.as_param_var_name_ident().expect("");
        let variant_ident = sub_part.as_variant_ident();

        quote! {
        let #var_name_ident =
            ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                #ident::#variant_ident { #param_ident, .. } => Some(#param_ident),
                #catch_all
            });

        }
    });

    let mut child_memos_written = non_leaf_details
        .non_param_sub_parts
        .iter()
        .filter(|p| p.non_leaf_details.is_some())
        .map(|p| {
            let child_memo_var_name_ident = p.as_child_memo_var_name_ident();
            let variant_ident = p.as_variant_ident();
            quote! {
                let #child_memo_var_name_ident =
                    ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                        #ident::#variant_ident(child) => Some(child),
                        #catch_all
                    });
            }
        })
        .collect_vec();

    if let Some(Some(p)) = &part
        .non_leaf_details
        .as_ref()
        .map(|p| p.param_sub_part.clone())
    {
        if p.non_leaf_details.is_some() {
            let child_memo_var_name_ident = p.as_child_memo_var_name_ident();
            let variant_ident = p.as_variant_ident();
            child_memos_written.push(quote! {
                let #child_memo_var_name_ident =
                    ::leptos::prelude::Memo::new(move |_| match ::leptos::prelude::Get::get(&self) {
                        #ident::#variant_ident { child, .. } => Some(child),
                        #catch_all
                    });
            });
        }
    }

    let mut variants = part.non_leaf_details.iter().map(|n| n. non_param_sub_parts.clone()).flatten()
        .map(|p| {
            let variant_ident = p.as_variant_ident();
            Ok(match &p.non_leaf_details {
                None => {
                let view_ident = match &p.view {
                    Some(i) => i.clone(),
                    None => return Err(Error::new(p.span, "This leaf path has no associated view.")),
                };
                quote! {
                    #ident::#variant_ident => {
                        let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
                    > =
                            std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                        let params = prev_params;
                        let info = ::zwang_router::RoutingInfoForComponent {
                            arg_from_parent,
                            outlet,
                            params,
                        };

                        ::leptos::prelude::IntoAny::into_any(
                            ::zwang_router::RoutableComponent::into_view_with_route_info(#view_ident, info),
                        )
                    }
                }
            } ,

            Some(non_leaf_details) => {
                let layout_ident = non_leaf_details.layout.as_ref().map(|x| x.to_token_stream()).unwrap_or_else( || {
                    parse_quote! {::zwang_router::passthrough_component }
                    });
                let child_memo_var_name_ident = p.as_child_memo_var_name_ident();
                quote! {
                    #ident::#variant_ident(_) => {
                        let child_memo = ::zwang_router::MemoExt::unwrap(
                            #child_memo_var_name_ident
                        );

                        let outlet: ::std::sync::Arc<
                            dyn core::ops::Fn(_) -> _
                                + ::core::marker::Send
                                + ::core::marker::Sync,
                        > = ::std::sync::Arc::new(move |arg_from_parent| {
                            child_memo.render(arg_from_parent, prev_params)
                        });
                        let params = prev_params;

                        let info = ::zwang_router::RoutingInfoForComponent {
                            arg_from_parent,
                            outlet,
                            params,
                        };

                        ::leptos::prelude::IntoAny::into_any(
                            ::zwang_router::RoutableComponent::into_view_with_route_info(
                                #layout_ident,
                                info,
                            ),
                        )
                    }
                }
            }
            })
        })
    .collect::<Result<Vec<_>>>()?;

    if part.should_create_empty_variant() {
        let view_ident = part
            .view
            .as_ref()
            .expect("should_create_empty_variant() check ensures this is Some().");
        variants.push(quote! {
            #ident::Empty => {
                let outlet: ::std::sync::Arc<
                dyn core::ops::Fn(_) -> _ + ::core::marker::Send + ::core::marker::Sync,
            > =
                    std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                let params = prev_params;
                let info = ::zwang_router::RoutingInfoForComponent {
                    arg_from_parent,
                    outlet,
                    params,
                };

                ::leptos::prelude::IntoAny::into_any(
                    ::zwang_router::RoutableComponent::into_view_with_route_info(#view_ident, info),
                )
            }
        })
    }

    if let Some(Some(p)) = &part
        .non_leaf_details
        .as_ref()
        .map(|n| n.param_sub_part.clone())
    {
        let variant_ident = p.as_variant_ident();
        let params_construction = write_param_struct_construction_at_sub_part(&p);
        variants.push(
        match &p.non_leaf_details {
            None => {
            let view_ident = match &p.view {
                Some(i) => i.clone(),
                None => return Err(Error::new(p.span, "This leaf path has no associated view.")),
            };
            quote! {
                #ident::#variant_ident { .. } => {
                    #params_construction
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + core::marker::Send + core::marker::Sync
                    > =
                        ::std::sync::Arc::new(::zwang_router::empty_component::<Self::ArgFromParent>);
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };
                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            #view_ident, info,
                        ),
                    )
                }
            }
        } 
            Some(non_leaf_details) => {
            let layout_ident = non_leaf_details.layout.as_ref().map(|x| x.to_token_stream()).unwrap_or(parse_quote!(
                ::zwang_router::passthrough_component
            ));
            let child_memo_var_name_ident = p.as_child_memo_var_name_ident();
            quote! {
                #ident::#variant_ident { .. } => {
                    #params_construction
                    let child_memo =
                        ::zwang_router::MemoExt::unwrap(#child_memo_var_name_ident);
                    let outlet: ::std::sync::Arc<
                        dyn core::ops::Fn(_) -> _ + core::marker::Send + core::marker::Sync
                    > = std::sync::Arc::new(move |arg_from_parent| {
                        child_memo.render(arg_from_parent, params)
                    });
                    let info = ::zwang_router::RoutingInfoForComponent {
                        arg_from_parent,
                        outlet,
                        params,
                    };

                    ::leptos::prelude::IntoAny::into_any(
                        ::zwang_router::RoutableComponent::into_view_with_route_info(
                            #layout_ident,
                            info,
                        ),
                    )
                }
            }
        }});
    }

    let params_available_at_this_level = part.params_available_at_this_level()?;
    let prev_params_type = if params_available_at_this_level.len() > 0 {
        params_available_at_this_level.as_ident().to_token_stream()
    } else {
        parse_quote!(())
    };
    let arg_from_parent_type = 
        non_leaf_details
        .arg_to_sub_parts.clone();

    Ok(quote! {
        impl RouteToView for ::leptos::prelude::Memo<#ident> {
            type PrevParams = #prev_params_type;
            type ArgFromParent = #arg_from_parent_type;

            fn render(
                self,
                arg_from_parent: Self::ArgFromParent,
                prev_params: Self::PrevParams,
            ) -> impl ::leptos::prelude::IntoView {
                #param_memo_written
                #(#child_memos_written);*

                let this_part_only =
                    ::leptos::prelude::Memo::new(move |_| ::leptos::prelude::Get::get(&self).get_only());

                move || {
                    let _ = ::leptos::prelude::Get::get(&this_part_only); // Very weirdly, if I .track(), this sometimes doesn't
                                                                          // work.
                    match *::leptos::prelude::ReadUntracked::read_untracked(&self) {
                        #(#variants)*
                    }
                }
            }
        }
    })
}

fn write_param_struct_construction_at_sub_part(sub_part: &main_model::Part) -> TokenStream {
    let new_param = sub_part.param_at_this_level.as_ref().expect("");

    let mut assigns = sub_part
        .params_from_higher_levels
        .iter()
        .map(|p| {
            quote! { #p: prev_params.#p }
        })
        .collect_vec();

    let new_param_value_memo = sub_part.as_param_var_name_ident().expect("");
    assigns.push(quote! {
        #new_param: ::zwang_router::MemoExt::unwrap(#new_param_value_memo),
    });

    let params_struct_ident = sub_part
        .params_available_at_this_level()
        .expect("")
        .as_ident();

    quote! {
            let params = #params_struct_ident {
                #(#assigns),*
            };

    }
}

#[cfg(test)]
mod tests {
    use main_model::Parts;

    use crate::typed_router::{parsing, TEST_STR};

    use super::*;

    #[test]
    fn test_writing_output() -> Result<()> {
        let parsed: parsing::Part = parse_str(TEST_STR).expect("Unable to parse routes");
        let main_model_parts = Parts::try_from(parsed)?;
        let output = write_output(main_model_parts).unwrap().to_string();
        // println!("\n\n{output}\n\n");
        println!(
            "\n\n{}\n\n",
            prettyplease::unparse(&syn::parse_file(&output).unwrap())
        );
        Ok(())
    }
}
