# ActionsUpdateSelfHostedRunnerGroupForOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the runner group. | 
**visibility** | Option<**String**> | Visibility of a runner group. You can select all repositories, select individual repositories, or all private repositories. | [optional]
**allows_public_repositories** | Option<**bool**> | Whether the runner group can be used by `public` repositories. | [optional][default to false]
**restricted_to_workflows** | Option<**bool**> | If `true`, the runner group will be restricted to running only the workflows specified in the `selected_workflows` array. | [optional][default to false]
**selected_workflows** | Option<**Vec<String>**> | List of workflows the runner group should be allowed to run. This setting will be ignored unless `restricted_to_workflows` is set to `true`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


