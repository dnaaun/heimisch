# ActionsGenerateRunnerJitconfigForOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new runner. | 
**runner_group_id** | **i32** | The ID of the runner group to register the runner to. | 
**labels** | **Vec<String>** | The names of the custom labels to add to the runner. **Minimum items**: 1. **Maximum items**: 100. | 
**work_folder** | Option<**String**> | The working directory to be used for job execution, relative to the runner install directory. | [optional][default to _work]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


