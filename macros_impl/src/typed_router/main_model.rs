use convert_case::{Case, Casing};
use itertools::Itertools;
use nutype::nutype;
use quote::ToTokens;
use std::{
    collections::{btree_set, BTreeSet},
    iter::once,
};

pub use part::Part;
use syn::*;

use super::parsing;

#[derive(Debug)]
pub struct Parts {
    pub fallback: Ident,
    pub top_parts: Vec<Part>,
}

#[nutype(
    derive(Clone, Debug, Deref, PartialEq, Ord, Eq, PartialOrd, Display, From),
    sanitize(with = |s: String| s.to_case(Case::Pascal))
)]
struct Pascal(String);

#[derive(Clone, Debug, Default, derive_more::Deref, Hash, PartialEq, Eq)]
pub struct ParamsSet(BTreeSet<Ident>);

impl ParamsSet {
    pub fn with_added(&self, param: Ident) -> Result<Self> {
        let mut clone = self.0.clone();
        let span = param.span();
        match clone.entry(param.clone()) {
            btree_set::Entry::Occupied(_) => {
                return Err(Error::new(
                    span,
                    format!(
                        "The route param `{}` is already defined at a higher level",
                        param
                    ),
                ))
            }
            btree_set::Entry::Vacant(entry) => entry.insert(),
        }
        Ok(Self(clone))
    }
}

mod part {
    use bon::{bon, builder};
    use proc_macro2::Span;
    use syn::*;

    use super::{ParamsSet, __nutype_Pascal__::Pascal};

    /// NOTE: This can be divided into fields that are used at the top level, and the
    /// fields that are not.
    /// RTI upheld: `param_sub_part` will have a `Part` that has `Some()`
    /// `param_at_this_level`.
    /// RTI upheld: `path` is non-empty if it has sub-parts.
    #[derive(Debug, Clone)]
    pub struct Part {
        path: String,
        path_span: Span,

        /// This will contain the prefixes from higher levels.
        name: Pascal,

        short_name: Pascal,

        view: Option<Ident>,
        arg_from_parent_type: Type,

        params_from_higher_levels: ParamsSet,

        non_param_sub_parts: Vec<Part>,

        param_sub_part: Option<Box<Part>>,

        arg_to_sub_parts: Type,
        span: Span,

        param_at_this_level: Option<Ident>,
    }

    #[bon]
    impl Part {
        #[builder]
        pub fn new(
            path: String,
            path_span: Span,
            name: Pascal,
            short_name: Pascal,
            view: Option<Ident>,
            arg_from_parent_type: Type,
            params_from_higher_levels: ParamsSet,
            non_param_sub_parts: Vec<Part>,
            param_sub_part: Option<Box<Part>>,
            arg_to_sub_parts: Type,
            span: Span,
            param_at_this_level: Option<Ident>,
        ) -> Self {
            let ret = Self {
                path,
                path_span,
                name,
                short_name,
                view,
                arg_from_parent_type,
                params_from_higher_levels,
                non_param_sub_parts,
                param_sub_part,
                arg_to_sub_parts,
                span,
                param_at_this_level,
            };

            if ret.path.is_empty() && ret.has_sub_parts() {
                panic!("The subparts of this `Part` should be 'hoisted up' one level higher because the path here is empty.")
            }

            if let Some(p) = &ret.param_sub_part {
                if p.param_at_this_level.is_none() {
                    panic!("A param sub-part has no param?")
                }
            }

            ret
        }
    }

    impl Part {
        pub fn has_sub_parts(&self) -> bool {
            self.count_sub_parts() > 0
        }

        pub fn count_sub_parts(&self) -> usize {
            self.non_param_sub_parts.len() + self.param_sub_part.iter().count()
        }
        pub fn path(&self) -> &str {
            &self.path
        }

        pub fn path_span(&self) -> Span {
            self.path_span
        }

        pub fn name(&self) -> &Pascal {
            &self.name
        }

        pub fn short_name(&self) -> &Pascal {
            &self.short_name
        }

        pub fn view(&self) -> Option<&Ident> {
            self.view.as_ref()
        }

        pub fn arg_from_parent_type(&self) -> &Type {
            &self.arg_from_parent_type
        }

        pub fn params_from_higher_levels(&self) -> &ParamsSet {
            &self.params_from_higher_levels
        }

        pub fn non_param_sub_parts(&self) -> &[Part] {
            &self.non_param_sub_parts
        }

