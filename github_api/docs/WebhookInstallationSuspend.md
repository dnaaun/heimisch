# WebhookInstallationSuspend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | [**models::Installation**](installation.md) |  | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**repositories** | Option<[**Vec<models::WebhooksRepositoriesInner>**](webhooks_repositories_inner.md)> | An array of repository objects that the installation can access. | [optional]
**repository** | Option<[**models::RepositoryWebhooks**](repository-webhooks.md)> |  | [optional]
**requester** | Option<[**serde_json::Value**](.md)> |  | [optional]
**sender** | [**models::SimpleUser**](simple-user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


