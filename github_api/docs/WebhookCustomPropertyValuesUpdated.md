# WebhookCustomPropertyValuesUpdated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**organization** | [**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md) |  | 
**sender** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]
**new_property_values** | [**Vec<models::CustomPropertyValue>**](custom-property-value.md) | The new custom property values for the repository. | 
**old_property_values** | [**Vec<models::CustomPropertyValue>**](custom-property-value.md) | The old custom property values for the repository. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


