# CodeScanningAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#ref** | **String** | The Git reference, formatted as `refs/pull/<number>/merge`, `refs/pull/<number>/head`, `refs/heads/<branch name>` or simply `<branch name>`. | 
**commit_sha** | **String** | The SHA of the commit to which the analysis you are uploading relates. | 
**analysis_key** | **String** | Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name. | 
**environment** | **String** | Identifies the variable values associated with the environment in which this analysis was performed. | 
**category** | Option<**String**> | Identifies the configuration under which the analysis was executed. Used to distinguish between multiple analyses for the same tool and commit, but performed on different languages or different parts of the code. | [optional]
**error** | **String** |  | 
**created_at** | **String** | The time that the analysis was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**results_count** | **i32** | The total number of results in the analysis. | 
**rules_count** | **i32** | The total number of rules used in the analysis. | 
**id** | **i32** | Unique identifier for this analysis. | 
**url** | **String** | The REST API URL of the analysis resource. | [readonly]
**sarif_id** | **String** | An identifier for the upload. | 
**tool** | [**models::CodeScanningAnalysisTool**](code-scanning-analysis-tool.md) |  | 
**deletable** | **bool** |  | 
**warning** | **String** | Warning generated when processing the analysis | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


