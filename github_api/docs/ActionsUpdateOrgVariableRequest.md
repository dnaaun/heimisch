# ActionsUpdateOrgVariableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the variable. | [optional]
**value** | Option<**String**> | The value of the variable. | [optional]
**visibility** | Option<**String**> | The type of repositories in the organization that can access the variable. `selected` means only the repositories specified by `selected_repository_ids` can access the variable. | [optional]
**selected_repository_ids** | Option<**Vec<i32>**> | An array of repository ids that can access the organization variable. You can only provide a list of repository ids when the `visibility` is set to `selected`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


