# CommitSearchResultItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**sha** | **String** |  | 
**html_url** | **String** |  | 
**comments_url** | **String** |  | 
**commit** | [**models::CommitSearchResultItemCommit**](commit_search_result_item_commit.md) |  | 
**author** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**committer** | Option<[**models::NullableGitUser**](nullable-git-user.md)> |  | 
**parents** | [**Vec<models::FileCommitCommitParentsInner>**](file_commit_commit_parents_inner.md) |  | 
**repository** | [**models::MinimalRepository**](minimal-repository.md) |  | 
**score** | **f64** |  | 
**node_id** | **String** |  | 
**text_matches** | Option<[**Vec<models::SearchResultTextMatchesInner>**](search_result_text_matches_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


