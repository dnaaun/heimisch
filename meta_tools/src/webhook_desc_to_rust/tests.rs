use std::collections::BTreeMap;

use anyhow::{Ok, Result};
use assert_json_diff::assert_json_eq;
use schemars::schema::{RootSchema, Schema};
use serde_json::json;

use crate::webhook_desc_to_rust::t1::{
    Availability, ParameterType, TopLevelWebhookSchema, WebhookSchema,
};

use super::{
    t1,
    t1_to_t2::t1_top_level_to_t2,
    t2_to_json_schema::{t2_to_schema_context::T2ToSchemaContext, t2_to_schema_object},
};

fn wrap_in_toplevel(webhook_schemas: Vec<WebhookSchema>) -> TopLevelWebhookSchema {
    TopLevelWebhookSchema {
        availability: vec![Availability::App],
        body_parameters: webhook_schemas,
        category: "cat".into(),
        action: "act".into(),
        ..Default::default()
    }
}

pub fn t1_top_level_to_json_schema(t1_top_level: &t1::TopLevelWebhookSchema) -> RootSchema {
    let mut literal_string_union_store = Default::default();
    let mut object_store = Default::default();

    let t2_type = t1_top_level_to_t2(
        t1_top_level,
        &mut literal_string_union_store,
        &mut object_store,
    )
    .unwrap();

    let mut t2_to_schema_context = T2ToSchemaContext::new(literal_string_union_store, object_store);

    let schema = t2_to_schema_object(t2_type, &mut t2_to_schema_context);

    let definitions: BTreeMap<String, Schema> = t2_to_schema_context
        .all_definitions()
        .map(|(name, schema_object)| (name, schema_object.into()))
        .collect();

    RootSchema {
        meta_schema: None,
        schema,
        definitions,
    }
}

#[test]
fn literal_bool() -> Result<()> {
    let top_level = wrap_in_toplevel(vec![WebhookSchema {
        name: "areYouAwesome".into(),
        r#type: ParameterType::Boolean,
        enum_values: Some(vec![Some(true.into())]),
        ..Default::default()
    }]);
    let schema_json = serde_json::to_value(&t1_top_level_to_json_schema(&top_level))?;

    let expected_schema_json = json!({
      "title": "CatAct",
      "description": "",
      "type": "object",
      "properties": {
        "areYouAwesome": {
          "title": "CatActAreYouAwesome",
          "type": "boolean",
          "const": true
        }
      }
    });

    assert_json_eq!(schema_json, expected_schema_json);

    Ok(())
}

#[test]
fn required_object_props() -> Result<()> {
    let top_level = wrap_in_toplevel(vec![
        WebhookSchema {
            name: "required_prop".into(),
            r#type: ParameterType::String,
            is_required: Some(true.into()),
            ..Default::default()
        },
        WebhookSchema {
            name: "not_required_prop".into(),
            r#type: ParameterType::Integer,
            ..Default::default()
        },
    ]);
    let schema_json = serde_json::to_value(&t1_top_level_to_json_schema(&top_level))?;

    let expected_schema_json = serde_json::json!({
      "description": "",
      "properties": {
        "not_required_prop": {
          "title": "CatActNotRequiredProp",
          "type": "integer"
        },
        "required_prop": {
          "title": "CatActRequiredProp",
          "type": "string"
        }
      },
      "required": [
        "required_prop"
      ],
      "title": "CatAct",
      "type": "object"
    });
    assert_json_eq!(schema_json, expected_schema_json);
    Ok(())
}

