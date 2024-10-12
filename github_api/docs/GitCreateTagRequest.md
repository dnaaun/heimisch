# GitCreateTagRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag** | **String** | The tag's name. This is typically a version (e.g., \"v0.0.1\"). | 
**message** | **String** | The tag message. | 
**object** | **String** | The SHA of the git object this is tagging. | 
**r#type** | **String** | The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`. | 
**tagger** | Option<[**models::GitCreateTagRequestTagger**](git_create_tag_request_tagger.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


