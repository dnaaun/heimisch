# CodeScanningVariantAnalysisScannedRepositoriesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**repository** | [**models::CodeScanningVariantAnalysisRepository**](code-scanning-variant-analysis-repository.md) |  | 
**analysis_status** | [**models::CodeScanningVariantAnalysisStatus**](code-scanning-variant-analysis-status.md) |  | 
**result_count** | Option<**i32**> | The number of results in the case of a successful analysis. This is only available for successful analyses. | [optional]
**artifact_size_in_bytes** | Option<**i32**> | The size of the artifact. This is only available for successful analyses. | [optional]
**failure_message** | Option<**String**> | The reason of the failure of this repo task. This is only available if the repository task has failed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


