# RepositoryRulesetConditionsRepositoryNameTargetRepositoryName

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include** | Option<**Vec<String>**> | Array of repository names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~ALL` to include all repositories. | [optional]
**exclude** | Option<**Vec<String>**> | Array of repository names or patterns to exclude. The condition will not pass if any of these patterns match. | [optional]
**protected** | Option<**bool**> | Whether renaming of target repositories is prevented. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


