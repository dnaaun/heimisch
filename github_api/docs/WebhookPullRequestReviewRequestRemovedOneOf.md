# WebhookPullRequestReviewRequestRemovedOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**number** | **i32** | The pull request number. | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**pull_request** | [**models::PullRequest6**](Pull_Request_6.md) |  | 
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**requested_reviewer** | Option<[**models::User2**](User_2.md)> |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


