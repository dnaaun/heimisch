# OrgsUpdateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**models::OrgsUpdateWebhookRequestConfig**](orgs_update_webhook_request_config.md)> |  | [optional]
**events** | Option<**Vec<String>**> | Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. | [optional][default to ["push"]]
**active** | Option<**bool**> | Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications. | [optional][default to true]
**name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


