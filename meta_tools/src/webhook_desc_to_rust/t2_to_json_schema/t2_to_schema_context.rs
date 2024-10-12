use std::{collections::HashMap, hash::Hash};

use schemars::schema::SchemaObject;

use crate::webhook_desc_to_rust::t2::{self, TypeRef};

pub struct T2ToSchemaContext {
    literal_string_union_store: HashMap<TypeRef<t2::LiteralStringUnionInner>, u32>,
    object_store: HashMap<TypeRef<t2::ObjectInner>, u32>,
    used_def_names: HashMap<String, u32>,
    literal_string_union_definitions:
        HashMap<TypeRef<t2::LiteralStringUnionInner>, (String, SchemaObject)>,
    object_definitions: HashMap<TypeRef<t2::ObjectInner>, (String, SchemaObject)>,
}

impl T2ToSchemaContext {
    pub fn new(
        literal_string_union_store: HashMap<TypeRef<t2::LiteralStringUnionInner>, u32>,
        object_store: HashMap<TypeRef<t2::ObjectInner>, u32>,
    ) -> Self {
        Self {
            literal_string_union_store,
            object_store,
            used_def_names: Default::default(),
            literal_string_union_definitions: Default::default(),
            object_definitions: Default::default(),
        }
    }

    pub fn schema_object_for_literal_string_union_ref(
        &mut self,
        ref_: TypeRef<t2::LiteralStringUnionInner>,
        convert: impl FnOnce(t2::LiteralStringUnionInner) -> SchemaObject,
    ) -> SchemaObject {
        Self::schema_object_for(
            ref_,
            convert,
            &self.literal_string_union_store,
            &mut self.used_def_names,
            &mut self.literal_string_union_definitions,
        )
    }

    pub fn schema_object_for_object_ref(
        &mut self,
        ref_: TypeRef<t2::ObjectInner>,
        convert: impl FnOnce(t2::ObjectInner) -> SchemaObject,
    ) -> SchemaObject {
        Self::schema_object_for(
            ref_,
            convert,
            &self.object_store,
            &mut self.used_def_names,
            &mut self.object_definitions,
        )
    }

    /// Will return the "reference": "NameOfLiteral" json schema thingy.
    fn schema_object_for<T: std::fmt::Debug + Hash + Eq + Clone + 'static>(
        ref_: TypeRef<T>,
        convert: impl FnOnce(T) -> SchemaObject,
        store: &HashMap<TypeRef<T>, u32>,
        used_def_names: &mut HashMap<String, u32>,
        definitions: &mut HashMap<TypeRef<T>, (String, SchemaObject)>,
    ) -> SchemaObject {
        let result = match definitions.get(&ref_) {
            Some((name, _)) => SchemaObject {
                reference: Some(format!("#/$defs/{name}")),
                ..Default::default()
            },
            None => {
                let type_in_t2_count = store
                    .get(&ref_)
                    .expect("internal error. reference to type should be found here.");

                if *type_in_t2_count == 1 {
                    convert(ref_.r#type().to_owned())
                } else {
                    let def_name_use_count = used_def_names.entry(ref_.name().clone()).or_insert(0);
                    *def_name_use_count += 1;
                    let name = if *def_name_use_count > 1 {
                        format!("{}{def_name_use_count}", ref_.name())
                    } else {
                        ref_.name().clone()
                    };
                    let schema_object = convert(ref_.r#type().clone());
                    definitions.insert(ref_.clone(), (name.clone(), schema_object));
                    SchemaObject {
                        reference: Some(format!("#/$defs/{name}")),
                        ..Default::default()
                    }
                }
            }
        };
        result
    }

    pub fn all_definitions(self) -> impl Iterator<Item = (String, SchemaObject)> {
        let Self {
            literal_string_union_definitions,
            object_definitions,
            ..
        } = self;
        literal_string_union_definitions
            .into_iter()
            .map(|(_, v)| v)
            .chain(object_definitions.into_iter().map(|(_, v)| v))
    }
}
