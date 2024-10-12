# PullRequest1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_links** | [**models::WebhooksPullRequest5Links**](webhooks_pull_request_5__links.md) |  | 
**active_lock_reason** | Option<**String**> |  | 
**additions** | Option<**i32**> |  | [optional]
**assignee** | Option<[**models::User2**](User_2.md)> |  | 
**assignees** | [**Vec<models::User>**](User.md) |  | 
**author_association** | **String** | How the author is associated with the repository. | 
**auto_merge** | Option<[**models::PullRequestAutoMerge**](PullRequestAutoMerge.md)> |  | 
**base** | [**models::PullRequest1Base**](Pull_Request_1_base.md) |  | 
**body** | Option<**String**> |  | 
**changed_files** | Option<**i32**> |  | [optional]
**closed_at** | Option<**String**> |  | 
**comments** | Option<**i32**> |  | [optional]
**comments_url** | **String** |  | 
**commits** | Option<**i32**> |  | [optional]
**commits_url** | **String** |  | 
**created_at** | **String** |  | 
**deletions** | Option<**i32**> |  | [optional]
**diff_url** | **String** |  | 
**draft** | **bool** | Indicates whether or not the pull request is a draft. | 
**head** | [**models::PullRequestBase**](Pull_Request_base.md) |  | 
**html_url** | **String** |  | 
**id** | **i32** |  | 
**issue_url** | **String** |  | 
**labels** | [**Vec<models::Label>**](Label.md) |  | 
**locked** | **bool** |  | 
**maintainer_can_modify** | Option<**bool**> | Indicates whether maintainers can modify the pull request. | [optional]
**merge_commit_sha** | Option<**String**> |  | 
**mergeable** | Option<**bool**> |  | [optional]
**mergeable_state** | Option<**String**> |  | [optional]
**merged** | Option<**bool**> |  | [optional]
**merged_at** | Option<**String**> |  | 
**merged_by** | Option<[**models::User2**](User_2.md)> |  | [optional]
**milestone** | Option<[**models::Milestone1**](Milestone_1.md)> |  | 
**node_id** | **String** |  | 
**number** | **i32** | Number uniquely identifying the pull request within its repository. | 
**patch_url** | **String** |  | 
**rebaseable** | Option<**bool**> |  | [optional]
**requested_reviewers** | [**Vec<models::PullRequestRequestedReviewersInner>**](Pull_Request_requested_reviewers_inner.md) |  | 
**requested_teams** | [**Vec<models::Team>**](Team.md) |  | 
**review_comment_url** | **String** |  | 
**review_comments** | Option<**i32**> |  | [optional]
**review_comments_url** | **String** |  | 
**state** | **String** | State of this Pull Request. Either `open` or `closed`. | 
**statuses_url** | **String** |  | 
**title** | **String** | The title of the pull request. | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**user** | Option<[**models::User3**](User_3.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


