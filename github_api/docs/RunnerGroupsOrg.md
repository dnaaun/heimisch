# RunnerGroupsOrg

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **f64** |  | 
**name** | **String** |  | 
**visibility** | **String** |  | 
**default** | **bool** |  | 
**selected_repositories_url** | Option<**String**> | Link to the selected repositories resource for this runner group. Not present unless visibility was set to `selected` | [optional]
**runners_url** | **String** |  | 
**hosted_runners_url** | Option<**String**> |  | [optional]
**inherited** | **bool** |  | 
**inherited_allows_public_repositories** | Option<**bool**> |  | [optional]
**allows_public_repositories** | **bool** |  | 
**workflow_restrictions_read_only** | Option<**bool**> | If `true`, the `restricted_to_workflows` and `selected_workflows` fields cannot be modified. | [optional][default to false]
**restricted_to_workflows** | Option<**bool**> | If `true`, the runner group will be restricted to running only the workflows specified in the `selected_workflows` array. | [optional][default to false]
**selected_workflows** | Option<**Vec<String>**> | List of workflows the runner group should be allowed to run. This setting will be ignored unless `restricted_to_workflows` is set to `true`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


