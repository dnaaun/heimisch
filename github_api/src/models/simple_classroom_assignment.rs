/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SimpleClassroomAssignment : A GitHub Classroom assignment
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleClassroomAssignment {
    /// Unique identifier of the repository.
    #[serde(rename = "id")]
    pub id: i32,
    /// Whether an accepted assignment creates a public repository.
    #[serde(rename = "public_repo")]
    pub public_repo: bool,
    /// Assignment title.
    #[serde(rename = "title")]
    pub title: String,
    /// Whether it's a Group Assignment or Individual Assignment.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The link that a student can use to accept the assignment.
    #[serde(rename = "invite_link")]
    pub invite_link: String,
    /// Whether the invitation link is enabled. Visiting an enabled invitation link will accept the assignment.
    #[serde(rename = "invitations_enabled")]
    pub invitations_enabled: bool,
    /// Sluggified name of the assignment.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Whether students are admins on created repository on accepted assignment.
    #[serde(rename = "students_are_repo_admins")]
    pub students_are_repo_admins: bool,
    /// Whether feedback pull request will be created on assignment acceptance.
    #[serde(rename = "feedback_pull_requests_enabled")]
    pub feedback_pull_requests_enabled: bool,
    /// The maximum allowable teams for the assignment.
    #[serde(
        rename = "max_teams",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_teams: Option<Option<i32>>,
    /// The maximum allowable members per team.
    #[serde(
        rename = "max_members",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_members: Option<Option<i32>>,
    /// The selected editor for the assignment.
    #[serde(rename = "editor")]
    pub editor: String,
    /// The number of students that have accepted the assignment.
    #[serde(rename = "accepted")]
    pub accepted: i32,
    /// The number of students that have submitted the assignment.
    #[serde(rename = "submitted")]
    pub submitted: i32,
    /// The number of students that have passed the assignment.
    #[serde(rename = "passing")]
    pub passing: i32,
    /// The programming language used in the assignment.
    #[serde(rename = "language")]
    pub language: String,
    /// The time at which the assignment is due.
    #[serde(rename = "deadline", deserialize_with = "Option::deserialize")]
    pub deadline: Option<String>,
    #[serde(rename = "classroom")]
    pub classroom: Box<models::SimpleClassroom>,
}

impl SimpleClassroomAssignment {
    /// A GitHub Classroom assignment
    pub fn new(
        id: i32,
        public_repo: bool,
        title: String,
        r#type: Type,
        invite_link: String,
        invitations_enabled: bool,
        slug: String,
        students_are_repo_admins: bool,
        feedback_pull_requests_enabled: bool,
        editor: String,
        accepted: i32,
        submitted: i32,
        passing: i32,
        language: String,
        deadline: Option<String>,
        classroom: models::SimpleClassroom,
    ) -> SimpleClassroomAssignment {
        SimpleClassroomAssignment {
            id,
            public_repo,
            title,
            r#type,
            invite_link,
            invitations_enabled,
            slug,
            students_are_repo_admins,
            feedback_pull_requests_enabled,
            max_teams: None,
            max_members: None,
            editor,
            accepted,
            submitted,
            passing,
            language,
            deadline,
            classroom: Box::new(classroom),
        }
    }
}
/// Whether it's a Group Assignment or Individual Assignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "group")]
    Group,
}

impl Default for Type {
    fn default() -> Type {
        Self::Individual
    }
}
