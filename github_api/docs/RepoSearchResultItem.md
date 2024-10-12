# RepoSearchResultItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**name** | **String** |  | 
**full_name** | **String** |  | 
**owner** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**private** | **bool** |  | 
**html_url** | **String** |  | 
**description** | Option<**String**> |  | 
**fork** | **bool** |  | 
**url** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**pushed_at** | **String** |  | 
**homepage** | Option<**String**> |  | 
**size** | **i32** |  | 
**stargazers_count** | **i32** |  | 
**watchers_count** | **i32** |  | 
**language** | Option<**String**> |  | 
**forks_count** | **i32** |  | 
**open_issues_count** | **i32** |  | 
**master_branch** | Option<**String**> |  | [optional]
**default_branch** | **String** |  | 
**score** | **f64** |  | 
**forks_url** | **String** |  | 
**keys_url** | **String** |  | 
**collaborators_url** | **String** |  | 
**teams_url** | **String** |  | 
**hooks_url** | **String** |  | 
**issue_events_url** | **String** |  | 
**events_url** | **String** |  | 
**assignees_url** | **String** |  | 
**branches_url** | **String** |  | 
**tags_url** | **String** |  | 
**blobs_url** | **String** |  | 
**git_tags_url** | **String** |  | 
**git_refs_url** | **String** |  | 
**trees_url** | **String** |  | 
**statuses_url** | **String** |  | 
**languages_url** | **String** |  | 
**stargazers_url** | **String** |  | 
**contributors_url** | **String** |  | 
**subscribers_url** | **String** |  | 
**subscription_url** | **String** |  | 
**commits_url** | **String** |  | 
**git_commits_url** | **String** |  | 
**comments_url** | **String** |  | 
**issue_comment_url** | **String** |  | 
**contents_url** | **String** |  | 
**compare_url** | **String** |  | 
**merges_url** | **String** |  | 
**archive_url** | **String** |  | 
**downloads_url** | **String** |  | 
**issues_url** | **String** |  | 
**pulls_url** | **String** |  | 
**milestones_url** | **String** |  | 
**notifications_url** | **String** |  | 
**labels_url** | **String** |  | 
**releases_url** | **String** |  | 
**deployments_url** | **String** |  | 
**git_url** | **String** |  | 
**ssh_url** | **String** |  | 
**clone_url** | **String** |  | 
**svn_url** | **String** |  | 
**forks** | **i32** |  | 
**open_issues** | **i32** |  | 
**watchers** | **i32** |  | 
**topics** | Option<**Vec<String>**> |  | [optional]
**mirror_url** | Option<**String**> |  | 
**has_issues** | **bool** |  | 
**has_projects** | **bool** |  | 
**has_pages** | **bool** |  | 
**has_wiki** | **bool** |  | 
**has_downloads** | **bool** |  | 
**has_discussions** | Option<**bool**> |  | [optional]
**archived** | **bool** |  | 
**disabled** | **bool** | Returns whether or not this repository disabled. | 
**visibility** | Option<**String**> | The repository visibility: public, private, or internal. | [optional]
**license** | Option<[**models::NullableLicenseSimple**](nullable-license-simple.md)> |  | 
**permissions** | Option<[**models::FullRepositoryPermissions**](full_repository_permissions.md)> |  | [optional]
**text_matches** | Option<[**Vec<models::SearchResultTextMatchesInner>**](search_result_text_matches_inner.md)> |  | [optional]
**temp_clone_token** | Option<**String**> |  | [optional]
**allow_merge_commit** | Option<**bool**> |  | [optional]
**allow_squash_merge** | Option<**bool**> |  | [optional]
**allow_rebase_merge** | Option<**bool**> |  | [optional]
**allow_auto_merge** | Option<**bool**> |  | [optional]
**delete_branch_on_merge** | Option<**bool**> |  | [optional]
**allow_forking** | Option<**bool**> |  | [optional]
**is_template** | Option<**bool**> |  | [optional]
**web_commit_signoff_required** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


