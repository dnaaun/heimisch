# GistsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the gist. | [optional]
**files** | Option<[**std::collections::HashMap<String, models::GistsUpdateRequestFilesValue>**](gists_update_request_files_value.md)> | The gist files to be updated, renamed, or deleted. Each `key` must match the current filename (including extension) of the targeted gist file. For example: `hello.py`.  To delete a file, set the whole file to null. For example: `hello.py : null`. The file will also be deleted if the specified object does not contain at least one of `content` or `filename`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


