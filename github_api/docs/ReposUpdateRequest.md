# ReposUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the repository. | [optional]
**description** | Option<**String**> | A short description of the repository. | [optional]
**homepage** | Option<**String**> | A URL with more information about the repository. | [optional]
**private** | Option<**bool**> | Either `true` to make the repository private or `false` to make it public. Default: `false`.   **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://docs.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private. | [optional][default to false]
**visibility** | Option<**String**> | The visibility of the repository. | [optional]
**security_and_analysis** | Option<[**models::ReposUpdateRequestSecurityAndAnalysis**](repos_update_request_security_and_analysis.md)> |  | [optional]
**has_issues** | Option<**bool**> | Either `true` to enable issues for this repository or `false` to disable them. | [optional][default to true]
**has_projects** | Option<**bool**> | Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error. | [optional][default to true]
**has_wiki** | Option<**bool**> | Either `true` to enable the wiki for this repository or `false` to disable it. | [optional][default to true]
**is_template** | Option<**bool**> | Either `true` to make this repo available as a template repository or `false` to prevent it. | [optional][default to false]
**default_branch** | Option<**String**> | Updates the default branch for this repository. | [optional]
**allow_squash_merge** | Option<**bool**> | Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. | [optional][default to true]
**allow_merge_commit** | Option<**bool**> | Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. | [optional][default to true]
**allow_rebase_merge** | Option<**bool**> | Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. | [optional][default to true]
**allow_auto_merge** | Option<**bool**> | Either `true` to allow auto-merge on pull requests, or `false` to disallow auto-merge. | [optional][default to false]
**delete_branch_on_merge** | Option<**bool**> | Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion. | [optional][default to false]
**allow_update_branch** | Option<**bool**> | Either `true` to always allow a pull request head branch that is behind its base branch to be updated even if it is not required to be up to date before merging, or false otherwise. | [optional][default to false]
**use_squash_pr_title_as_default** | Option<**bool**> | Either `true` to allow squash-merge commits to use pull request title, or `false` to use commit message. **This property is closing down. Please use `squash_merge_commit_title` instead. | [optional][default to false]
**squash_merge_commit_title** | Option<**String**> | Required when using `squash_merge_commit_message`.  The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit). | [optional]
**squash_merge_commit_message** | Option<**String**> | The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message. | [optional]
**merge_commit_title** | Option<**String**> | Required when using `merge_commit_message`.  The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name). | [optional]
**merge_commit_message** | Option<**String**> | The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message. | [optional]
**archived** | Option<**bool**> | Whether to archive this repository. `false` will unarchive a previously archived repository. | [optional][default to false]
**allow_forking** | Option<**bool**> | Either `true` to allow private forks, or `false` to prevent private forks. | [optional][default to false]
**web_commit_signoff_required** | Option<**bool**> | Either `true` to require contributors to sign off on web-based commits, or `false` to not require contributors to sign off on web-based commits. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


