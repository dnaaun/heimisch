# SecretScanningLocationWikiCommit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | **String** | The file path of the wiki page | 
**start_line** | **f64** | Line number at which the secret starts in the file | 
**end_line** | **f64** | Line number at which the secret ends in the file | 
**start_column** | **f64** | The column at which the secret starts within the start line when the file is interpreted as 8-bit ASCII. | 
**end_column** | **f64** | The column at which the secret ends within the end line when the file is interpreted as 8-bit ASCII. | 
**blob_sha** | **String** | SHA-1 hash ID of the associated blob | 
**page_url** | **String** | The GitHub URL to get the associated wiki page | 
**commit_sha** | **String** | SHA-1 hash ID of the associated commit | 
**commit_url** | **String** | The GitHub URL to get the associated wiki commit | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


