use anyhow::{anyhow, Result};
use itertools::Itertools;
use ordered_float::NotNan;
use std::collections::HashSet;
use std::iter::once;
use std::{collections::HashMap, ops::Deref};

use convert_case::{Case, Casing};

use super::{t1, t2};

pub fn t1_top_level_to_t2(
    t1_top_level: &t1::TopLevelWebhookSchema,
    literal_string_union_store: &mut HashMap<t2::TypeRef<t2::LiteralStringUnionInner>, u32>,
    object_store: &mut HashMap<t2::TypeRef<t2::ObjectInner>, u32>,
) -> Option<t2::Type> {
    let avail_to_app = t1_top_level
        .availability
        .iter()
        .any(|a| matches!(a, t1::Availability::App));

    if avail_to_app {
        let prefix = t1_top_level.category.to_case(Case::Pascal)
            + &t1_top_level.action.to_case(Case::Pascal);
        Some(
            t1_to_t2(
                &t1::WebhookSchema {
                    r#type: t1::ParameterType::Object,
                    name: String::new(),
                    description: Some(t1_top_level.description_html.clone()),
                    is_required: Some(true.into()),
                    child_params_groups: Some(t1_top_level.body_parameters.to_vec()),
                    ..Default::default()
                },
                &[prefix],
                literal_string_union_store,
                object_store,
            )
            .expect("Errors that bubble to this level evidently couldn't be handled"),
        )
    } else {
        eprintln!(
            "Skipping {}/{} because it's not a webhook available to Github Apps.",
            t1_top_level.category, t1_top_level.action
        );
        None
    }
}

