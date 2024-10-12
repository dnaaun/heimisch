# IssueEventForIssue

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
**performed_via_github_app** | Option<[**models::Integration**](integration.md)> |  | 
**label** | [**models::LabeledIssueEventLabel**](labeled_issue_event_label.md) |  | 
**assignee** | [**models::SimpleUser**](simple-user.md) |  | 
**assigner** | [**models::SimpleUser**](simple-user.md) |  | 
**milestone** | [**models::MilestonedIssueEventMilestone**](milestoned_issue_event_milestone.md) |  | 
**rename** | [**models::RenamedIssueEventRename**](renamed_issue_event_rename.md) |  | 
**review_requester** | [**models::SimpleUser**](simple-user.md) |  | 
**requested_team** | Option<[**models::Team**](team.md)> |  | [optional]
**requested_reviewer** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]
**dismissed_review** | [**models::ReviewDismissedIssueEventDismissedReview**](review_dismissed_issue_event_dismissed_review.md) |  | 
**lock_reason** | Option<**String**> |  | 
**project_card** | Option<[**models::AddedToProjectIssueEventProjectCard**](added_to_project_issue_event_project_card.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


