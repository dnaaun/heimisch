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
pub struct RepositoryRuleMergeQueueParameters {
    /// Maximum time for a required status check to report a conclusion. After this much time has elapsed, checks that have not reported a conclusion will be assumed to have failed
    #[serde(rename = "check_response_timeout_minutes")]
    pub check_response_timeout_minutes: i32,
    /// When set to ALLGREEN, the merge commit created by merge queue for each PR in the group must pass all required checks to merge. When set to HEADGREEN, only the commit at the head of the merge group, i.e. the commit containing changes from all of the PRs in the group, must pass its required checks to merge.
    #[serde(rename = "grouping_strategy")]
    pub grouping_strategy: GroupingStrategy,
    /// Limit the number of queued pull requests requesting checks and workflow runs at the same time.
    #[serde(rename = "max_entries_to_build")]
    pub max_entries_to_build: i32,
    /// The maximum number of PRs that will be merged together in a group.
    #[serde(rename = "max_entries_to_merge")]
    pub max_entries_to_merge: i32,
    /// Method to use when merging changes from queued pull requests.
    #[serde(rename = "merge_method")]
    pub merge_method: MergeMethod,
    /// The minimum number of PRs that will be merged together in a group.
    #[serde(rename = "min_entries_to_merge")]
    pub min_entries_to_merge: i32,
    /// The time merge queue should wait after the first PR is added to the queue for the minimum group size to be met. After this time has elapsed, the minimum group size will be ignored and a smaller group will be merged.
    #[serde(rename = "min_entries_to_merge_wait_minutes")]
    pub min_entries_to_merge_wait_minutes: i32,
}

impl RepositoryRuleMergeQueueParameters {
    pub fn new(
        check_response_timeout_minutes: i32,
        grouping_strategy: GroupingStrategy,
        max_entries_to_build: i32,
        max_entries_to_merge: i32,
        merge_method: MergeMethod,
        min_entries_to_merge: i32,
        min_entries_to_merge_wait_minutes: i32,
    ) -> RepositoryRuleMergeQueueParameters {
        RepositoryRuleMergeQueueParameters {
            check_response_timeout_minutes,
            grouping_strategy,
            max_entries_to_build,
            max_entries_to_merge,
            merge_method,
            min_entries_to_merge,
            min_entries_to_merge_wait_minutes,
        }
    }
}
/// When set to ALLGREEN, the merge commit created by merge queue for each PR in the group must pass all required checks to merge. When set to HEADGREEN, only the commit at the head of the merge group, i.e. the commit containing changes from all of the PRs in the group, must pass its required checks to merge.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupingStrategy {
    #[serde(rename = "ALLGREEN")]
    Allgreen,
    #[serde(rename = "HEADGREEN")]
    Headgreen,
}

impl Default for GroupingStrategy {
    fn default() -> GroupingStrategy {
        Self::Allgreen
    }
}
/// Method to use when merging changes from queued pull requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeMethod {
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "SQUASH")]
    Squash,
    #[serde(rename = "REBASE")]
    Rebase,
}

impl Default for MergeMethod {
    fn default() -> MergeMethod {
        Self::Merge
    }
}
