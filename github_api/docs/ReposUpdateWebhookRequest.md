# ReposUpdateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**models::WebhookConfig**](webhook-config.md)> |  | [optional]
**events** | Option<**Vec<String>**> | Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. This replaces the entire array of events. | [optional][default to ["push"]]
**add_events** | Option<**Vec<String>**> | Determines a list of events to be added to the list of events that the Hook triggers for. | [optional]
**remove_events** | Option<**Vec<String>**> | Determines a list of events to be removed from the list of events that the Hook triggers for. | [optional]
**active** | Option<**bool**> | Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


