# ActionsCreateOrgVariableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the variable. | 
**value** | **String** | The value of the variable. | 
**visibility** | **String** | The type of repositories in the organization that can access the variable. `selected` means only the repositories specified by `selected_repository_ids` can access the variable. | 
**selected_repository_ids** | Option<**Vec<i32>**> | An array of repository ids that can access the organization variable. You can only provide a list of repository ids when the `visibility` is set to `selected`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


