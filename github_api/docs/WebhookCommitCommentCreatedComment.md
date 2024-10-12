# WebhookCommitCommentCreatedComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_association** | **String** | How the author is associated with the repository. | 
**body** | **String** | The text of the comment. | 
**commit_id** | **String** | The SHA of the commit to which the comment applies. | 
**created_at** | **String** |  | 
**html_url** | **String** |  | 
**id** | **i32** | The ID of the commit comment. | 
**line** | Option<**i32**> | The line of the blob to which the comment applies. The last line of the range for a multi-line comment | 
**node_id** | **String** | The node ID of the commit comment. | 
**path** | Option<**String**> | The relative path of the file to which the comment applies. | 
**position** | Option<**i32**> | The line index in the diff to which the comment applies. | 
**reactions** | Option<[**models::Reactions**](Reactions.md)> |  | [optional]
**updated_at** | **String** |  | 
**url** | **String** |  | 
**user** | Option<[**models::User1**](User_1.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