fn t1_to_t2(
    t1_schema: impl Deref<Target = t1::WebhookSchema>,
    current_name_path: &[String],
    literal_string_union_store: &mut HashMap<t2::TypeRef<t2::LiteralStringUnionInner>, u32>,
    object_store: &mut HashMap<t2::TypeRef<t2::ObjectInner>, u32>,
) -> Result<t2::Type> {
    let t1_schema = t1_schema.deref();
    Ok(match t1_schema.r#type {
        t1::ParameterType::Null => {
            check_no_enum_values(t1_schema)?;
            check_no_default(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let meta = t2::Meta {
                title: Some(make_name(current_name_path, &t1_schema.name)),
                description: t1_schema.description.clone(),
                is_required: t1_schema
                    .is_required
                    .as_ref()
                    .map(bool::from)
                    .unwrap_or(false),
            };

            t2::Type {
                inner: t2::TypeInner::Null,
                meta,
            }
        }
        t1::ParameterType::String => {
            check_no_child_params_groups(t1_schema)?;
            let (inner, meta) = if let Some(enum_values) = &t1_schema.enum_values {
                // In this way, we avoid having an indirection of having a json-schema definition
                // defining the constant string, which prevents producing `#[serde(tag=something)]`
                // enums, and instead everything becomes #[serde(untagged)]`.
                if enum_values.len() == 1 {
                    let (string, meta) = parse_literal_string(t1_schema, current_name_path)?;
                    (t2::TypeInner::LiteralString(string), meta)
                } else {
                    let (string_literal_union_ref, meta) = parse_literal_string_union(
                        t1_schema,
                        false,
                        current_name_path,
                        literal_string_union_store,
                    )?;

                    (t2::TypeInner::from(string_literal_union_ref), meta)
                }
            } else {
                let (string, meta) = parse_string(t1_schema, current_name_path)?;
                (t2::TypeInner::from(string), meta)
            };
            t2::Type { inner, meta }
        }
        t1::ParameterType::Integer => {
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;
            let (integer, meta) = parse_integer(t1_schema, current_name_path)?;
            t2::Type {
                inner: integer.into(),
                meta,
            }
        }
        t1::ParameterType::Number => {
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;
            let (float, meta) = parse_float(t1_schema, current_name_path)?;
            t2::Type {
                inner: float.into(),
                meta,
            }
        }
        t1::ParameterType::Boolean => {
            check_no_child_params_groups(t1_schema)?;
            let (boolean, meta) = parse_boolean(t1_schema, current_name_path)?;
            if let Some(enum_values) = &t1_schema.enum_values {
                let enum_values = enum_values
                    .iter()
                    .map(|i| match i {
                        Some(i) => Ok(i),
                        None => Err(anyhow!(
                            "found null in enum_values despite type being boolean"
                        )),
                    })
                    .collect::<Result<Vec<_>>>()?;

                let enum_values = enum_values
                    .clone()
                    .into_iter()
                    .map(|i| {
                        if let t1::T1Enum::Bool(b) = i {
                            Ok(b)
                        } else {
                            Err(anyhow!(
                                "Non string in enum values of string: {t1_schema:?}"
                            ))
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;
                if enum_values.len() != 1 {
                    return Err(anyhow!("Expecting one value in enum_values since type is bool, but found: {t1_schema:#?}"));
                } else {
                    t2::Type {
                        inner: t2::TypeInner::LiteralBool(**enum_values.first().unwrap()),
                        meta,
                    }
                }
            } else {
                t2::Type {
                    inner: boolean.into(),
                    meta,
                }
            }
        }
        t1::ParameterType::Array => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;
            let (array, meta) = parse_array(t1_schema, current_name_path)?;
            t2::Type {
                inner: array.into(),
                meta,
            }
        }
        t1::ParameterType::Object => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            let (object, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;
            t2::Type {
                inner: object.into(),
                meta,
            }
        }
        t1::ParameterType::IntegerOrNull => {
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;
            let (integer, meta) = parse_integer(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(integer.into())),
                meta,
            }
        }
        t1::ParameterType::NullOrObject | t1::ParameterType::ObjectOrNull => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            let (object, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;
            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(object.into())),
                meta,
            }
        }
        t1::ParameterType::StringOrNull => {
            check_no_child_params_groups(t1_schema)?;
            let (inner, meta) = match &t1_schema.enum_values {
                Some(_) => {
                    let (string_literal_union, meta) = parse_literal_string_union(
                        t1_schema,
                        true,
                        current_name_path,
                        literal_string_union_store,
                    )?;
                    (t2::TypeInner::from(string_literal_union), meta)
                }
                None => {
                    let (string, meta) = parse_string(t1_schema, current_name_path)?;
                    (t2::TypeInner::from(string), meta)
                }
            };

            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(inner)),
                meta,
            }
        }
        t1::ParameterType::IntegerOrString => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;
            let (string, meta) = parse_string(t1_schema, current_name_path)?;
            let (integer, _) = parse_integer(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::TypeInner::Union(t2::UnionInner {
                    variants: vec![string.into(), integer.into()],
                }),
                meta,
            }
        }
        t1::ParameterType::StringOrObject | t1::ParameterType::ObjectOrString => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (object, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;

            let (string, _) = parse_string(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::UnionInner {
                    variants: vec![object.into(), string.into()],
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::StringOrNumber => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;
            let (float, _) = parse_float(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::UnionInner {
                    variants: vec![float.into(), string.into()],
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::NumberOrNull => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (float, meta) = parse_float(t1_schema, current_name_path)?;

            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(float.into())),
                meta,
            }
        }
        t1::ParameterType::BooleanOrNull => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (boolean, meta) = parse_boolean(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(boolean.into())),
                meta,
            }
        }
        t1::ParameterType::ArrayOfObjects => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (members, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;
            t2::Type {
                inner: t2::ArrayInner {
                    members: Box::new(members.into()),
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::ArrayOfStrings => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::ArrayInner {
                    members: Box::new(string.into()),
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::NullOrStringOrArray => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;
            let array = t2::ArrayInner {
                members: Box::new(t2::TypeInner::Unspecified),
            };

            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(
                    t2::UnionInner {
                        variants: vec![string.into(), array.into()],
                    }
                    .into(),
                )),
                meta,
            }
        }
        t1::ParameterType::NullOrIntegerOrString => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;

            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(
                    t2::UnionInner {
                        variants: vec![string.into(), t2::IntegerInner { default: None }.into()],
                    }
                    .into(),
                )),
                meta,
            }
        }
        t1::ParameterType::ArrayOfObjectNulls => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (object, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;
            t2::Type {
                inner: t2::ArrayInner {
                    members: Box::new(t2::TypeInner::Nullable(Box::new(object.into()))),
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::ArrayOfStringNulls => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::ArrayInner {
                    members: Box::new(t2::TypeInner::Nullable(Box::new(string.into()))),
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::NullOrStringOrObject => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (object, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;

            let (string, _) = parse_string(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(
                    t2::UnionInner {
                        variants: vec![object.into(), string.into()],
                    }
                    .into(),
                )),
                meta,
            }
        }
        t1::ParameterType::ArrayOfObjectsOrNull => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (object, meta) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;

            let array_inner = t2::ArrayInner {
                members: Box::new(object.into()),
            };

            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(array_inner.into())),
                meta,
            }
        }
        t1::ParameterType::ArrayOfStringsOrNull => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            check_no_child_params_groups(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;

            let array_inner = t2::ArrayInner {
                members: Box::new(string.into()),
            };

            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(array_inner.into())),
                meta,
            }
        }
        t1::ParameterType::BooleanOrStringOrIntegerOrObject => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;
            let (boolean, meta) = parse_boolean(t1_schema, current_name_path)?;
            let (string, _) = parse_string(t1_schema, current_name_path)?;
            let (integer, _) = parse_integer(t1_schema, current_name_path)?;
            let (object, _) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;
            t2::Type {
                inner: t2::UnionInner {
                    variants: [boolean.into(), string.into(), integer.into(), object.into()]
                        .into_iter()
                        .collect(),
                }
                .into(),
                meta,
            }
        }
        t1::ParameterType::NullOrStringOrObjectOrInteger => {
            check_no_default(t1_schema)?;
            check_no_enum_values(t1_schema)?;

            let (string, meta) = parse_string(t1_schema, current_name_path)?;
            let (object, _) = parse_object(
                t1_schema,
                current_name_path,
                literal_string_union_store,
                object_store,
            )?;
            let (integer, _) = parse_integer(t1_schema, current_name_path)?;
            t2::Type {
                inner: t2::TypeInner::Nullable(Box::new(
                    t2::UnionInner {
                        variants: [string.into(), integer.into(), object.into()]
                            .into_iter()
                            .collect(),
                    }
                    .into(),
                )),
                meta,
            }
        }
    })
}

