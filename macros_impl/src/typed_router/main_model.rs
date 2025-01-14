use convert_case::{Case, Casing};
use itertools::Itertools;
use nutype::nutype;
use std::{
    collections::{btree_set, BTreeSet},
    iter::once,
};

use proc_macro2::Span;
use syn::*;

use super::parsing;

#[derive(Debug)]
pub struct Parts {
    pub fallback: Ident,
    pub top_parts: Vec<Part>,
}

// impl Deref for Part {
//     type Target = PartInfo;
//
//     fn deref(&self) -> &Self::Target {
//         match &self {
//             Part::NonParam(part_info) => part_info,
//             Part::Param(part_info) => part_info,
//         }
//     }
// }

#[nutype(
    derive(Clone, Debug, Deref, PartialEq, Ord, Eq, PartialOrd, Display, From),
    sanitize(with = |s: String| s.to_case(Case::Pascal))
)]
struct Pascal(String);

/// NOTE: This can be divided into fields that are used at the top level, and the fields that are
/// not.
#[derive(Debug, Clone)]
pub struct Part {
    pub path: String,
    pub path_span: Span,

    /// This will contain the prefixes from higher levels.
    pub name: Pascal,

    pub short_name: Pascal,

    pub view: Option<Ident>,
    pub arg_from_parent_type: Type,

    pub params_from_higher_levels: BTreeSet<Ident>,

    pub non_param_sub_parts: Vec<Part>,

    pub param_sub_part: Option<Box<Part>>,

    pub arg_to_sub_parts: Type,
    pub span: Span,

    pub is_param_itself: bool,
}

// #[derive(derive_more::From, derive_more::Deref, Debug, Clone)]
// pub struct ParamPart(pub PartInfo);
//
// #[derive(derive_more::From, derive_more::Deref, Debug, Clone)]
// pub struct NonParamPart(pub PartInfo);

impl Part {
    pub fn has_sub_parts(&self) -> bool {
        self.non_param_sub_parts.len() + self.param_sub_part.iter().count() > 0
    }
}

fn from_parsing_route(
    parsing_part: parsing::Part,
    arg_from_parent_type: Type,
    names_from_higher_levels: Vec<String>,
    params_from_higher_levels: BTreeSet<Ident>,
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

    let mut params_from_higher_levels_to_sub_parts = params_from_higher_levels.clone();
    if let parsing::PathSegment::Param(p) = &parsing_part.path.0 {
        let param = Ident::new(p, parsing_part.path.1);

        match params_from_higher_levels_to_sub_parts.entry(param.clone()) {
            btree_set::Entry::Occupied(_) => {
                return Err(Error::new(
                    param.span(),
                    format!(
                        "The route param `{}` is already defined at a higher level",
                        param
                    ),
                ))
            }
            btree_set::Entry::Vacant(entry) => entry.insert(),
        }
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
                params_from_higher_levels_to_sub_parts.clone(),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let duplicated_names = sub_parts
        .iter()
        .map(|c| &c.short_name)
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

    let (param_sub_parts, non_param_sub_parts): (Vec<_>, Vec<_>) =
        sub_parts.into_iter().partition(|p| p.is_param_itself);

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


    let part = Part {
        path: parsing_part.path.0.to_string(),
        path_span: parsing_part.path.1,
        name: name.into(),
        short_name: short_name.into(),
        view: parsing_part.view,
        arg_from_parent_type,
        params_from_higher_levels,
        non_param_sub_parts,
        param_sub_part,
        arg_to_sub_parts,
        span: parsing_part.span,
        is_param_itself: match parsing_part.path.0 {
            parsing::PathSegment::Static(_) => false,
            parsing::PathSegment::Param(_) => true,
        },
    };

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
