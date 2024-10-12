# Hook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**id** | **i32** | Unique identifier of the webhook. | 
**name** | **String** | The name of a valid service, use 'web' for a webhook. | 
**active** | **bool** | Determines whether the hook is actually triggered on pushes. | 
**events** | **Vec<String>** | Determines what events the hook is triggered for. Default: ['push']. | 
**config** | [**models::WebhookConfig**](webhook-config.md) |  | 
**updated_at** | **String** |  | 
**created_at** | **String** |  | 
**url** | **String** |  | 
**test_url** | **String** |  | 
**ping_url** | **String** |  | 
**deliveries_url** | Option<**String**> |  | [optional]
**last_response** | [**models::HookResponse**](hook-response.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


