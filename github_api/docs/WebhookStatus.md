# WebhookStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_url** | Option<**String**> |  | [optional]
**branches** | [**Vec<models::WebhookStatusBranchesInner>**](webhook_status_branches_inner.md) | An array of branch objects containing the status' SHA. Each branch contains the given SHA, but the SHA may or may not be the head of the branch. The array includes a maximum of 10 branches. | 
**commit** | [**models::WebhookStatusCommit**](webhook_status_commit.md) |  | 
**context** | **String** |  | 
**created_at** | **String** |  | 
**description** | Option<**String**> | The optional human-readable description added to the status. | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**id** | **i32** | The unique identifier of the status. | 
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**name** | **String** |  | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 
**sha** | **String** | The Commit SHA. | 
**state** | **String** | The new state. Can be `pending`, `success`, `failure`, or `error`. | 
**target_url** | Option<**String**> | The optional link added to the status. | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