fn make_name(current_name_path: &[String], name: &String) -> String {
    let mut parts = current_name_path.iter();
    let new_part = name.to_case(Case::Pascal);
    if current_name_path.is_empty() {
        name.clone()
    } else if current_name_path[current_name_path.len() - 1] != new_part {
        parts.chain(once(&new_part)).join("")
    } else {
        parts.join("")
    }
}

pub fn check_no_enum_values(t1_schema: &t1::WebhookSchema) -> Result<()> {
    match t1_schema.enum_values.iter().flatten().next() {
        Some(_) => Err(anyhow!(
            "for a string type, enum_values shouldn't be specified (or be empty): {t1_schema:?}"
        )),

        None => Ok(()),
    }
}

pub fn check_no_child_params_groups(t1_schema: &t1::WebhookSchema) -> Result<()> {
    match t1_schema.child_params_groups.iter().flatten().next() {
        Some(_) => Err(anyhow!(
            "child_params_group shouldn't be present: {t1_schema:?}"
        )),

        None => Ok(()),
    }
}

pub fn check_no_default(t1_schema: &t1::WebhookSchema) -> Result<()> {
    match t1_schema.default {
        Some(_) => Err(anyhow!("default shouldn't be present: {t1_schema:?}")),
        None => Ok(()),
    }
}

