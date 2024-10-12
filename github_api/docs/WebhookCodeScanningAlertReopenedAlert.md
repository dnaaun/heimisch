# WebhookCodeScanningAlertReopenedAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.` | 
**dismissed_at** | Option<**String**> | The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | 
**dismissed_by** | Option<[**serde_json::Value**](.md)> |  | 
**dismissed_reason** | Option<**String**> | The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`. | 
**html_url** | **String** | The GitHub URL of the alert resource. | 
**most_recent_instance** | Option<[**models::AlertInstance**](Alert_Instance.md)> |  | [optional]
**number** | **i32** | The code scanning alert number. | 
**rule** | [**models::WebhookCodeScanningAlertClosedByUserAlertRule**](webhook_code_scanning_alert_closed_by_user_alert_rule.md) |  | 
**state** | **String** | State of a code scanning alert. | 
**tool** | [**models::WebhookCodeScanningAlertClosedByUserAlertTool**](webhook_code_scanning_alert_closed_by_user_alert_tool.md) |  | 
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


