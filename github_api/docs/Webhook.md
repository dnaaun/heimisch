# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** | Determines whether the hook is actually triggered for the events it subscribes to. | 
**app_id** | Option<**i32**> | Only included for GitHub Apps. When you register a new GitHub App, GitHub sends a ping event to the webhook URL you specified during registration. The GitHub App ID sent in this field is required for authenticating an app. | [optional]
**config** | [**models::WebhookConfig**](Webhook_config.md) |  | 
**created_at** | **String** |  | 
**deliveries_url** | Option<**String**> |  | [optional]
**events** | **Vec<String>** | Determines what events the hook is triggered for. Default: ['push']. | 
**id** | **i32** | Unique identifier of the webhook. | 
**last_response** | Option<[**models::HookResponse**](hook-response.md)> |  | [optional]
**name** | **String** | The type of webhook. The only valid value is 'web'. | 
**ping_url** | Option<**String**> |  | [optional]
**test_url** | Option<**String**> |  | [optional]
**r#type** | **String** |  | 
**updated_at** | **String** |  | 
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


