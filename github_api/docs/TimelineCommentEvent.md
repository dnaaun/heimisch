# TimelineCommentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | **String** |  | 
**actor** | [**models::SimpleUser**](simple-user.md) |  | 
**id** | **i32** | Unique identifier of the issue comment | 
**node_id** | **String** |  | 
**url** | **String** | URL for the issue comment | 
**body** | Option<**String**> | Contents of the issue comment | [optional]
**body_text** | Option<**String**> |  | [optional]
**body_html** | Option<**String**> |  | [optional]
**html_url** | **String** |  | 
**user** | [**models::SimpleUser**](simple-user.md) |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**issue_url** | **String** |  | 
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 
**performed_via_github_app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | [optional]
**reactions** | Option<[**models::ReactionRollup**](reaction-rollup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


