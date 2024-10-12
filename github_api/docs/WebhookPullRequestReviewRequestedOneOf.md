# WebhookPullRequestReviewRequestedOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**number** | **i32** | The pull request number. | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**pull_request** | [**models::PullRequest8**](Pull_Request_8.md) |  | 
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**requested_reviewer** | Option<[**models::User4**](User_4.md)> |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


