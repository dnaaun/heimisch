# PullRequestWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**id** | **i64** |  | 
**node_id** | **String** |  | 
**html_url** | **String** |  | 
**diff_url** | **String** |  | 
**patch_url** | **String** |  | 
**issue_url** | **String** |  | 
**commits_url** | **String** |  | 
**review_comments_url** | **String** |  | 
**review_comment_url** | **String** |  | 
**comments_url** | **String** |  | 
**statuses_url** | **String** |  | 
**number** | **i32** | Number uniquely identifying the pull request within its repository. | 
**state** | **String** | State of this Pull Request. Either `open` or `closed`. | 
**locked** | **bool** |  | 
**title** | **String** | The title of the pull request. | 
**user** | [**models::SimpleUser**](simple-user.md) |  | 
**body** | Option<**String**> |  | 
**labels** | [**Vec<models::PullRequestLabelsInner>**](pull_request_labels_inner.md) |  | 
**milestone** | Option<[**models::NullableMilestone**](nullable-milestone.md)> |  | 
**active_lock_reason** | Option<**String**> |  | [optional]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**closed_at** | Option<**String**> |  | 
**merged_at** | Option<**String**> |  | 
**merge_commit_sha** | Option<**String**> |  | 
**assignee** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**assignees** | Option<[**Vec<models::SimpleUser>**](simple-user.md)> |  | [optional]
**requested_reviewers** | Option<[**Vec<models::SimpleUser>**](simple-user.md)> |  | [optional]
**requested_teams** | Option<[**Vec<models::TeamSimple>**](team-simple.md)> |  | [optional]
**head** | [**models::PullRequestHead**](pull_request_head.md) |  | 
**base** | [**models::PullRequestHead**](pull_request_head.md) |  | 
**_links** | [**models::PullRequestSimpleLinks**](pull_request_simple__links.md) |  | 
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 
**auto_merge** | Option<[**models::AutoMerge**](auto-merge.md)> |  | 
**draft** | Option<**bool**> | Indicates whether or not the pull request is a draft. | [optional]
**merged** | **bool** |  | 
**mergeable** | Option<**bool**> |  | 
**rebaseable** | Option<**bool**> |  | [optional]
**mergeable_state** | **String** |  | 
**merged_by** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**comments** | **i32** |  | 
**review_comments** | **i32** |  | 
**maintainer_can_modify** | **bool** | Indicates whether maintainers can modify the pull request. | 
**commits** | **i32** |  | 
**additions** | **i32** |  | 
**deletions** | **i32** |  | 
**changed_files** | **i32** |  | 
**allow_auto_merge** | Option<**bool**> | Whether to allow auto-merge for pull requests. | [optional][default to false]
**allow_update_branch** | Option<**bool**> | Whether to allow updating the pull request's branch. | [optional]
**delete_branch_on_merge** | Option<**bool**> | Whether to delete head branches when pull requests are merged. | [optional][default to false]
**merge_commit_message** | Option<**String**> | The default value for a merge commit message. - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message. | [optional]
**merge_commit_title** | Option<**String**> | The default value for a merge commit title. - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., \"Merge pull request #123 from branch-name\"). | [optional]
**squash_merge_commit_message** | Option<**String**> | The default value for a squash merge commit message: - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message. | [optional]
**squash_merge_commit_title** | Option<**String**> | The default value for a squash merge commit title: - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit). | [optional]
**use_squash_pr_title_as_default** | Option<**bool**> | Whether a squash merge commit can use the pull request title as default. **This property is closing down. Please use `squash_merge_commit_title` instead.** | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


