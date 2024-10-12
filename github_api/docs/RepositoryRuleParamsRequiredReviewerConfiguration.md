# RepositoryRuleParamsRequiredReviewerConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_patterns** | **Vec<String>** | Array of file patterns. Pull requests which change matching files must be approved by the specified team. File patterns use the same syntax as `.gitignore` files. | 
**minimum_approvals** | **i32** | Minimum number of approvals required from the specified team. If set to zero, the team will be added to the pull request but approval is optional. | 
**reviewer_id** | **String** | Node ID of the team which must review changes to matching files. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


