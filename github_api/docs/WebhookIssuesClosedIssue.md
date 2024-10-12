# WebhookIssuesClosedIssue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_lock_reason** | Option<**String**> |  | 
**assignee** | Option<[**serde_json::Value**](.md)> |  | [optional]
**assignees** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**author_association** | **String** |  | 
**body** | Option<**String**> |  | 
**closed_at** | Option<**String**> |  | 
**comments** | **i32** |  | 
**comments_url** | **String** |  | 
**created_at** | **String** |  | 
**draft** | Option<**bool**> |  | [optional]
**events_url** | **String** |  | 
**html_url** | **String** |  | 
**id** | **i32** |  | 
**labels** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**labels_url** | **String** |  | 
**locked** | Option<**bool**> |  | [optional]
**milestone** | Option<[**serde_json::Value**](.md)> |  | 
**node_id** | **String** |  | 
**number** | **i32** |  | 
**performed_via_github_app** | Option<[**serde_json::Value**](.md)> |  | [optional]
**pull_request** | Option<[**models::WebhooksIssuePullRequest**](webhooks_issue_pull_request.md)> |  | [optional]
**reactions** | [**models::WebhookIssueCommentCreatedIssueAllOfReactions**](webhook_issue_comment_created_issue_allOf_reactions.md) |  | 
**repository_url** | **String** |  | 
**state** | **String** |  | 
**state_reason** | Option<**String**> |  | [optional]
**timeline_url** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**user** | [**models::WebhookIssueCommentDeletedIssueAllOfUser**](webhook_issue_comment_deleted_issue_allOf_user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


