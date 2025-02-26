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
pub struct PullsCreateReviewRequest {
    /// The SHA of the commit that needs a review. Not using the latest commit SHA may render your review comment outdated if a subsequent commit modifies the line you specify as the `position`. Defaults to the most recent commit in the pull request when you do not specify a value.
    #[serde(rename = "commit_id", skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// **Required** when using `REQUEST_CHANGES` or `COMMENT` for the `event` parameter. The body text of the pull request review.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request) when you are ready.
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    /// Use the following table to specify the location, destination, and contents of the draft review comment.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<models::PullsCreateReviewRequestCommentsInner>>,
}

impl PullsCreateReviewRequest {
    pub fn new() -> PullsCreateReviewRequest {
        PullsCreateReviewRequest {
            commit_id: None,
            body: None,
            event: None,
            comments: None,
        }
    }
}
/// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request) when you are ready.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "REQUEST_CHANGES")]
    RequestChanges,
    #[serde(rename = "COMMENT")]
    Comment,
}

impl Default for Event {
    fn default() -> Event {
        Self::Approve
    }
}
