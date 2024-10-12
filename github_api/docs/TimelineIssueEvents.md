# TimelineIssueEvents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**url** | **String** |  | 
**actor** | [**models::SimpleUser**](simple-user.md) |  | 
**event** | **String** |  | 
**commit_id** | Option<**String**> |  | 
**commit_url** | Option<**String**> |  | 
**created_at** | **String** |  | 
**performed_via_github_app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | 
**label** | [**models::LabeledIssueEventLabel**](labeled_issue_event_label.md) |  | 
**milestone** | [**models::MilestonedIssueEventMilestone**](milestoned_issue_event_milestone.md) |  | 
**rename** | [**models::RenamedIssueEventRename**](renamed_issue_event_rename.md) |  | 
**review_requester** | [**models::SimpleUser**](simple-user.md) |  | 
**requested_team** | Option<[**models::Team**](team.md)> |  | [optional]
**requested_reviewer** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]
**dismissed_review** | [**models::ReviewDismissedIssueEventDismissedReview**](review_dismissed_issue_event_dismissed_review.md) |  | 
**lock_reason** | Option<**String**> |  | 
**project_card** | Option<[**models::AddedToProjectIssueEventProjectCard**](added_to_project_issue_event_project_card.md)> |  | [optional]
**body** | Option<**String**> | The text of the review. | 
**body_text** | Option<**String**> |  | [optional]
**body_html** | Option<**String**> |  | [optional]
**html_url** | **String** |  | 
**user** | [**models::SimpleUser**](simple-user.md) |  | 
**updated_at** | **String** |  | 
**issue_url** | **String** |  | 
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 
**reactions** | Option<[**models::ReactionRollup**](reaction-rollup.md)> |  | [optional]
**source** | [**models::TimelineCrossReferencedEventSource**](timeline_cross_referenced_event_source.md) |  | 
**sha** | **String** | SHA for the commit | 
**author** | [**models::GitCommitAuthor**](git_commit_author.md) |  | 
**committer** | [**models::GitCommitAuthor**](git_commit_author.md) |  | 
**message** | **String** | Message describing the purpose of the commit | 
**tree** | [**models::GitCommitTree**](git_commit_tree.md) |  | 
**parents** | [**Vec<models::GitCommitParentsInner>**](git_commit_parents_inner.md) |  | 
**verification** | [**models::GitCommitVerification**](git_commit_verification.md) |  | 
**state** | **String** |  | 
**pull_request_url** | **String** |  | 
**_links** | [**models::TimelineReviewedEventLinks**](timeline_reviewed_event__links.md) |  | 
**submitted_at** | Option<**String**> |  | [optional]
**comments** | Option<[**Vec<models::CommitComment>**](commit-comment.md)> |  | [optional]
**assignee** | [**models::SimpleUser**](simple-user.md) |  | 
**state_reason** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


