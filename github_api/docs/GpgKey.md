# GpgKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**name** | Option<**String**> |  | [optional]
**primary_key_id** | Option<**i32**> |  | 
**key_id** | **String** |  | 
**public_key** | **String** |  | 
**emails** | [**Vec<models::GpgKeyEmailsInner>**](gpg_key_emails_inner.md) |  | 
**subkeys** | [**Vec<models::GpgKeySubkeysInner>**](gpg_key_subkeys_inner.md) |  | 
**can_sign** | **bool** |  | 
**can_encrypt_comms** | **bool** |  | 
**can_encrypt_storage** | **bool** |  | 
**can_certify** | **bool** |  | 
**created_at** | **String** |  | 
**expires_at** | Option<**String**> |  | 
**revoked** | **bool** |  | 
**raw_key** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


