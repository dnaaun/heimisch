use std::{collections::HashSet, fs::DirEntry, io::Write};
mod t1;
mod t1_to_t2;
mod t2;
mod t2_to_json_schema;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use quote::ToTokens;
use schemars::schema::{
    InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SubschemaValidation,
};
use std::{
    collections::HashMap,
    fs::File,
    path::Path,
    process::{Command, Stdio},
};
use t1_to_t2::t1_top_level_to_t2;
use t2::postprocess_t2_inner;

use t2_to_json_schema::{t2_to_schema_context::T2ToSchemaContext, t2_to_schema_object};
use typify::TypeSpace;

pub fn pretty_print(stream: impl ToTokens) -> Result<String> {
    let tokens = stream.into_token_stream().to_string();

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(tokens.as_bytes())?;
    stdin.flush()?;
    drop(stdin);

    let output = child.wait_with_output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn webhook_desc_to_rust(dir_containing_webhooks: &Path) -> Result<()> {
    let json_path_dir_entries: Vec<DirEntry> = dir_containing_webhooks.read_dir()?.try_collect()?;

    let top_level_webhook_schemas_by_event = json_path_dir_entries
        .iter()
        .map(|dir_entry| {
            let path = dir_entry.path();
            let deser = &mut serde_json::Deserializer::from_reader(File::open(&path)?);
            let t1_top_level_many: HashMap<String, t1::TopLevelWebhookSchema> =
                serde_path_to_error::deserialize(deser)
                    .context(format!("{}", path.to_str().unwrap()))?;
            let t1_top_level_schemas_by_event =
                t1_top_level_many.into_iter().map(|(_, v)| v).collect_vec();
            let events = t1_top_level_schemas_by_event
                .iter()
                .map(|i| i.category.clone())
                .collect::<HashSet<_>>();
            if events.len() != 1 {
                return Err(anyhow!(
                    "Didn't expect multiple events in same file: {events:?}"
                ));
            }
            Ok((
                events.into_iter().next().unwrap(),
                t1_top_level_schemas_by_event,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut literal_string_union_store = Default::default();
    let mut object_store = Default::default();

    // We sort below in hopes of having (slighty?) better reproducability when rerunning this tool
    // on the same input.
    let t2_types_by_event = top_level_webhook_schemas_by_event
        .into_iter()
        .filter_map(|(event, top_level_webhook_schemas)| {
            let t2_types = top_level_webhook_schemas
                .iter()
                .sorted_by_key(|s| &s.action)
                .filter_map(|t1_top_level| {
                    t1_top_level_to_t2(
                        t1_top_level,
                        &mut literal_string_union_store,
                        &mut object_store,
                    )
                })
                .collect::<Vec<_>>();

            // I couldn't tack the .map below onto the above because Rust thinks object_store is
            // being mutablly borrowed twice at the same time. (Could probably rewrite this whole
            // thing as a for loop and convince Rust of that, but who cares?).
            let t2_types = t2_types
                .into_iter()
                .map(|t2_type| {
                    t2_type.map_inner(|t| postprocess_t2_inner(t, &mut object_store, true))
                })
                .collect::<Vec<_>>();

            // For some events, none will have webhooks available to Github Apps, which is
            // indicated by None being returned by `t1_top_level_to_t2`.
            if t2_types.len() == 0 {
                None
            } else {
                Some((event, t2_types))
            }
        })
        .collect::<Vec<_>>();

    let mut t2_to_schema_context = T2ToSchemaContext::new(literal_string_union_store, object_store);

    let schemas_by_event = t2_types_by_event
        .into_iter()
        .map(|(event, t2_types)| {
            (
                event,
                t2_types
                    .into_iter()
                    .map(|t2_type| {
                        Schema::Object(t2_to_schema_object(t2_type, &mut t2_to_schema_context))
                    })
                    .collect_vec(),
            )
        })
        .collect::<Vec<_>>();

    let mut typespace = TypeSpace::new(&Default::default());

    typespace.add_ref_types(
        t2_to_schema_context
            .all_definitions()
            .map(|(name, schema_object)| (name, Schema::Object(schema_object))),
    )?;

    let union_schema_objects_by_event = schemas_by_event
        .iter()
        .map(|(event, schemas)| {
            (
                event.clone(),
                SchemaObject {
                    metadata: Some(
                        Metadata {
                            title: Some((*event).clone()),
                            ..Default::default()
                        }
                        .into(),
                    ),
                    subschemas: Some(Box::new(SubschemaValidation {
                        one_of: Some(schemas.clone()),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )
        })
        .collect_vec();

    let (sub_schemas, defns): (Vec<_>, Vec<_>) = union_schema_objects_by_event
        .iter()
        .map(|(event, schema_object)| {
            let defn = (event.clone(), Schema::from(schema_object.clone()));
            let defn_ref = SchemaObject {
                reference: Some(format!("#/$defs/{event}")),
                ..Default::default()
            };
            let properties = [(event.clone(), defn_ref.into())].into();
            let required = [event.clone()].into();

            let sub_schema = Schema::Object(SchemaObject {
                metadata: Some(
                    Metadata {
                        title: (*event).clone().into(),
                        ..Default::default()
                    }
                    .into(),
                ),
                instance_type: Some(InstanceType::Object.into()),
                object: Some(
                    ObjectValidation {
                        properties,
                        required,
                        ..Default::default()
                    }
                    .into(),
                ),
                ..Default::default()
            });

            (sub_schema, defn)
        })
        .unzip();
    typespace.add_ref_types(defns)?;

    let big_union: Schema = SchemaObject {
        metadata: Some(
            Metadata {
                title: Some("WebhookBody".into()),
                ..Default::default()
            }
            .into(),
        ),
        instance_type: Some(InstanceType::Object.into()),
        subschemas: Some(Box::new(SubschemaValidation {
            one_of: Some(sub_schemas),
            ..Default::default()
        })),
        ..Default::default()
    }
    .into();
    typespace.add_type_with_name(&big_union, Some("WebhookBody".into()))?;

    let stream = typespace.to_stream();

    print!("{}", pretty_print(stream)?);

    Ok(())
}
