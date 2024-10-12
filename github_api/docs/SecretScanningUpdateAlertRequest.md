# SecretScanningUpdateAlertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | [**models::SecretScanningAlertState**](secret-scanning-alert-state.md) |  | 
**resolution** | Option<[**models::SecretScanningAlertResolution**](secret-scanning-alert-resolution.md)> |  | [optional]
**resolution_comment** | Option<**String**> | An optional comment when closing an alert. Cannot be updated or deleted. Must be `null` when changing `state` to `open`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


