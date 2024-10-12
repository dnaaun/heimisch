# OrgsCreateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Must be passed as \"web\". | 
**config** | [**models::OrgsCreateWebhookRequestConfig**](orgs_create_webhook_request_config.md) |  | 
**events** | Option<**Vec<String>**> | Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. Set to `[\"*\"]` to receive all possible events. | [optional][default to ["push"]]
**active** | Option<**bool**> | Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


