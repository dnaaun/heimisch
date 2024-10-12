# ReposCreateForAuthenticatedUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the repository. | 
**description** | Option<**String**> | A short description of the repository. | [optional]
**homepage** | Option<**String**> | A URL with more information about the repository. | [optional]
**private** | Option<**bool**> | Whether the repository is private. | [optional][default to false]
**has_issues** | Option<**bool**> | Whether issues are enabled. | [optional][default to true]
**has_projects** | Option<**bool**> | Whether projects are enabled. | [optional][default to true]
**has_wiki** | Option<**bool**> | Whether the wiki is enabled. | [optional][default to true]
**has_discussions** | Option<**bool**> | Whether discussions are enabled. | [optional][default to false]
**team_id** | Option<**i32**> | The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization. | [optional]
**auto_init** | Option<**bool**> | Whether the repository is initialized with a minimal README. | [optional][default to false]
**gitignore_template** | Option<**String**> | The desired language or platform to apply to the .gitignore. | [optional]
**license_template** | Option<**String**> | The license keyword of the open source license for this repository. | [optional]
**allow_squash_merge** | Option<**bool**> | Whether to allow squash merges for pull requests. | [optional][default to true]
**allow_merge_commit** | Option<**bool**> | Whether to allow merge commits for pull requests. | [optional][default to true]
**allow_rebase_merge** | Option<**bool**> | Whether to allow rebase merges for pull requests. | [optional][default to true]
**allow_auto_merge** | Option<**bool**> | Whether to allow Auto-merge to be used on pull requests. | [optional][default to false]
**delete_branch_on_merge** | Option<**bool**> | Whether to delete head branches when pull requests are merged | [optional][default to false]
**squash_merge_commit_title** | Option<**String**> | Required when using `squash_merge_commit_message`.  The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit). | [optional]
**squash_merge_commit_message** | Option<**String**> | The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message. | [optional]
**merge_commit_title** | Option<**String**> | Required when using `merge_commit_message`.  The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name). | [optional]
**merge_commit_message** | Option<**String**> | The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message. | [optional]
**has_downloads** | Option<**bool**> | Whether downloads are enabled. | [optional][default to true]
**is_template** | Option<**bool**> | Whether this repository acts as a template that can be used to generate new repositories. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


