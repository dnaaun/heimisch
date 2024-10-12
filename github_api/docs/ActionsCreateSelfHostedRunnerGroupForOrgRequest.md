# ActionsCreateSelfHostedRunnerGroupForOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the runner group. | 
**visibility** | Option<**String**> | Visibility of a runner group. You can select all repositories, select individual repositories, or limit access to private repositories. | [optional][default to All]
**selected_repository_ids** | Option<**Vec<i32>**> | List of repository IDs that can access the runner group. | [optional]
**runners** | Option<**Vec<i32>**> | List of runner IDs to add to the runner group. | [optional]
**allows_public_repositories** | Option<**bool**> | Whether the runner group can be used by `public` repositories. | [optional][default to false]
**restricted_to_workflows** | Option<**bool**> | If `true`, the runner group will be restricted to running only the workflows specified in the `selected_workflows` array. | [optional][default to false]
**selected_workflows** | Option<**Vec<String>**> | List of workflows the runner group should be allowed to run. This setting will be ignored unless `restricted_to_workflows` is set to `true`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


