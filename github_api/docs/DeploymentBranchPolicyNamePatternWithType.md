# DeploymentBranchPolicyNamePatternWithType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name pattern that branches or tags must match in order to deploy to the environment.  Wildcard characters will not match `/`. For example, to match branches that begin with `release/` and contain an additional single slash, use `release/_*_/_*`. For more information about pattern matching syntax, see the [Ruby File.fnmatch documentation](https://ruby-doc.org/core-2.5.1/File.html#method-c-fnmatch). | 
**r#type** | Option<**String**> | Whether this rule targets a branch or tag | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


