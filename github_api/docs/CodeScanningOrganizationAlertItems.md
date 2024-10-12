# CodeScanningOrganizationAlertItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number** | **i32** | The security alert number. | [readonly]
**created_at** | **String** | The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**updated_at** | Option<**String**> | The time that the alert was last updated in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional][readonly]
**url** | **String** | The REST API URL of the alert resource. | [readonly]
**html_url** | **String** | The GitHub URL of the alert resource. | [readonly]
**instances_url** | **String** | The REST API URL for fetching the list of instances for an alert. | [readonly]
**state** | Option<[**models::CodeScanningAlertState**](code-scanning-alert-state.md)> |  | 
**fixed_at** | Option<**String**> | The time that the alert was no longer detected and was considered fixed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional][readonly]
**dismissed_by** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**dismissed_at** | Option<**String**> | The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**dismissed_reason** | Option<[**models::CodeScanningAlertDismissedReason**](code-scanning-alert-dismissed-reason.md)> |  | 
**dismissed_comment** | Option<**String**> | The dismissal comment associated with the dismissal of the alert. | [optional]
**rule** | [**models::CodeScanningAlertRuleSummary**](code-scanning-alert-rule-summary.md) |  | 
**tool** | [**models::CodeScanningAnalysisTool**](code-scanning-analysis-tool.md) |  | 
**most_recent_instance** | [**models::CodeScanningAlertInstance**](code-scanning-alert-instance.md) |  | 
**repository** | [**models::SimpleRepository**](simple-repository.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


