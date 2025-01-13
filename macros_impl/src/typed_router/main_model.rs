use itertools::{Either, Itertools};
use std::{
    collections::{btree_set, BTreeSet},
    iter::once,
};

use proc_macro2::Span;
use syn::*;

use super::parsing;

#[derive(Debug)]
pub struct Parts {
    fallback: Ident,
    top_parts: Vec<Part>,
}

pub type Part = Either<NonParamPart, ParamPart>;

#[derive(Debug)]
pub struct PartInfo {
    path: String,
    path_span: Span,
    name: String,
    names_from_higher_levels: Vec<String>,
    view: Option<Ident>,
    arg_from_parent_type: Type,
    params_from_higher_levels: BTreeSet<Ident>,
    non_param_children: Vec<NonParamPart>,
    param_child: Option<Box<ParamPart>>,
    arg_to_children: Type,
    span: Span,
}

#[derive(derive_more::Deref, Debug)]
pub struct ParamPart(pub PartInfo);

#[derive(derive_more::Deref, Debug)]
pub struct NonParamPart(pub PartInfo);

impl PartInfo {
    fn len_sub_levels(&self) -> usize {
        self.non_param_children.len() + self.param_child.iter().count()
    }
}

fn from_parsing_route(
    parsing_part: parsing::Part,
    arg_from_parent_type: Type,
    names_from_higher_levels: Vec<String>,
    params_from_higher_levels: BTreeSet<Ident>,
) -> Result<Part> {
    let mut name = parsing_part.path.0.to_string();
    if name.len() == 0 {
        name = "empty".to_owned()
    }
    let names_from_higher_levels_to_children = names_from_higher_levels
        .iter()
        .cloned()
        .chain(once(name.clone()))
        .collect::<Vec<_>>();
    let mut params_from_higher_levels_to_children = params_from_higher_levels.clone();
    if let parsing::PathSegment::Param(p) = &parsing_part.path.0 {
        let param = Ident::new(p, parsing_part.path.1);

        match params_from_higher_levels_to_children.entry(param.clone()) {
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

    let arg_to_children = parsing_part.will_pass.unwrap_or(parse_quote!(()));
    let children = parsing_part
        .children
        .into_iter()
        .map(|child_part| {
            from_parsing_route(
                child_part,
                arg_to_children.clone(),
                names_from_higher_levels_to_children.clone(),
                params_from_higher_levels_to_children.clone(),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let duplicated_names = children
        .iter()
        .map(|c| c.name.to_lowercase())
        .sorted()
        .chunk_by(|n| n.clone())
        .into_iter()
        .filter_map(|(name, things)| if things.count() > 1 { Some(name) } else { None })
        .collect::<Vec<_>>();

    if duplicated_names.len() > 0 {
        return Err(Error::new(
            parsing_part.span,
            "Mulitiple children of this part have the same name/path.",
        ));
    }

    let (non_param_children, param_children): (Vec<_>, Vec<_>) =
        children.into_iter().partition_map(|p| p);

    let mut param_children = param_children.into_iter();
    let param_child = param_children.next().map(Box::new);
    if param_children.next().is_some() {
        return Err(Error::new(
            parsing_part.span,
            "This part has more than one parameterized child parts. That's not supported.",
        ));
    }

    let part_info = PartInfo {
        path: parsing_part.path.0.to_string(),
        name,
        names_from_higher_levels,
        path_span: parsing_part.path.1,
        view: parsing_part.view,
        arg_from_parent_type,
        params_from_higher_levels,
        non_param_children,
        param_child,
        arg_to_children,
        span: parsing_part.span,
    };

    Ok(match parsing_part.path.0 {
        parsing::PathSegment::Static(_) => Either::Left(NonParamPart(part_info)),
        parsing::PathSegment::Param(_) => Either::Right(ParamPart(part_info)),
    })
}

const ROOT: &str = "root";

impl TryFrom<parsing::Parts> for Parts {
    type Error = Error;

    fn try_from(value: parsing::Parts) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            fallback: value.fallback,
            top_parts: value
                .parts
                .into_iter()
                .map(|x| from_parsing_route(x, parse_quote!(()), vec![ROOT.into()], Default::default()))
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
