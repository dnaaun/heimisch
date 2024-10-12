# PullRequestSimple

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
**number** | **i32** |  | 
**state** | **String** |  | 
**locked** | **bool** |  | 
**title** | **String** |  | 
**user** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**body** | Option<**String**> |  | 
**labels** | [**Vec<models::PullRequestSimpleLabelsInner>**](pull_request_simple_labels_inner.md) |  | 
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
**requested_teams** | Option<[**Vec<models::Team>**](team.md)> |  | [optional]
**head** | [**models::PullRequestSimpleHead**](pull_request_simple_head.md) |  | 
**base** | [**models::PullRequestSimpleHead**](pull_request_simple_head.md) |  | 
**_links** | [**models::PullRequestSimpleLinks**](pull_request_simple__links.md) |  | 
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 
**auto_merge** | Option<[**models::AutoMerge**](auto-merge.md)> |  | 
**draft** | Option<**bool**> | Indicates whether or not the pull request is a draft. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