        pub fn param_sub_part(&self) -> Option<&Box<Part>> {
            self.param_sub_part.as_ref()
        }

        pub fn arg_to_sub_parts(&self) -> &Type {
            &self.arg_to_sub_parts
        }

        pub fn span(&self) -> Span {
            self.span
        }

        pub fn param_at_this_level(&self) -> Option<&Ident> {
            self.param_at_this_level.as_ref()
        }
    }
}

fn from_parsing_route(
    parsing_part: parsing::Part,
    arg_from_parent_type: Type,
    names_from_higher_levels: Vec<String>,
    params_from_higher_levels: ParamsSet,
) -> Result<Part> {
    let mut short_name = parsing_part.path.0.to_string().to_case(Case::Pascal);
    if short_name.len() == 0 {
        short_name = "Empty".to_owned()
    }
    let names_from_higher_levels_to_sub_parts = names_from_higher_levels
        .iter()
        .cloned()
        .chain(once(short_name.clone()))
        .collect::<Vec<_>>();

    let param_at_this_level = if let parsing::PathSegment::Param(p) = &parsing_part.path.0 {
        Some(Ident::new(p, parsing_part.path.1))
    } else {
        None
    };

    let params_from_higher_levels_to_children = match param_at_this_level.clone() {
        Some(param) => params_from_higher_levels.with_added(param)?,
        None => params_from_higher_levels.clone(),
    };

    let arg_to_sub_parts = parsing_part.will_pass.unwrap_or(parse_quote!(()));
    let sub_parts = parsing_part
        .sub_parts
        .into_iter()
        .map(|sub_part| {
            from_parsing_route(
                sub_part,
                arg_to_sub_parts.clone(),
                names_from_higher_levels_to_sub_parts.clone(),
                params_from_higher_levels_to_children.clone(),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let duplicated_names = sub_parts
        .iter()
        .map(|c| c.short_name())
        .sorted()
        .chunk_by(|n| {
            #[allow(suspicious_double_ref_op)]
            n.clone()
        })
        .into_iter()
        .filter_map(|(name, things)| if things.count() > 1 { Some(name) } else { None })
        .collect::<Vec<_>>();

    if duplicated_names.len() > 0 {
        return Err(Error::new(
            parsing_part.span,
            format!(
            "Mulitiple sub-parts of this part have the same name/path. Duplicates names/paths are: {}", duplicated_names.iter().cloned().join(", ")),
        ));
    }

    let (param_sub_parts, non_param_sub_parts): (Vec<_>, Vec<_>) = sub_parts
        .into_iter()
        .partition(|p| p.param_at_this_level().is_some());

    let mut param_sub_parts = param_sub_parts.into_iter();
    let param_sub_part = param_sub_parts.next().map(Box::new);
    if param_sub_parts.next().is_some() {
        return Err(Error::new(
            parsing_part.span,
            "This part has more than one parameterized sub parts. That's not supported.",
        ));
    }

    let name = names_from_higher_levels
        .iter()
        .map(|n| n.to_case(Case::Pascal))
        .chain(once(short_name.clone()))
        .reduce(|a, b| a + &b)
        .expect("");

    let part = Part::builder()
        .path(parsing_part.path.0.to_string())
        .path_span(parsing_part.path.1)
        .name(name.into())
        .short_name(short_name.into())
        .maybe_view(parsing_part.view)
        .arg_from_parent_type(arg_from_parent_type)
        .params_from_higher_levels(params_from_higher_levels)
        .non_param_sub_parts(non_param_sub_parts)
        .maybe_param_sub_part(param_sub_part)
        .arg_to_sub_parts(arg_to_sub_parts)
        .span(parsing_part.span)
        .maybe_param_at_this_level(param_at_this_level)
        .build();

    Ok(part)
}

pub const ROOT: &str = "Root";

impl TryFrom<parsing::Parts> for Parts {
    type Error = Error;

    fn try_from(value: parsing::Parts) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            fallback: value.fallback,
            top_parts: value
                .parts
                .into_iter()
                .map(|x| {
                    from_parsing_route(x, parse_quote!(()), vec![ROOT.into()], Default::default())
                })
                .collect::<Result<_>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::typed_router::TEST_STR;

    use super::*;

    #[test]
    fn test_converting_to_main_model_parts() -> Result<()> {
        let parsed: parsing::Parts = parse_str(TEST_STR).expect("Unable to parse routes");
        let main_model_parts = Parts::try_from(parsed)?;
        println!("{:#?}", main_model_parts);
        Ok(())
    }
}
