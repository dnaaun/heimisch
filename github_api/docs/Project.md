# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_url** | **String** |  | 
**url** | **String** |  | 
**html_url** | **String** |  | 
**columns_url** | **String** |  | 
**id** | **i32** |  | 
**node_id** | **String** |  | 
**name** | **String** | Name of the project | 
**body** | Option<**String**> | Body of the project | 
**number** | **i32** |  | 
**state** | **String** | State of the project; either 'open' or 'closed' | 
**creator** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**organization_permission** | Option<**String**> | The baseline permission that all organization members have on this project. Only present if owner is an organization. | [optional]
**private** | Option<**bool**> | Whether or not this project can be seen by everyone. Only present if owner is an organization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


