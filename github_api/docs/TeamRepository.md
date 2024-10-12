# TeamRepository

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier of the repository | 
**node_id** | **String** |  | 
**name** | **String** | The name of the repository. | 
**full_name** | **String** |  | 
**license** | Option<[**models::NullableLicenseSimple**](nullable-license-simple.md)> |  | 
**forks** | **i32** |  | 
**permissions** | Option<[**models::RepositoryPermissions**](repository_permissions.md)> |  | [optional]
**role_name** | Option<**String**> |  | [optional]
**owner** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**private** | **bool** | Whether the repository is private or public. | [default to false]
**html_url** | **String** |  | 
**description** | Option<**String**> |  | 
**fork** | **bool** |  | 
**url** | **String** |  | 
**archive_url** | **String** |  | 
**assignees_url** | **String** |  | 
**blobs_url** | **String** |  | 
**branches_url** | **String** |  | 
**collaborators_url** | **String** |  | 
**comments_url** | **String** |  | 
**commits_url** | **String** |  | 
**compare_url** | **String** |  | 
**contents_url** | **String** |  | 
**contributors_url** | **String** |  | 
**deployments_url** | **String** |  | 
**downloads_url** | **String** |  | 
**events_url** | **String** |  | 
**forks_url** | **String** |  | 
**git_commits_url** | **String** |  | 
**git_refs_url** | **String** |  | 
**git_tags_url** | **String** |  | 
**git_url** | **String** |  | 
**issue_comment_url** | **String** |  | 
**issue_events_url** | **String** |  | 
**issues_url** | **String** |  | 
**keys_url** | **String** |  | 
**labels_url** | **String** |  | 
**languages_url** | **String** |  | 
**merges_url** | **String** |  | 
**milestones_url** | **String** |  | 
**notifications_url** | **String** |  | 
**pulls_url** | **String** |  | 
**releases_url** | **String** |  | 
**ssh_url** | **String** |  | 
**stargazers_url** | **String** |  | 
**statuses_url** | **String** |  | 
**subscribers_url** | **String** |  | 
**subscription_url** | **String** |  | 
**tags_url** | **String** |  | 
**teams_url** | **String** |  | 
**trees_url** | **String** |  | 
**clone_url** | **String** |  | 
**mirror_url** | Option<**String**> |  | 
**hooks_url** | **String** |  | 
**svn_url** | **String** |  | 
**homepage** | Option<**String**> |  | 
**language** | Option<**String**> |  | 
**forks_count** | **i32** |  | 
**stargazers_count** | **i32** |  | 
**watchers_count** | **i32** |  | 
**size** | **i32** |  | 
**default_branch** | **String** | The default branch of the repository. | 
**open_issues_count** | **i32** |  | 
**is_template** | Option<**bool**> | Whether this repository acts as a template that can be used to generate new repositories. | [optional][default to false]
**topics** | Option<**Vec<String>**> |  | [optional]
**has_issues** | **bool** | Whether issues are enabled. | [default to true]
**has_projects** | **bool** | Whether projects are enabled. | [default to true]
**has_wiki** | **bool** | Whether the wiki is enabled. | [default to true]
**has_pages** | **bool** |  | 
**has_downloads** | **bool** | Whether downloads are enabled. | [default to true]
**archived** | **bool** | Whether the repository is archived. | [default to false]
**disabled** | **bool** | Returns whether or not this repository disabled. | 
**visibility** | Option<**String**> | The repository visibility: public, private, or internal. | [optional][default to public]
**pushed_at** | Option<**String**> |  | 
**created_at** | Option<**String**> |  | 
**updated_at** | Option<**String**> |  | 
**allow_rebase_merge** | Option<**bool**> | Whether to allow rebase merges for pull requests. | [optional][default to true]
**temp_clone_token** | Option<**String**> |  | [optional]
**allow_squash_merge** | Option<**bool**> | Whether to allow squash merges for pull requests. | [optional][default to true]
**allow_auto_merge** | Option<**bool**> | Whether to allow Auto-merge to be used on pull requests. | [optional][default to false]
**delete_branch_on_merge** | Option<**bool**> | Whether to delete head branches when pull requests are merged | [optional][default to false]
**allow_merge_commit** | Option<**bool**> | Whether to allow merge commits for pull requests. | [optional][default to true]
**allow_forking** | Option<**bool**> | Whether to allow forking this repo | [optional][default to false]
**web_commit_signoff_required** | Option<**bool**> | Whether to require contributors to sign off on web-based commits | [optional][default to false]
**subscribers_count** | Option<**i32**> |  | [optional]
**network_count** | Option<**i32**> |  | [optional]
**open_issues** | **i32** |  | 
**watchers** | **i32** |  | 
**master_branch** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


