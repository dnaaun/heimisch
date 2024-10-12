# IssueEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**node_id** | **String** |  | 
**url** | **String** |  | 
**actor** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**event** | **String** |  | 
**commit_id** | Option<**String**> |  | 
**commit_url** | Option<**String**> |  | 
**created_at** | **String** |  | 
**issue** | Option<[**models::NullableIssue**](nullable-issue.md)> |  | [optional]
**label** | Option<[**models::IssueEventLabel**](issue-event-label.md)> |  | [optional]
**assignee** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**assigner** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**review_requester** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**requested_reviewer** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**requested_team** | Option<[**models::Team**](team.md)> |  | [optional]
**dismissed_review** | Option<[**models::IssueEventDismissedReview**](issue-event-dismissed-review.md)> |  | [optional]
**milestone** | Option<[**models::IssueEventMilestone**](issue-event-milestone.md)> |  | [optional]
**project_card** | Option<[**models::IssueEventProjectCard**](issue-event-project-card.md)> |  | [optional]
**rename** | Option<[**models::IssueEventRename**](issue-event-rename.md)> |  | [optional]
**author_association** | Option<[**models::AuthorAssociation**](author-association.md)> |  | [optional]
**lock_reason** | Option<**String**> |  | [optional]
**performed_via_github_app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


