use std::convert::Infallible;

use github_webhook_body::*;

use crate::sync_engine::conversions::ToDb;

impl ToDb for IssueCommentCreatedIssueActiveLockReason {
    type OtherChanges = ();
    type DbType = github_api::models::issue::ActiveLockReason;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, ()), Self::Error> {
        use github_api::models::issue::ActiveLockReason::*;
        Ok((
            match self {
                IssueCommentCreatedIssueActiveLockReason::OffTopic => OffTopic,
                IssueCommentCreatedIssueActiveLockReason::Resolved => Resolved,
                IssueCommentCreatedIssueActiveLockReason::Spam => Spam,
                IssueCommentCreatedIssueActiveLockReason::TooHeated => TooHeated,
            },
            Default::default(),
        ))
    }
}

impl ToDb for IssueCommentCreatedIssuePullRequest {
    type OtherChanges = ();
    type DbType = github_api::models::WebhooksIssuePullRequest;

    type Error = jiff::Error;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let IssueCommentCreatedIssuePullRequest {
            diff_url,
            html_url,
            merged_at,
            patch_url,
            url,
        } = self;
        let pr = github_api::models::WebhooksIssuePullRequest {
            diff_url,
            html_url,
            merged_at: Some(merged_at.map(|m| (m).parse()).transpose()?),
            patch_url,
            url,
        };
        Ok((pr, Default::default()))
    }
}
