# WebhookConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_type** | Option<**String**> | The media type used to serialize the payloads. Supported values include `json` and `form`. The default is `form`. | [optional]
**insecure_ssl** | Option<[**models::WebhookConfigInsecureSsl**](webhook-config-insecure-ssl.md)> |  | [optional]
**secret** | Option<**String**> | If provided, the `secret` will be used as the `key` to generate the HMAC hex digest value for [delivery signature headers](https://docs.github.com/webhooks/event-payloads/#delivery-headers). | [optional]
**url** | Option<**String**> | The URL to which the payloads will be delivered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


