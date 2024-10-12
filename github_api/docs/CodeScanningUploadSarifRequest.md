# CodeScanningUploadSarifRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**commit_sha** | **String** | The SHA of the commit to which the analysis you are uploading relates. | 
**r#ref** | **String** | The full Git reference, formatted as `refs/heads/<branch name>`, `refs/tags/<tag>`, `refs/pull/<number>/merge`, or `refs/pull/<number>/head`. | 
**sarif** | **String** | A Base64 string representing the SARIF file to upload. You must first compress your SARIF file using [`gzip`](http://www.gnu.org/software/gzip/manual/gzip.html) and then translate the contents of the file into a Base64 encoding string. For more information, see \"[SARIF support for code scanning](https://docs.github.com/code-security/secure-coding/sarif-support-for-code-scanning).\" | 
**checkout_uri** | Option<**String**> | The base directory used in the analysis, as it appears in the SARIF file. This property is used to convert file paths from absolute to relative, so that alerts can be mapped to their correct location in the repository. | [optional]
**started_at** | Option<**String**> | The time that the analysis run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional]
**tool_name** | Option<**String**> | The name of the tool used to generate the code scanning analysis. If this parameter is not used, the tool name defaults to \"API\". If the uploaded SARIF contains a tool GUID, this will be available for filtering using the `tool_guid` parameter of operations such as `GET /repos/{owner}/{repo}/code-scanning/alerts`. | [optional]
**validate** | Option<**bool**> | Whether the SARIF file will be validated according to the code scanning specifications. This parameter is intended to help integrators ensure that the uploaded SARIF files are correctly rendered by code scanning. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


