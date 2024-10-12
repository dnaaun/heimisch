# CodeScanningVariantAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The ID of the variant analysis. | 
**controller_repo** | [**models::SimpleRepository**](simple-repository.md) |  | 
**actor** | [**models::SimpleUser**](simple-user.md) |  | 
**query_language** | [**models::CodeScanningVariantAnalysisLanguage**](code-scanning-variant-analysis-language.md) |  | 
**query_pack_url** | **String** | The download url for the query pack. | 
**created_at** | Option<**String**> | The date and time at which the variant analysis was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ. | [optional]
**updated_at** | Option<**String**> | The date and time at which the variant analysis was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ. | [optional]
**completed_at** | Option<**String**> | The date and time at which the variant analysis was completed, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ. Will be null if the variant analysis has not yet completed or this information is not available. | [optional]
**status** | **String** |  | 
**actions_workflow_run_id** | Option<**i32**> | The GitHub Actions workflow run used to execute this variant analysis. This is only available if the workflow run has started. | [optional]
**failure_reason** | Option<**String**> | The reason for a failure of the variant analysis. This is only available if the variant analysis has failed. | [optional]
**scanned_repositories** | Option<[**Vec<models::CodeScanningVariantAnalysisScannedRepositoriesInner>**](code_scanning_variant_analysis_scanned_repositories_inner.md)> |  | [optional]
**skipped_repositories** | Option<[**models::CodeScanningVariantAnalysisSkippedRepositories**](code_scanning_variant_analysis_skipped_repositories.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