/// t2::TypeInner will be StringInner, or t2::LiteralStringUnionInner only.
pub fn parse_string(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
) -> Result<(t2::StringInner, t2::Meta)> {
    let t1::WebhookSchema {
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        enum_values: t1_enum_values,
        default: t1_default,
        ..
    } = t1_schema;
    let description = t1_description.clone();
    let default = parse_default_for_string_or_string_literal_union(t1_default, t1_schema).unwrap();

    let type_inner = match t1_enum_values {
        Some(_) => {
            return Err(anyhow!(
                "for a string type, enum_values shouldnt' be specified: {t1_schema:?}"
            ))
        }
        None => t2::StringInner { default },
    };
    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);

    Ok((
        type_inner,
        t2::Meta {
            title: make_name(current_name_path, t1_name).into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_literal_string(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
) -> Result<(String, t2::Meta)> {
    let t1::WebhookSchema {
        name: _t1_name,
        description: t1_description,
        is_required: t1_is_required,
        enum_values: t1_enum_values,
        ..
    } = t1_schema;
    let description = t1_description.clone();

    let string = match t1_enum_values {
        Some(members) => {
            let members: Vec<_> = members
                .clone()
                .into_iter()
                .map(|s| match s {
                    Some(s) => Ok(s),
                    None => Err(anyhow!("null value found in enum_values of a non-nullable")),
                })
                .collect::<Result<Vec<_>>>()?;

            let members = members
                .clone()
                .into_iter()
                .map(|p| {
                    if let t1::T1Enum::String(s) = p {
                        Ok(s)
                    } else {
                        Err(anyhow!(
                            "Non string in enum values of string: {t1_schema:?}"
                        ))
                    }
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .collect::<HashSet<_>>();

            if members.len() != 1 {
                return Err(anyhow!("Expected exactly 1 enum value"));
            }
            members.into_iter().next().unwrap()
        }
        None => {
            return Err(anyhow!(
                "enum_values is specified, but array is empty: {t1_schema:?}"
            ))
        }
    };

    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);

    Ok((
        string,
        t2::Meta {
            title: current_name_path.join("").into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_literal_string_union(
    t1_schema: &t1::WebhookSchema,
    allow_null_in_union: bool,
    current_name_path: &[String],
    literal_string_union_store: &mut HashMap<t2::TypeRef<t2::LiteralStringUnionInner>, u32>,
) -> Result<(t2::TypeRef<t2::LiteralStringUnionInner>, t2::Meta)> {
    let t1::WebhookSchema {
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        enum_values: t1_enum_values,
        default: t1_default,
        ..
    } = t1_schema;
    let description = t1_description.clone();
    let default = parse_default_for_string_or_string_literal_union(t1_default, t1_schema)?;

    let type_inner = match t1_enum_values {
        Some(members) => {
            let members: Vec<_> = if allow_null_in_union {
                members.clone().into_iter().flatten().collect()
            } else {
                members
                    .clone()
                    .into_iter()
                    .map(|s| match s {
                        Some(s) => Ok(s),
                        None => Err(anyhow!("null value found in enum_values of a non-nullable")),
                    })
                    .collect::<Result<Vec<_>>>()?
            };

            let members = members
                .clone()
                .into_iter()
                .map(|p| {
                    if let t1::T1Enum::String(s) = p {
                        Ok(s)
                    } else {
                        Err(anyhow!(
                            "Non string in enum values of string: {t1_schema:?}"
                        ))
                    }
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .collect::<HashSet<_>>();
            let default = default.map(|d| {
                        if !members.contains(&d) {
                            return Err(anyhow!("Default value for string literal union is not in the string literal union: {t1_schema:?}"));
                        };
                        Ok(d)
                    }).transpose()?;
            t2::LiteralStringUnionInner { members, default }
        }
        None => {
            return Err(anyhow!(
                "enum_values is specified, but array is empty: {t1_schema:?}"
            ))
        }
    };

    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);

    let ref_name = make_name(current_name_path, t1_name);
    let ref_ =
        t2::TypeRef::new_or_incr_count(type_inner, ref_name.clone(), literal_string_union_store);

    Ok((
        ref_,
        t2::Meta {
            title: ref_name.into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_integer(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
) -> Result<(t2::IntegerInner, t2::Meta)> {
    let t1::WebhookSchema {
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        default: t1_default,
        ..
    } = t1_schema;
    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);
    let description = t1_description.clone();
    let default = parse_default_for_integer(t1_default, t1_schema).unwrap();
    Ok((
        t2::IntegerInner { default },
        t2::Meta {
            title: make_name(current_name_path, t1_name).into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_float(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
) -> Result<(t2::FloatInner, t2::Meta)> {
    let t1::WebhookSchema {
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        default: t1_default,
        ..
    } = t1_schema;
    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);
    let description = t1_description.clone();
    let default = parse_default_for_float(t1_default, t1_schema).unwrap();
    Ok((
        t2::FloatInner { default },
        t2::Meta {
            title: make_name(current_name_path, t1_name).into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_array(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
) -> Result<(t2::ArrayInner, t2::Meta)> {
    let t1::WebhookSchema {
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        ..
    } = t1_schema;
    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);
    let description = t1_description.clone();

    Ok((
        t2::ArrayInner {
            members: Box::new(t2::TypeInner::Unspecified),
        },
        t2::Meta {
            title: make_name(current_name_path, t1_name).into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_object(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
    literal_string_union_store: &mut HashMap<t2::TypeRef<t2::LiteralStringUnionInner>, u32>,
    object_store: &mut HashMap<t2::TypeRef<t2::ObjectInner>, u32>,
) -> Result<(t2::TypeRef<t2::ObjectInner>, t2::Meta)> {
    let t1::WebhookSchema {
        child_params_groups: t1_child_params_groups,
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        ..
    } = t1_schema;
    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);
    let description = t1_description.clone();

    let members = parse_members_for_object(
        t1_child_params_groups,
        current_name_path,
        literal_string_union_store,
        object_store,
    )?;

    let object_inner = t2::ObjectInner { members };
    let ref_name = make_name(current_name_path, t1_name);
    let ref_ = t2::TypeRef::new_or_incr_count(object_inner, ref_name.clone(), object_store);
    Ok((
        ref_,
        t2::Meta {
            title: ref_name.into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_boolean(
    t1_schema: &t1::WebhookSchema,
    current_name_path: &[String],
) -> Result<(t2::BooleanInner, t2::Meta)> {
    let t1::WebhookSchema {
        child_params_groups: t1_child_params_groups,
        name: t1_name,
        description: t1_description,
        is_required: t1_is_required,
        default: t1_default,
        ..
    } = t1_schema;
    let is_required = t1_is_required.as_ref().map(bool::from).unwrap_or(false);
    let description = t1_description.clone();
    if t1_child_params_groups.is_some() {
        return Err(anyhow!(
            "child_params_groups should be null for a bool: {t1_schema:?}"
        ));
    }
    let default = parse_default_for_boolean(t1_default, t1_schema).unwrap();

    Ok((
        t2::BooleanInner { default },
        t2::Meta {
            title: make_name(current_name_path, t1_name).into(),
            description,
            is_required,
        },
    ))
}

pub fn parse_members_for_object(
    t1_child_params_groups: &Option<Vec<t1::WebhookSchema>>,
    current_name_path: &[String],
    literal_string_union_store: &mut HashMap<t2::TypeRef<t2::LiteralStringUnionInner>, u32>,
    object_store: &mut HashMap<t2::TypeRef<t2::ObjectInner>, u32>,
) -> Result<Vec<t2::ObjectMember>> {
    match t1_child_params_groups {
        Some(t1_child_params_groups) => t1_child_params_groups
            .iter()
            .map(|t1_child_params_group| {
                let key = &t1_child_params_group.name;
                Ok(t2::ObjectMember {
                    key: key.clone(),
                    value: t1_to_t2(
                        t1_child_params_group,
                        &current_name_path
                            .iter()
                            .cloned()
                            .chain([key.to_case(Case::Pascal)])
                            .collect::<Vec<_>>(),
                        literal_string_union_store,
                        object_store,
                    )?,
                })
            })
            .collect::<Result<Vec<_>>>(),
        None => Ok(vec![]),
    }
}

pub fn parse_default_for_string_or_string_literal_union(
    t1_default: &Option<serde_json::Value>,
    t1_schema: &t1::WebhookSchema,
) -> Result<Option<String>> {
    t1_default
        .as_ref()
        .map(|default| match &default {
            serde_json::Value::String(default) => Ok(default.clone()),
            _ => Err(anyhow!(
                "Non-string default ({default:?}) for a string type: {t1_schema:?}"
            )),
        })
        .transpose()
}

pub fn parse_default_for_float(
    t1_default: &Option<serde_json::Value>,
    t1_schema: &t1::WebhookSchema,
) -> Result<Option<NotNan<f64>>> {
    t1_default
        .as_ref()
        .map(|t1_default| match t1_default {
            serde_json::Value::Number(number) => match number.as_f64() {
                Some(number) => Ok(NotNan::new(number)?),
                None => Err(anyhow!(
                    "Default number doesn't fit into an f64: {t1_schema:?}"
                )),
            },
            _ => Err(anyhow!("non number default for number type: {t1_schema:?}")),
        })
        .transpose()
}

pub fn parse_default_for_integer(
    t1_default: &Option<serde_json::Value>,
    t1_schema: &t1::WebhookSchema,
) -> Result<Option<i64>> {
    t1_default
        .as_ref()
        .map(|t1_default| match t1_default {
            serde_json::Value::Number(number) => match number.as_i64() {
                Some(number) => Ok(number),
                None => Err(anyhow!(
                    "Default number doesn't fit into an i64: {t1_schema:?}"
                )),
            },
            _ => Err(anyhow!("non number default for number type: {t1_schema:?}")),
        })
        .transpose()
}

pub fn parse_default_for_boolean(
    t1_default: &Option<serde_json::Value>,
    t1_schema: &t1::WebhookSchema,
) -> Result<Option<bool>> {
    t1_default
        .as_ref()
        .map(|t1_default| match t1_default {
            serde_json::Value::Bool(value) => Ok(*value),
            _ => Err(anyhow!("non number default for number type: {t1_schema:?}")),
        })
        .transpose()
}

/// Allow this function to be unused in case we need it in the future.
#[allow(unused)]
pub fn parse_default_for_array(
    t1_default: &Option<serde_json::Value>,
    t1_schema: &t1::WebhookSchema,
) -> Result<Option<Vec<serde_json::Value>>> {
    t1_default
        .as_ref()
        .map(|t1_default| match t1_default {
            serde_json::Value::Array(arr) => Ok(arr.clone()),
            _ => Err(anyhow!("non array default for array type: {t1_schema:?}")),
        })
        .transpose()
}

/// Allow this function to be unused in case we need it in the future.
#[allow(unused)]
pub fn parse_default_for_object(
    t1_default: &Option<serde_json::Value>,
    t1_schema: &t1::WebhookSchema,
) -> Result<Option<serde_json::Map<String, serde_json::Value>>> {
    t1_default
        .as_ref()
        .map(|t1_default| match t1_default {
            serde_json::Value::Object(default) => Ok(default.clone()),
            _ => Err(anyhow!("Non object default for object: {t1_schema:?}")),
        })
        .transpose()
}
