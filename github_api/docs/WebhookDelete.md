# WebhookDelete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**pusher_type** | **String** | The pusher type for the event. Can be either `user` or a deploy key. | 
**r#ref** | **String** | The [`git ref`](https://docs.github.com/rest/git/refs#get-a-reference) resource. | 
**ref_type** | **String** | The type of Git ref object deleted in the repository. | 
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


