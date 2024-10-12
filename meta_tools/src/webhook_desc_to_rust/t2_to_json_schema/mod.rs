use super::t2;
use itertools::Itertools;
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject,
    SubschemaValidation,
};
use std::collections::{BTreeMap, BTreeSet};
use t2_to_schema_context::T2ToSchemaContext;

pub mod t2_to_schema_context;

pub fn t2_to_schema_object(t2_type: t2::Type, context: &mut T2ToSchemaContext) -> SchemaObject {
    let t2::Type { meta, inner } = t2_type;
    let default_metadata: Metadata = Metadata {
        title: meta.title,
        description: meta.description,
        ..Default::default()
    };
    let schema_object = match inner {
        t2::TypeInner::Null => SchemaObject {
            metadata: Some(Box::new(default_metadata)),
            instance_type: Some(InstanceType::Null.into()),
            ..Default::default()
        },
        t2::TypeInner::Unspecified => SchemaObject {
            metadata: Some(Box::new(default_metadata)),
            ..Default::default()
        },
        t2::TypeInner::String(string_inner) => SchemaObject {
            metadata: Some(Box::new(Metadata {
                default: string_inner.default.map(serde_json::Value::from),
                ..default_metadata
            })),
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        },
        t2::TypeInner::Integer(integer_inner) => SchemaObject {
            metadata: Some(Box::new(Metadata {
                default: integer_inner.default.map(serde_json::Value::from),
                ..default_metadata
            })),
            instance_type: Some(InstanceType::Integer.into()),
            ..Default::default()
        },
        t2::TypeInner::Float(float_inner) => SchemaObject {
            metadata: Some(Box::new(Metadata {
                default: float_inner
                    .default
                    .clone()
                    .map(|f| serde_json::Value::from(f.into_inner())),
                ..default_metadata
            })),
            instance_type: Some(InstanceType::Number.into()),
            ..Default::default()
        },
        t2::TypeInner::Boolean(boolean_inner) => SchemaObject {
            metadata: Some(Box::new(Metadata {
                default: boolean_inner.default.map(serde_json::Value::from),
                ..default_metadata
            })),
            instance_type: Some(InstanceType::Boolean.into()),
            ..Default::default()
        },
        t2::TypeInner::LiteralBool(literal_bool) => SchemaObject {
            metadata: Some(Box::new(default_metadata)),
            instance_type: Some(InstanceType::Boolean.into()),
            const_value: Some(literal_bool.into()),
            ..Default::default()
        },
        t2::TypeInner::LiteralString(literal_string) => SchemaObject {
            metadata: Some(Box::new(default_metadata)),
            instance_type: Some(InstanceType::String.into()),
            const_value: Some(literal_string.into()),
            ..Default::default()
        },
        t2::TypeInner::Nullable(type_inner) => {
            let subschema = t2_to_schema_object(
                t2::Type {
                    inner: *type_inner,
                    meta: Default::default(),
                },
                context,
            );
            let mut schema = make_schema_nullable(subschema);
            schema.metadata = Some(Box::new(default_metadata));
            schema
        }
        t2::TypeInner::Union(t2::UnionInner { variants }) => {
            let schemas = variants
                .iter()
                .map(|variant| {
                    Schema::Object(t2_to_schema_object(
                        t2::Type {
                            inner: *variant.clone(),
                            meta: Default::default(),
                        },
                        context,
                    ))
                })
                .collect_vec();

            SchemaObject {
                subschemas: Some(Box::new(SubschemaValidation {
                    any_of: Some(schemas),
                    ..Default::default()
                })),
                metadata: Some(Box::new(default_metadata)),
                ..Default::default()
            }
        }
        t2::TypeInner::LiteralStringUnionRef(literal_type_ref) => context
            .schema_object_for_literal_string_union_ref(literal_type_ref, |r| {
                t2_literal_string_union_to_schema(r, default_metadata)
            }),

        t2::TypeInner::Array(t2::ArrayInner { members }) => {
            let subschema = t2_to_schema_object(
                t2::Type {
                    inner: *members,
                    meta: Default::default(),
                },
                context,
            );

            SchemaObject {
                metadata: Some(Box::new(default_metadata)),
                instance_type: Some(InstanceType::Array.into()),
                array: Some(Box::new(ArrayValidation {
                    items: Some(Schema::Object(subschema).into()),
                    ..Default::default()
                })),
                ..Default::default()
            }
        }
        t2::TypeInner::ObjectRef(object_ref) => {
            let schema =
                t2_object_to_schema(object_ref.r#type().clone(), default_metadata, context);
            context.schema_object_for_object_ref(object_ref, |_| schema)
        }
    };

    schema_object
}

fn t2_literal_string_union_to_schema(
    literal_string_union_inner: t2::LiteralStringUnionInner,
    default_metadata: Metadata,
) -> SchemaObject {
    let t2::LiteralStringUnionInner { members, default } = literal_string_union_inner;
    SchemaObject {
        metadata: Some(Box::new(Metadata {
            default: default.map(serde_json::Value::from),
            ..default_metadata
        })),
        instance_type: Some(InstanceType::String.into()),

        // sort HashSet to help with reproducibility.
        enum_values: Some(
            members
                .into_iter()
                .sorted()
                .map(|m| m.into())
                .collect::<Vec<_>>(),
        ),
        ..Default::default()
    }
}

fn t2_object_to_schema(
    object_inner: t2::ObjectInner,
    default_metadata: Metadata,
    context: &mut T2ToSchemaContext,
) -> SchemaObject {
    let t2::ObjectInner { members } = object_inner;

    let required: BTreeSet<_> = members
        .iter()
        .sorted_by_key(|member| &member.key) // sort to help with reproducibility.
        .filter(|m| (m.value.meta.is_required))
        .map(|m| m.key.clone())
        .collect();

    let properties: BTreeMap<_, _> = members
        .into_iter()
        .map(|m| {
            let subschema = t2_to_schema_object(m.value, context);

            (m.key, Schema::Object(subschema))
        })
        .collect();

    let schema_object = SchemaObject {
        metadata: Some(Box::new(default_metadata)),
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            properties,
            required,
            ..Default::default()
        })),
        ..Default::default()
    };

    schema_object
}

fn make_schema_nullable(schema: SchemaObject) -> SchemaObject {
    let null_schema = SchemaObject {
        instance_type: Some(InstanceType::Null.into()),
        ..Default::default()
    }
    .into();
    SchemaObject {
        subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
            one_of: Some(vec![schema.into(), null_schema]),
            ..Default::default()
        })),
        ..Default::default()
    }
}
