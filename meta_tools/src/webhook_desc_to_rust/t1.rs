use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    #[default]
    String,
    Object,
    Integer,
    Number,
    Boolean,
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "null or object")]
    NullOrObject,
    #[serde(rename = "null or string or object")]
    NullOrStringOrObject,
    #[serde(rename = "null or string or array")]
    NullOrStringOrArray,
    #[serde(rename = "object or string")]
    ObjectOrString,
    #[serde(rename = "string or object")]
    StringOrObject,
    #[serde(rename = "string or number")]
    StringOrNumber,
    #[serde(rename = "object or null")]
    ObjectOrNull,
    #[serde(rename = "string or null")]
    StringOrNull,
    #[serde(rename = "integer or null")]
    IntegerOrNull,
    #[serde(rename = "number or null")]
    NumberOrNull,
    #[serde(rename = "boolean or null")]
    BooleanOrNull,
    #[serde(rename = "integer or string")]
    IntegerOrString,
    #[serde(rename = "null or integer or string")]
    NullOrIntegerOrString,
    #[serde(rename = "array of objects")]
    ArrayOfObjects,
    #[serde(rename = "array of objects or null")]
    ArrayOfObjectsOrNull,
    #[serde(rename = "array of strings")]
    ArrayOfStrings,
    #[serde(rename = "array of strings or null")]
    ArrayOfStringsOrNull,
    #[serde(rename = "array of object,nulls")]
    ArrayOfObjectNulls,
    #[serde(rename = "array of string,nulls")]
    ArrayOfStringNulls,
    #[serde(rename = "boolean or string or integer or object")]
    BooleanOrStringOrIntegerOrObject,
    #[serde(rename = "null or string or object or integer")]
    NullOrStringOrObjectOrInteger,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
    Body,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TopLevelWebhookSchema {
    pub description_html: String,
    pub summary_html: String,
    pub body_parameters: Vec<WebhookSchema>,
    pub availability: Vec<Availability>,
    pub category: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, strum::EnumTryAs)]
#[serde(untagged)]
pub enum T1Enum {
    Bool(bool),
    String(String),
}

impl From<bool> for T1Enum {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for T1Enum {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum IsRequiredWeirdness {
    /// For the `repository_ruleset` webhooks, it seems that the `is_required` attribute
    /// can be a list that indicates which of the attributes of the schema that the object
    /// `is_required` is on are required. In all other webhooks, `is_required` is either `true` or
    /// `false` and indicates (I presume) whether the very attribute that the object `is_required`
    /// is on is required on. To accomodate `repository_ruleset`, we have `IsRequiredWeirdness`.
    Arrary(Vec<String>),
    Bool(bool),
}

impl From<bool> for IsRequiredWeirdness {
    fn from(value: bool) -> Self {
        IsRequiredWeirdness::Bool(value)
    }
}

impl From<&IsRequiredWeirdness> for bool {
    fn from(value: &IsRequiredWeirdness) -> Self {
        match value {
            // Look at IsRequiredWeirdness for why this is false.
            IsRequiredWeirdness::Arrary(_) => false,
            IsRequiredWeirdness::Bool(b) => *b,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebhookSchema {
    pub r#type: ParameterType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<IsRequiredWeirdness>,
    #[serde(rename = "enum", default, skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Option<T1Enum>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child_params_groups: Option<Vec<Box<WebhookSchema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}

/// TODO: Check that this contains `App` every time.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Repository,
    Organization,
    App,
    SponsorsListing,
    Business,
    Marketplace,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    #[default]
    PullRequests,
    Issues,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Assigned,
    AutoMergeDisabled,
    AutoMergeEnabled,
    Closed,
    ConvertedToDraft,
    Demilestoned,
    Dequeued,
    Edited,
    Enqueued,
    Labeled,
    Locked,
    Milestoned,
    Opened,
    ReadyForReview,
    Reopened,
    ReviewRequestRemoved,
    ReviewRequested,
    Synchronize,
    Unassigned,
    Unlabeled,
    Unlocked,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, strum::Display)]
#[serde(rename_all = "lowercase")]
pub enum IssueAction {
    Assigned,
    Closed,
    Deleted,
    Demilestoned,
    Edited,
    Labeled,
    Locked,
    Milestoned,
    Opened,
    Pinned,
    Reopened,
    Transferred,
    Unassigned,
    Unlabeled,
    Unlocked,
    Unpinned,
}
