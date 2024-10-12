# ReposCreateDispatchEventRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_type** | **String** | A custom webhook event name. Must be 100 characters or fewer. | 
**client_payload** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | JSON payload with extra information about the webhook event that your action or workflow may use. The maximum number of top-level properties is 10. The total size of the JSON payload must be less than 64KB. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


