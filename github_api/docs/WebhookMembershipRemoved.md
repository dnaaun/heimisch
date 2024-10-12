# WebhookMembershipRemoved

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**member** | Option<[**models::WebhooksUser**](webhooks_user.md)> |  | 
**organization** | [**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md) |  | 
**repository** | Option<[**models::RepositoryWebhooks**](repository-webhooks.md)> |  | [optional]
**scope** | **String** | The scope of the membership. Currently, can only be `team`. | 
**sender** | Option<[**models::User2**](User_2.md)> |  | 
**team** | [**models::WebhooksTeam**](webhooks_team.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


