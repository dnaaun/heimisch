# CodeScanningCreateVariantAnalysisRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**language** | [**models::CodeScanningVariantAnalysisLanguage**](code-scanning-variant-analysis-language.md) |  | 
**query_pack** | **String** | A Base64-encoded tarball containing a CodeQL query and all its dependencies | 
**repositories** | Option<**Vec<String>**> | List of repository names (in the form `owner/repo-name`) to run the query against. Precisely one property from `repositories`, `repository_lists` and `repository_owners` is required. | [optional]
**repository_lists** | Option<**Vec<String>**> | List of repository lists to run the query against. Precisely one property from `repositories`, `repository_lists` and `repository_owners` is required. | [optional]
**repository_owners** | Option<**Vec<String>**> | List of organization or user names whose repositories the query should be run against. Precisely one property from `repositories`, `repository_lists` and `repository_owners` is required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


