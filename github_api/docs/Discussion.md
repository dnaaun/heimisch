# Discussion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_lock_reason** | Option<**String**> |  | 
**answer_chosen_at** | Option<**String**> |  | 
**answer_chosen_by** | Option<[**models::User2**](User_2.md)> |  | 
**answer_html_url** | Option<**String**> |  | 
**author_association** | **String** | How the author is associated with the repository. | 
**body** | **String** |  | 
**category** | [**models::DiscussionCategory**](discussion_category.md) |  | 
**comments** | **i32** |  | 
**created_at** | **String** |  | 
**html_url** | **String** |  | 
**id** | **i32** |  | 
**locked** | **bool** |  | 
**node_id** | **String** |  | 
**number** | **i32** |  | 
**reactions** | Option<[**models::Reactions**](Reactions.md)> |  | [optional]
**repository_url** | **String** |  | 
**state** | **String** | The current state of the discussion. `converting` means that the discussion is being converted from an issue. `transferring` means that the discussion is being transferred from another repository. | 
**state_reason** | Option<**String**> | The reason for the current state | 
**timeline_url** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**updated_at** | **String** |  | 
**user** | Option<[**models::User1**](User_1.md)> |  | 
**labels** | Option<[**Vec<models::Label>**](label.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


