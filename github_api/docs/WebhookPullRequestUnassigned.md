# WebhookPullRequestUnassigned

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**assignee** | Option<[**models::WebhooksUserMannequin**](webhooks_user_mannequin.md)> |  | [optional]
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**number** | **i32** | The pull request number. | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**pull_request** | [**models::PullRequest11**](Pull_Request_11.md) |  | 
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**sender** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


