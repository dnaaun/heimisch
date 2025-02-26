# Repository1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_auto_merge** | Option<**bool**> | Whether to allow auto-merge for pull requests. | [optional][default to false]
**allow_forking** | Option<**bool**> | Whether to allow private forks | [optional]
**allow_merge_commit** | Option<**bool**> | Whether to allow merge commits for pull requests. | [optional][default to true]
**allow_rebase_merge** | Option<**bool**> | Whether to allow rebase merges for pull requests. | [optional][default to true]
**allow_squash_merge** | Option<**bool**> | Whether to allow squash merges for pull requests. | [optional][default to true]
**allow_update_branch** | Option<**bool**> |  | [optional]
**archive_url** | **String** |  | 
**archived** | **bool** | Whether the repository is archived. | [default to false]
**assignees_url** | **String** |  | 
**blobs_url** | **String** |  | 
**branches_url** | **String** |  | 
**clone_url** | **String** |  | 
**collaborators_url** | **String** |  | 
**comments_url** | **String** |  | 
**commits_url** | **String** |  | 
**compare_url** | **String** |  | 
**contents_url** | **String** |  | 
**contributors_url** | **String** |  | 
**created_at** | [**models::RepositoryCreatedAt**](Repository_created_at.md) |  | 
**custom_properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The custom properties that were defined for the repository. The keys are the custom property names, and the values are the corresponding custom property values. | [optional]
**default_branch** | **String** | The default branch of the repository. | 
**delete_branch_on_merge** | Option<**bool**> | Whether to delete head branches when pull requests are merged | [optional][default to false]
**deployments_url** | **String** |  | 
**description** | Option<**String**> |  | 
**disabled** | Option<**bool**> | Returns whether or not this repository is disabled. | [optional]
**downloads_url** | **String** |  | 
**events_url** | **String** |  | 
**fork** | **bool** |  | 
**forks** | **i32** |  | 
**forks_count** | **i32** |  | 
**forks_url** | **String** |  | 
**full_name** | **String** |  | 
**git_commits_url** | **String** |  | 
**git_refs_url** | **String** |  | 
**git_tags_url** | **String** |  | 
**git_url** | **String** |  | 
**has_discussions** | Option<**bool**> | Whether the repository has discussions enabled. | [optional]
**has_downloads** | **bool** | Whether downloads are enabled. | [default to true]
**has_issues** | **bool** | Whether issues are enabled. | [default to true]
**has_pages** | **bool** |  | 
**has_projects** | **bool** | Whether projects are enabled. | [default to true]
**has_wiki** | **bool** | Whether the wiki is enabled. | [default to true]
**homepage** | Option<**String**> |  | 
**hooks_url** | **String** |  | 
**html_url** | **String** |  | 
**id** | **i64** | Unique identifier of the repository | 
**is_template** | Option<**bool**> |  | [optional]
**issue_comment_url** | **String** |  | 
**issue_events_url** | **String** |  | 
**issues_url** | **String** |  | 
**keys_url** | **String** |  | 
**labels_url** | **String** |  | 
**language** | Option<**String**> |  | 
**languages_url** | **String** |  | 
**license** | Option<[**models::License**](License.md)> |  | 
**master_branch** | Option<**String**> |  | [optional]
**merges_url** | **String** |  | 
**milestones_url** | **String** |  | 
**mirror_url** | Option<**String**> |  | 
**name** | **String** | The name of the repository. | 
**node_id** | **String** |  | 
**notifications_url** | **String** |  | 
**open_issues** | **i32** |  | 
**open_issues_count** | **i32** |  | 
**organization** | Option<**String**> |  | [optional]
**owner** | Option<[**models::User2**](User_2.md)> |  | 
**permissions** | Option<[**models::RepositoryPermissions**](Repository_permissions.md)> |  | [optional]
**private** | **bool** | Whether the repository is private or public. | 
**public** | Option<**bool**> |  | [optional]
**pulls_url** | **String** |  | 
**pushed_at** | Option<[**models::RepositoryPushedAt**](Repository_pushed_at.md)> |  | 
**releases_url** | **String** |  | 
**role_name** | Option<**String**> |  | [optional]
**size** | **i32** |  | 
**ssh_url** | **String** |  | 
**stargazers** | Option<**i32**> |  | [optional]
**stargazers_count** | **i32** |  | 
**stargazers_url** | **String** |  | 
**statuses_url** | **String** |  | 
**subscribers_url** | **String** |  | 
**subscription_url** | **String** |  | 
**svn_url** | **String** |  | 
**tags_url** | **String** |  | 
**teams_url** | **String** |  | 
**topics** | **Vec<String>** |  | 
**trees_url** | **String** |  | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**visibility** | **String** |  | 
**watchers** | **i32** |  | 
**watchers_count** | **i32** |  | 
**web_commit_signoff_required** | Option<**bool**> | Whether to require commit signoff. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


