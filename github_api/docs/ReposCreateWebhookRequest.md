# ReposCreateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Use `web` to create a webhook. Default: `web`. This parameter only accepts the value `web`. | [optional]
**config** | Option<[**models::ReposCreateWebhookRequestConfig**](repos_create_webhook_request_config.md)> |  | [optional]
**events** | Option<**Vec<String>**> | Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. | [optional][default to ["push"]]
**active** | Option<**bool**> | Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


