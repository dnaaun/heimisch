# SecretScanningLocationCommit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | **String** | The file path in the repository | 
**start_line** | **f64** | Line number at which the secret starts in the file | 
**end_line** | **f64** | Line number at which the secret ends in the file | 
**start_column** | **f64** | The column at which the secret starts within the start line when the file is interpreted as 8BIT ASCII | 
**end_column** | **f64** | The column at which the secret ends within the end line when the file is interpreted as 8BIT ASCII | 
**blob_sha** | **String** | SHA-1 hash ID of the associated blob | 
**blob_url** | **String** | The API URL to get the associated blob resource | 
**commit_sha** | **String** | SHA-1 hash ID of the associated commit | 
**commit_url** | **String** | The API URL to get the associated commit resource | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


