# ReposCreateInOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the repository. | 
**description** | Option<**String**> | A short description of the repository. | [optional]
**homepage** | Option<**String**> | A URL with more information about the repository. | [optional]
**private** | Option<**bool**> | Whether the repository is private. | [optional][default to false]
**visibility** | Option<**String**> | The visibility of the repository. | [optional]
**has_issues** | Option<**bool**> | Either `true` to enable issues for this repository or `false` to disable them. | [optional][default to true]
**has_projects** | Option<**bool**> | Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error. | [optional][default to true]
**has_wiki** | Option<**bool**> | Either `true` to enable the wiki for this repository or `false` to disable it. | [optional][default to true]
**has_downloads** | Option<**bool**> | Whether downloads are enabled. | [optional][default to true]
**is_template** | Option<**bool**> | Either `true` to make this repo available as a template repository or `false` to prevent it. | [optional][default to false]
**team_id** | Option<**i32**> | The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization. | [optional]
**auto_init** | Option<**bool**> | Pass `true` to create an initial commit with empty README. | [optional][default to false]
**gitignore_template** | Option<**String**> | Desired language or platform [.gitignore template](https://github.com/github/gitignore) to apply. Use the name of the template without the extension. For example, \"Haskell\". | [optional]
**license_template** | Option<**String**> | Choose an [open source license template](https://choosealicense.com/) that best suits your needs, and then use the [license keyword](https://docs.github.com/articles/licensing-a-repository/#searching-github-by-license-type) as the `license_template` string. For example, \"mit\" or \"mpl-2.0\". | [optional]
**allow_squash_merge** | Option<**bool**> | Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. | [optional][default to true]
**allow_merge_commit** | Option<**bool**> | Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. | [optional][default to true]
**allow_rebase_merge** | Option<**bool**> | Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. | [optional][default to true]
**allow_auto_merge** | Option<**bool**> | Either `true` to allow auto-merge on pull requests, or `false` to disallow auto-merge. | [optional][default to false]
**delete_branch_on_merge** | Option<**bool**> | Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion. **The authenticated user must be an organization owner to set this property to `true`.** | [optional][default to false]
**use_squash_pr_title_as_default** | Option<**bool**> | Either `true` to allow squash-merge commits to use pull request title, or `false` to use commit message. **This property is closing down. Please use `squash_merge_commit_title` instead. | [optional][default to false]
**squash_merge_commit_title** | Option<**String**> | Required when using `squash_merge_commit_message`.  The default value for a squash merge commit title:  - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit). | [optional]
**squash_merge_commit_message** | Option<**String**> | The default value for a squash merge commit message:  - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message. | [optional]
**merge_commit_title** | Option<**String**> | Required when using `merge_commit_message`.  The default value for a merge commit title.  - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name). | [optional]
**merge_commit_message** | Option<**String**> | The default value for a merge commit message.  - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message. | [optional]
**custom_properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The custom properties for the new repository. The keys are the custom property names, and the values are the corresponding custom property values. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


