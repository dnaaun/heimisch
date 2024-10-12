# CodeScanningAlertInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#ref** | Option<**String**> | The Git reference, formatted as `refs/pull/<number>/merge`, `refs/pull/<number>/head`, `refs/heads/<branch name>` or simply `<branch name>`. | [optional]
**analysis_key** | Option<**String**> | Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name. | [optional]
**environment** | Option<**String**> | Identifies the variable values associated with the environment in which the analysis that generated this alert instance was performed, such as the language that was analyzed. | [optional]
**category** | Option<**String**> | Identifies the configuration under which the analysis was executed. Used to distinguish between multiple analyses for the same tool and commit, but performed on different languages or different parts of the code. | [optional]
**state** | Option<[**models::CodeScanningAlertState**](code-scanning-alert-state.md)> |  | [optional]
**commit_sha** | Option<**String**> |  | [optional]
**message** | Option<[**models::CodeScanningAlertInstanceMessage**](code_scanning_alert_instance_message.md)> |  | [optional]
**location** | Option<[**models::CodeScanningAlertLocation**](code-scanning-alert-location.md)> |  | [optional]
**html_url** | Option<**String**> |  | [optional]
**classifications** | Option<[**Vec<models::CodeScanningAlertClassification>**](code-scanning-alert-classification.md)> | Classifications that have been applied to the file that triggered the alert. For example identifying it as documentation, or a generated file. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


