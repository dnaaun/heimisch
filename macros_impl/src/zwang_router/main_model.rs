use convert_case::{Case, Casing};
use itertools::Itertools;
use nutype::nutype;
use part::NonLeafDetails;
use proc_macro2::Span;
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
    pub top_part: Part,
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
    // use bon::{bon, builder};
    use proc_macro2::Span;
    use syn::*;

    use super::{ParamsSet, __nutype_Pascal__::Pascal};

    /// RTI upheld: at least one sub part.
    #[derive(Debug, Clone)]
    pub struct NonLeafDetails {
        pub layout: Option<Ident>,
        pub non_param_sub_parts: Vec<Part>,
        pub param_sub_part: Option<Box<Part>>,
        pub arg_to_sub_parts: Type,
        pub is_root_level: bool,
    }

    #[derive(Debug, Clone)]
    pub struct Part {
        pub path: String,
        pub path_span: Span,
        /// This will contain the prefixes from higher levels.
        pub name: Pascal,
        pub short_name: Pascal,
        pub view: Option<Ident>,
        pub arg_from_parent_type: Type,
        pub params_from_higher_levels: ParamsSet,
        pub span: Span,
        pub param_at_this_level: Option<Ident>,
        pub non_leaf_details: Option<NonLeafDetails>,
    }
}

pub fn from_parsing_route(
    parsing_part: parsing::Part,
    arg_from_parent_type: Type,
    names_from_higher_levels: Vec<String>,
    params_from_higher_levels: ParamsSet,
    is_root_level: bool,
) -> Result<Part> {
    if !is_root_level && parsing_part.fallback.is_some() {
        return Err(Error::new(
            parsing_part.span,
            "`fallback` should be specified only at top level.",
        ));
    }
    let path = if is_root_level {
        parsing_part
            .path
            .unwrap_or((parsing::PathSegment::Static("".into()), Span::call_site()))
    } else {
        match parsing_part.path {
            Some(p) => {
                // if p.0.is_empty() {
                // return Err(Error::new(
                //     parsing_part.span,
                //     "Empty `path` not allowed anywhere except the top level specified.",
                // ));
                // } else {
                p
                // }
            }
            None => {
                (parsing::PathSegment::Static("".into()), Span::call_site())
                // return Err(Error::new(
                //     parsing_part.span,
                //     "`path` can be ommitted only at the top-level.",
                // ))
            }
        }
    };
    let short_name = if is_root_level {
        "Root".into()
    } else {
        if path.0.is_empty() {
            "Empty".to_owned()
        } else {
            path.0.to_string().to_case(Case::Pascal)
        }
    };

    let names_from_higher_levels_to_sub_parts = names_from_higher_levels
        .iter()
        .cloned()
        .chain(once(short_name.clone()))
        .collect::<Vec<_>>();

    let param_at_this_level = if let parsing::PathSegment::Param(p) = &path.0 {
        Some(Ident::new(p, path.1))
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
                false,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let duplicated_names = sub_parts
        .iter()
        .map(|c| c.short_name.clone())
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
        .partition(|p| p.param_at_this_level.is_some());

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

    let non_leaf_details = if non_param_sub_parts.len() + param_sub_part.iter().count() > 0 {
        Some(NonLeafDetails {
            layout: parsing_part.layout,
            non_param_sub_parts,
            param_sub_part,
            arg_to_sub_parts,
            is_root_level,
        })
    } else {
        None
    };
    let part = Part {
        path: path.0.to_string(),
        path_span: path.1,
        name: name.into(),
        short_name: short_name.into(),
        view: parsing_part.view,
        arg_from_parent_type,
        params_from_higher_levels,
        non_leaf_details,
        span: parsing_part.span,
        param_at_this_level,
    };

    Ok(part)
}

pub const ROOT: &str = "Root";

fn process_to_add_empty_leaf_parts_when_necessary(
    part: parsing::Part,
    parent_will_pass: Option<Type>,
) -> parsing::Part {
    if part.sub_parts.len() > 0 {
        parsing::Part {
            sub_parts: part
                .sub_parts
                .into_iter()
                .map(|x| process_to_add_empty_leaf_parts_when_necessary(x, part.will_pass.clone()))
                .collect(),
            ..part
        }
    } else {
        // If the leaf node is already empty, no need to add anything here.
        let leaf_is_empty = match &part.path {
            None => true,
            Some(p) if p.0.len() == 0 => true,
            _ => false,
        };
        if leaf_is_empty {
            part
        } else {
            let new_empty_part = parsing::Part {
                path: Some((parsing::PathSegment::Static("".into()), part.span)),
                will_pass: None,
                ..part
            };

            parsing::Part {
                path: part.path,
                view: None,
                fallback: None,
                layout: None,
                sub_parts: vec![new_empty_part],
                will_pass: parent_will_pass,
                span: part.span,
            }
        }
    }
}

impl TryFrom<parsing::Part> for Parts {
    type Error = Error;

    fn try_from(value: parsing::Part) -> std::result::Result<Self, Self::Error> {
        let fallback = value.fallback.clone();
        let value = process_to_add_empty_leaf_parts_when_necessary(value, None);
        Ok(Self {
            fallback: fallback.ok_or(Error::new(
                value.span,
                "`fallback` expected at the top level.",
            ))?,
            top_part: from_parsing_route(
                value,
                parse_quote!(()),
                vec![],
                Default::default(),
                true,
            )?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::typed_router::TEST_STR;

    use super::*;

    #[test]
    fn test_converting_to_main_model_parts() -> Result<()> {
        let parsed: parsing::Part = parse_str(TEST_STR).expect("Unable to parse routes");
        let main_model_parts = Parts::try_from(parsed)?;
        println!("{:#?}", main_model_parts);
        Ok(())
    }
}