#[test]
fn string_or_null_enum() -> Result<()> {
    let top_level = wrap_in_toplevel(vec![WebhookSchema {
        name: "hey".into(),
        r#type: ParameterType::StringOrNull,
        enum_values: Some(vec![Some("ho".to_string().into()), None]),
        ..Default::default()
    }]);
    let schema_json = serde_json::to_value(&t1_top_level_to_json_schema(&top_level))?;

    let expected_schema_json = serde_json::json!({
      "description": "",
      "properties": {
        "hey": {
          "oneOf": [
            {
              "enum": [
                "ho"
              ],
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "title": "CatActHey"
        }
      },
      "title": "CatAct",
      "type": "object"
    }
    );
    assert_json_eq!(schema_json, expected_schema_json);
    Ok(())
}

#[test]
fn increment_name_suffix() -> Result<()> {
    let address_identically_named = Box::new(WebhookSchema {
        name: "po_box".into(),
        r#type: ParameterType::Integer,
        ..Default::default()
    });

    let name_identically_named = Box::new(WebhookSchema {
        name: "IdenticallyNamedObject".into(),
        r#type: ParameterType::Object,
        child_params_groups: Some(vec![
            Box::new(WebhookSchema {
                name: "first".into(),
                r#type: ParameterType::String,
                ..Default::default()
            }),
            Box::new(WebhookSchema {
                name: "last".into(),
                r#type: ParameterType::String,
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });

    let top_level = wrap_in_toplevel(vec![
        WebhookSchema {
            name: "prop1".to_owned(),
            r#type: ParameterType::Object,
            child_params_groups: Some(vec![
                WebhookSchema {
                    name: "mediator".to_owned(),
                    r#type: ParameterType::Object,
                    child_params_groups: Some(vec![
                        name_identically_named.clone(),
                        WebhookSchema {
                            name: "so_mediator_doesnt_get_deduped_left".to_owned(),
                            r#type: ParameterType::Integer,
                            ..Default::default()
                        }
                        .into(),
                    ]),
                    ..Default::default()
                }
                .into(),
                WebhookSchema {
                    name: "IdenticallyNamedObject".into(),
                    r#type: ParameterType::Object,
                    child_params_groups: Some(vec![address_identically_named.clone()]),
                    ..Default::default()
                }
                .into(),
            ]),
            ..Default::default()
        },
        WebhookSchema {
            name: "prop2".to_owned(),
            r#type: ParameterType::Object,
            child_params_groups: Some(vec![
                WebhookSchema {
                    name: "mediator".to_owned(),
                    r#type: ParameterType::Object,
                    child_params_groups: Some(vec![
                        name_identically_named,
                        WebhookSchema {
                            name: "so_mediator_doesnt_get_deduped_right".to_owned(),
                            r#type: ParameterType::Integer,
                            ..Default::default()
                        }
                        .into(),
                    ]),
                    ..Default::default()
                }
                .into(),
                WebhookSchema {
                    name: "IdenticallyNamedObject".into(),
                    r#type: ParameterType::Object,
                    child_params_groups: Some(vec![address_identically_named]),
                    ..Default::default()
                }
                .into(),
            ]),
            ..Default::default()
        },
    ]);

    let root_schema = t1_top_level_to_json_schema(&top_level);
    let schema_json_value = serde_json::to_value(&root_schema)?;
    let definitions = schema_json_value["definitions"].as_object().unwrap();
    let expected_definitions = json!(
        {
            "CatActProp1MediatorIdenticallyNamedObject": {
          "title": "CatActProp1MediatorIdenticallyNamedObject",
          "type": "object",
          "properties": {
            "first": {
              "title": "CatActProp1MediatorIdenticallyNamedObjectFirst",
              "type": "string"
            },
            "last": {
              "title": "CatActProp1MediatorIdenticallyNamedObjectLast",
              "type": "string"
            }
          }
    },
    "CatActProp1IdenticallyNamedObject": {
          "title": "CatActProp1IdenticallyNamedObject",
          "type": "object",
          "properties": {
            "po_box": {
              "title": "CatActProp1IdenticallyNamedObjectPoBox",
              "type": "integer"
            }
          }
        }
        });

    assert_json_eq!(definitions, expected_definitions);

    Ok(())
}

#[test]
fn simple_object_deduplication() -> Result<()> {
    let to_be_dedup = WebhookSchema {
        r#type: ParameterType::Object,
        name: "to_be_dedup".to_string(),
        child_params_groups: Some(vec![
            Box::new(WebhookSchema {
                r#type: ParameterType::Integer,
                name: "age".to_string(),
                ..Default::default()
            }),
            Box::new(WebhookSchema {
                r#type: ParameterType::String,
                name: "first_name".to_string(),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    };
    let t1_top_level = wrap_in_toplevel(vec![
        WebhookSchema {
            name: "user1".into(),
            ..to_be_dedup.clone()
        },
        WebhookSchema {
            name: "user2".into(),
            ..to_be_dedup
        },
    ]);

    let schema_json = serde_json::to_value(&t1_top_level_to_json_schema(&t1_top_level))?;
    let expected_schema_json = serde_json::json!(
    {
      "title": "CatAct",
      "description": "",
      "type": "object",
          "properties": {
            "user1": {
              "$ref": "#/$defs/CatActUser1"
            },
            "user2": {
              "$ref": "#/$defs/CatActUser1"
            }
      },
      "definitions": {
        "CatActUser1": {
          "title": "CatActUser1",
          "type": "object",
          "properties": {
            "age": {
              "title": "CatActUser1Age",
              "type": "integer"
            },
            "first_name": {
              "title": "CatActUser1FirstName",
              "type": "string"
            }
          }
        }
      }
    }
    );
    assert_json_eq!(schema_json, expected_schema_json);
    Ok(())
}
