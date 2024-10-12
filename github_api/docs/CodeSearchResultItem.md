# CodeSearchResultItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**path** | **String** |  | 
**sha** | **String** |  | 
**url** | **String** |  | 
**git_url** | **String** |  | 
**html_url** | **String** |  | 
**repository** | [**models::MinimalRepository**](minimal-repository.md) |  | 
**score** | **f64** |  | 
**file_size** | Option<**i32**> |  | [optional]
**language** | Option<**String**> |  | [optional]
**last_modified_at** | Option<**String**> |  | [optional]
**line_numbers** | Option<**Vec<String>**> |  | [optional]
**text_matches** | Option<[**Vec<models::SearchResultTextMatchesInner>**](search_result_text_matches_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


