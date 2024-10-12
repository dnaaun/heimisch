# PullRequestReview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Unique identifier of the review | 
**node_id** | **String** |  | 
**user** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**body** | **String** | The text of the review. | 
**state** | **String** |  | 
**html_url** | **String** |  | 
**pull_request_url** | **String** |  | 
**_links** | [**models::TimelineReviewedEventLinks**](timeline_reviewed_event__links.md) |  | 
**submitted_at** | Option<**String**> |  | [optional]
**commit_id** | Option<**String**> | A commit SHA for the review. If the commit object was garbage collected or forcibly deleted, then it no longer exists in Git and this value will be `null`. | 
**body_html** | Option<**String**> |  | [optional]
**body_text** | Option<**String**> |  | [optional]
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


