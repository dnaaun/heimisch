# WebhookInstallationRepositoriesRemoved

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | [**models::Installation**](installation.md) |  | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**repositories_added** | [**Vec<models::WebhooksRepositoriesInner>**](webhooks_repositories_inner.md) | An array of repository objects, which were added to the installation. | 
**repositories_removed** | [**Vec<models::WebhooksRepositoriesInner>**](webhooks_repositories_inner.md) | An array of repository objects, which were removed from the installation. | 
**repository** | Option<[**models::RepositoryWebhooks**](repository-webhooks.md)> |  | [optional]
**repository_selection** | [**models::WebhooksRepositorySelection**](webhooks_repository_selection.md) |  | 
**requester** | Option<[**models::WebhooksUser**](webhooks_user.md)> |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


