# TimelineReviewedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | **String** |  | 
**id** | **i32** | Unique identifier of the review | 
**node_id** | **String** |  | 
**user** | [**models::SimpleUser**](simple-user.md) |  | 
**body** | Option<**String**> | The text of the review. | 
**state** | **String** |  | 
**html_url** | **String** |  | 
**pull_request_url** | **String** |  | 
**_links** | [**models::TimelineReviewedEventLinks**](timeline_reviewed_event__links.md) |  | 
**submitted_at** | Option<**String**> |  | [optional]
**commit_id** | **String** | A commit SHA for the review. | 
**body_html** | Option<**String**> |  | [optional]
**body_text** | Option<**String**> |  | [optional]
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


