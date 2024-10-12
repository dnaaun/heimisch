# TeamDiscussion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**body** | **String** | The main text of the discussion. | 
**body_html** | **String** |  | 
**body_version** | **String** | The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server. | 
**comments_count** | **i32** |  | 
**comments_url** | **String** |  | 
**created_at** | **String** |  | 
**last_edited_at** | Option<**String**> |  | 
**html_url** | **String** |  | 
**node_id** | **String** |  | 
**number** | **i32** | The unique sequence number of a team discussion. | 
**pinned** | **bool** | Whether or not this discussion should be pinned for easy retrieval. | 
**private** | **bool** | Whether or not this discussion should be restricted to team members and organization owners. | 
**team_url** | **String** |  | 
**title** | **String** | The title of the discussion. | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**reactions** | Option<[**models::ReactionRollup**](reaction-rollup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


