# AlertInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**analysis_key** | **String** | Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name. | 
**category** | Option<**String**> | Identifies the configuration under which the analysis was executed. | [optional]
**classifications** | Option<**Vec<String>**> |  | [optional]
**commit_sha** | Option<**String**> |  | [optional]
**environment** | **String** | Identifies the variable values associated with the environment in which the analysis that generated this alert instance was performed, such as the language that was analyzed. | 
**location** | Option<[**models::AlertInstanceLocation**](Alert_Instance_location.md)> |  | [optional]
**message** | Option<[**models::CodeScanningAlertInstanceMessage**](code_scanning_alert_instance_message.md)> |  | [optional]
**r#ref** | **String** | The full Git reference, formatted as `refs/heads/<branch name>`. | 
**state** | **String** | State of a code scanning alert. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


