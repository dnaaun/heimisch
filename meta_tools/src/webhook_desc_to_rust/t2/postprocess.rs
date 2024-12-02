use std::collections::HashMap;

use super::{
    ArrayInner, IntegerInner, Meta, ObjectInner, ObjectMember, Type, TypeInner, TypeRef, UnionInner,
};

/// Two things:
///  1. If there's an attribute at the top level called `installation` and it is either an object
///     with no properties, or a nullable object with no properties, then replace that attribute
///     with a `InstallationAttribute` object that actually has an `id`, which is what the webhook
///     responses show.
///
///  2. Rename objects with no attributes to `Untyped` (unless they match the above criteria, of
///     course).
pub fn postprocess_t2_inner(
    inner: TypeInner,
    object_store: &mut HashMap<TypeRef<ObjectInner>, u32>,
    at_toplevel: bool,
) -> TypeInner {
    match inner {
        TypeInner::Nullable(t) => {
            TypeInner::Nullable(postprocess_t2_inner(*t, object_store, false).into())
        }
        TypeInner::Union(UnionInner { variants }) => {
            let variants = variants
                .into_iter()
                .map(|i| postprocess_t2_inner(i, object_store, false))
                .collect();
            TypeInner::Union(UnionInner { variants })
        }
        TypeInner::Array(ArrayInner { members }) => {
            let members = postprocess_t2_inner(*members, object_store, false).into();
            TypeInner::Array(ArrayInner { members })
        }
        TypeInner::ObjectRef(type_ref) => {
            let new_members = type_ref
                .r#type()
                .members
                .clone()
                .into_iter()
                .map(|member| {
                    if at_toplevel && member.key == "installation" {
                        match &member.value.inner {
                            TypeInner::Nullable(type_inner) => {
                                if let TypeInner::ObjectRef(object_ref) = type_inner.as_ref() {
                                    if object_ref.r#type().members.is_empty() {
                                        let Type { inner, meta } = installation_attribute_type(
                                            member.value.meta,
                                            object_store,
                                        );

                                        return ObjectMember {
                                            key: member.key,
                                            value: Type {
                                                inner: TypeInner::Nullable(inner.into()),
                                                meta,
                                            },
                                        };
                                    }
                                }
                            }
                            TypeInner::ObjectRef(object_ref)
                                if object_ref.r#type().members.is_empty() =>
                            {
                                return ObjectMember {
                                    key: member.key,
                                    value: installation_attribute_type(
                                        member.value.meta,
                                        object_store,
                                    ),
                                }
                            }
                            _ => (),
                        }
                    };

                    member.map_value(|t| {
                        t.map_inner(|i| postprocess_t2_inner(i, object_store, false))
                    })
                })
                .collect::<Vec<_>>();

            let new_name = if new_members.is_empty() {
                "Untyped".to_owned()
            } else {
                type_ref.name().to_owned()
            };

            TypeRef::new_or_incr_count(
                ObjectInner {
                    members: new_members,
                },
                new_name,
                object_store,
            )
            .into()
        }
        _ => inner,
    }
}

fn installation_attribute_type(
    meta: Meta,
    object_store: &mut HashMap<TypeRef<ObjectInner>, u32>,
) -> Type {
    Type {
        inner: TypeRef::new_or_incr_count(
            ObjectInner {
                members: vec![ObjectMember {
                    key: "id".into(),
                    value: Type {
                        inner: IntegerInner::default().into(),
                        meta: Meta {
                            title: None,
                            is_required: true,
                            description: None,
                        },
                    },
                }],
            },
            "InstallationAttribute".to_owned(),
            object_store,
        )
        .into(),
        meta,
    }
}
