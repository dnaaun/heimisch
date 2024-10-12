# GpgKeySubkeysInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> |  | [optional]
**primary_key_id** | Option<**i32**> |  | [optional]
**key_id** | Option<**String**> |  | [optional]
**public_key** | Option<**String**> |  | [optional]
**emails** | Option<[**Vec<models::GpgKeyEmailsInner>**](gpg_key_emails_inner.md)> |  | [optional]
**subkeys** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**can_sign** | Option<**bool**> |  | [optional]
**can_encrypt_comms** | Option<**bool**> |  | [optional]
**can_encrypt_storage** | Option<**bool**> |  | [optional]
**can_certify** | Option<**bool**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**expires_at** | Option<**String**> |  | [optional]
**raw_key** | Option<**String**> |  | [optional]
**revoked** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


