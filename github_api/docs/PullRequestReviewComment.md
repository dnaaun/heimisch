# PullRequestReviewComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_links** | [**models::WebhooksReviewCommentLinks**](webhooks_review_comment__links.md) |  | 
**author_association** | **String** | How the author is associated with the repository. | 
**body** | **String** | The text of the comment. | 
**commit_id** | **String** | The SHA of the commit to which the comment applies. | 
**created_at** | **String** |  | 
**diff_hunk** | **String** | The diff of the line that the comment refers to. | 
**html_url** | **String** | HTML URL for the pull request review comment. | 
**id** | **i32** | The ID of the pull request review comment. | 
**in_reply_to_id** | Option<**i32**> | The comment ID to reply to. | [optional]
**line** | Option<**i32**> | The line of the blob to which the comment applies. The last line of the range for a multi-line comment | 
**node_id** | **String** | The node ID of the pull request review comment. | 
**original_commit_id** | **String** | The SHA of the original commit to which the comment applies. | 
**original_line** | Option<**i32**> | The line of the blob to which the comment applies. The last line of the range for a multi-line comment | 
**original_position** | **i32** | The index of the original line in the diff to which the comment applies. | 
**original_start_line** | Option<**i32**> | The first line of the range for a multi-line comment. | 
**path** | **String** | The relative path of the file to which the comment applies. | 
**position** | Option<**i32**> | The line index in the diff to which the comment applies. | 
**pull_request_review_id** | Option<**i32**> | The ID of the pull request review to which the comment belongs. | 
**pull_request_url** | **String** | URL for the pull request that the review comment belongs to. | 
**reactions** | [**models::Reactions**](Reactions.md) |  | 
**side** | **String** | The side of the first line of the range for a multi-line comment. | 
**start_line** | Option<**i32**> | The first line of the range for a multi-line comment. | 
**start_side** | Option<**String**> | The side of the first line of the range for a multi-line comment. | [default to Right]
**subject_type** | Option<**String**> | The level at which the comment is targeted, can be a diff line or a file. | [optional]
**updated_at** | **String** |  | 
**url** | **String** | URL for the pull request review comment | 
**user** | Option<[**models::User1**](User_1.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


