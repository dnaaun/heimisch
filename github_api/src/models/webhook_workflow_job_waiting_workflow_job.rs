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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookWorkflowJobWaitingWorkflowJob {
    #[serde(rename = "check_run_url")]
    pub check_run_url: String,
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    #[serde(rename = "conclusion", deserialize_with = "Option::deserialize")]
    pub conclusion: Option<String>,
    /// The time that the job created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "run_attempt")]
    pub run_attempt: i32,
    #[serde(rename = "run_id")]
    pub run_id: f64,
    #[serde(rename = "run_url")]
    pub run_url: String,
    #[serde(rename = "runner_group_id", deserialize_with = "Option::deserialize")]
    pub runner_group_id: Option<i32>,
    #[serde(rename = "runner_group_name", deserialize_with = "Option::deserialize")]
    pub runner_group_name: Option<String>,
    #[serde(rename = "runner_id", deserialize_with = "Option::deserialize")]
    pub runner_id: Option<i32>,
    #[serde(rename = "runner_name", deserialize_with = "Option::deserialize")]
    pub runner_name: Option<String>,
    #[serde(rename = "started_at")]
    pub started_at: String,
    /// The name of the current branch.
    #[serde(rename = "head_branch", deserialize_with = "Option::deserialize")]
    pub head_branch: Option<String>,
    /// The name of the workflow.
    #[serde(rename = "workflow_name", deserialize_with = "Option::deserialize")]
    pub workflow_name: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "steps")]
    pub steps: Vec<models::WorkflowStep4>,
    #[serde(rename = "url")]
    pub url: String,
}

impl WebhookWorkflowJobWaitingWorkflowJob {
    pub fn new(
        check_run_url: String,
        completed_at: Option<String>,
        conclusion: Option<String>,
        created_at: String,
        head_sha: String,
        html_url: String,
        id: i32,
        labels: Vec<String>,
        name: String,
        node_id: String,
        run_attempt: i32,
        run_id: f64,
        run_url: String,
        runner_group_id: Option<i32>,
        runner_group_name: Option<String>,
        runner_id: Option<i32>,
        runner_name: Option<String>,
        started_at: String,
        head_branch: Option<String>,
        workflow_name: Option<String>,
        status: Status,
        steps: Vec<models::WorkflowStep4>,
        url: String,
    ) -> WebhookWorkflowJobWaitingWorkflowJob {
        WebhookWorkflowJobWaitingWorkflowJob {
            check_run_url,
            completed_at,
            conclusion,
            created_at,
            head_sha,
            html_url,
            id,
            labels,
            name,
            node_id,
            run_attempt,
            run_id,
            run_url,
            runner_group_id,
            runner_group_name,
            runner_id,
            runner_name,
            started_at,
            head_branch,
            workflow_name,
            status,
            steps,
            url,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "waiting")]
    Waiting,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}
