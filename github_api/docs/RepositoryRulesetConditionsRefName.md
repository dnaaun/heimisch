# RepositoryRulesetConditionsRefName

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include** | Option<**Vec<String>**> | Array of ref names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~DEFAULT_BRANCH` to include the default branch or `~ALL` to include all branches. | [optional]
**exclude** | Option<**Vec<String>**> | Array of ref names or patterns to exclude. The condition will not pass if any of these patterns match. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


