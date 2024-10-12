# FullRepository

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**node_id** | **String** |  | 
**name** | **String** |  | 
**full_name** | **String** |  | 
**owner** | [**models::SimpleUser**](simple-user.md) |  | 
**private** | **bool** |  | 
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
**size** | **i32** | The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0. | 
**default_branch** | **String** |  | 
**open_issues_count** | **i32** |  | 
**is_template** | Option<**bool**> |  | [optional]
**topics** | Option<**Vec<String>**> |  | [optional]
**has_issues** | **bool** |  | 
**has_projects** | **bool** |  | 
**has_wiki** | **bool** |  | 
**has_pages** | **bool** |  | 
**has_downloads** | Option<**bool**> |  | [optional]
**has_discussions** | **bool** |  | 
**archived** | **bool** |  | 
**disabled** | **bool** | Returns whether or not this repository disabled. | 
**visibility** | Option<**String**> | The repository visibility: public, private, or internal. | [optional]
**pushed_at** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**permissions** | Option<[**models::FullRepositoryPermissions**](full_repository_permissions.md)> |  | [optional]
**allow_rebase_merge** | Option<**bool**> |  | [optional]
**template_repository** | Option<[**models::NullableRepository**](nullable-repository.md)> |  | [optional]
**temp_clone_token** | Option<**String**> |  | [optional]
**allow_squash_merge** | Option<**bool**> |  | [optional]
**allow_auto_merge** | Option<**bool**> |  | [optional]
**delete_branch_on_merge** | Option<**bool**> |  | [optional]
**allow_merge_commit** | Option<**bool**> |  | [optional]
**allow_update_branch** | Option<**bool**> |  | [optional]
**use_squash_pr_title_as_default** | Option<**bool**> |  | [optional]
**squash_merge_commit_title** | Option<**String**> | The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit). | [optional]
**squash_merge_commit_message** | Option<**String**> | The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message. | [optional]
**merge_commit_title** | Option<**String**> | The default value for a merge commit title.    - `PR_TITLE` - default to the pull request's title.   - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name). | [optional]
**merge_commit_message** | Option<**String**> | The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message. | [optional]
**allow_forking** | Option<**bool**> |  | [optional]
**web_commit_signoff_required** | Option<**bool**> |  | [optional]
**subscribers_count** | **i32** |  | 
**network_count** | **i32** |  | 
**license** | Option<[**models::NullableLicenseSimple**](nullable-license-simple.md)> |  | 
**organization** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**parent** | Option<[**models::Repository**](repository.md)> |  | [optional]
**source** | Option<[**models::Repository**](repository.md)> |  | [optional]
**forks** | **i32** |  | 
**master_branch** | Option<**String**> |  | [optional]
**open_issues** | **i32** |  | 
**watchers** | **i32** |  | 
**anonymous_access_enabled** | Option<**bool**> | Whether anonymous git access is allowed. | [optional][default to true]
**code_of_conduct** | Option<[**models::CodeOfConductSimple**](code-of-conduct-simple.md)> |  | [optional]
**security_and_analysis** | Option<[**models::SecurityAndAnalysis**](security-and-analysis.md)> |  | [optional]
**custom_properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The custom properties that were defined for the repository. The keys are the custom property names, and the values are the corresponding custom property values. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


