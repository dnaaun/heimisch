# WebhookRepositoryDispatchSample

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | The `event_type` that was specified in the `POST /repos/{owner}/{repo}/dispatches` request body. | 
**branch** | **String** |  | 
**client_payload** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The `client_payload` that was specified in the `POST /repos/{owner}/{repo}/dispatches` request body. | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | [**models::SimpleInstallation**](simple-installation.md) |  | 
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


