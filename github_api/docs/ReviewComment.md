# ReviewComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**pull_request_review_id** | Option<**i64**> |  | 
**id** | **i64** |  | 
**node_id** | **String** |  | 
**diff_hunk** | **String** |  | 
**path** | **String** |  | 
**position** | Option<**i32**> |  | 
**original_position** | **i32** |  | 
**commit_id** | **String** |  | 
**original_commit_id** | **String** |  | 
**in_reply_to_id** | Option<**i32**> |  | [optional]
**user** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**body** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**html_url** | **String** |  | 
**pull_request_url** | **String** |  | 
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 
**_links** | [**models::ReviewCommentLinks**](review_comment__links.md) |  | 
**body_text** | Option<**String**> |  | [optional]
**body_html** | Option<**String**> |  | [optional]
**reactions** | Option<[**models::ReactionRollup**](reaction-rollup.md)> |  | [optional]
**side** | Option<**String**> | The side of the first line of the range for a multi-line comment. | [optional][default to Right]
**start_side** | Option<**String**> | The side of the first line of the range for a multi-line comment. | [optional][default to Right]
**line** | Option<**i32**> | The line of the blob to which the comment applies. The last line of the range for a multi-line comment | [optional]
**original_line** | Option<**i32**> | The original line of the blob to which the comment applies. The last line of the range for a multi-line comment | [optional]
**start_line** | Option<**i32**> | The first line of the range for a multi-line comment. | [optional]
**original_start_line** | Option<**i32**> | The original first line of the range for a multi-line comment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


