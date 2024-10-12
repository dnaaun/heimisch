# HookDelivery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier of the delivery. | 
**guid** | **String** | Unique identifier for the event (shared with all deliveries for all webhooks that subscribe to this event). | 
**delivered_at** | **String** | Time when the delivery was delivered. | 
**redelivery** | **bool** | Whether the delivery is a redelivery. | 
**duration** | **f64** | Time spent delivering. | 
**status** | **String** | Description of the status of the attempted delivery | 
**status_code** | **i32** | Status code received when delivery was made. | 
**event** | **String** | The event that triggered the delivery. | 
**action** | Option<**String**> | The type of activity for the event that triggered the delivery. | 
**installation_id** | Option<**i32**> | The id of the GitHub App installation associated with this event. | 
**repository_id** | Option<**i32**> | The id of the repository associated with this event. | 
**throttled_at** | Option<**String**> | Time when the webhook delivery was throttled. | [optional]
**url** | Option<**String**> | The URL target of the delivery. | [optional]
**request** | [**models::HookDeliveryRequest**](hook_delivery_request.md) |  | 
**response** | [**models::HookDeliveryResponse**](hook_delivery_response.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


