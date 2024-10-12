# ReposCreateOrUpdateFileContentsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | The commit message. | 
**content** | **String** | The new file content, using Base64 encoding. | 
**sha** | Option<**String**> | **Required if you are updating a file**. The blob SHA of the file being replaced. | [optional]
**branch** | Option<**String**> | The branch name. Default: the repository’s default branch. | [optional]
**committer** | Option<[**models::ReposCreateOrUpdateFileContentsRequestCommitter**](repos_create_or_update_file_contents_request_committer.md)> |  | [optional]
**author** | Option<[**models::ReposCreateOrUpdateFileContentsRequestAuthor**](repos_create_or_update_file_contents_request_author.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


