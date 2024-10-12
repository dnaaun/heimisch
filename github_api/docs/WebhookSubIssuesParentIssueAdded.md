# WebhookSubIssuesParentIssueAdded

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**parent_issue_id** | **f64** | The ID of the parent issue. | 
**parent_issue** | [**models::Issue**](issue.md) |  | 
**parent_issue_repo** | [**models::Repository**](repository.md) |  | 
**sub_issue_id** | **f64** | The ID of the sub-issue. | 
**sub_issue** | [**models::Issue**](issue.md) |  | 
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**repository** | Option<[**models::RepositoryWebhooks**](repository-webhooks.md)> |  | [optional]
**sender** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


