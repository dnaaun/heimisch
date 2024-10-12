# CodeScanningSarifsStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**processing_status** | Option<**String**> | `pending` files have not yet been processed, while `complete` means results from the SARIF have been stored. `failed` files have either not been processed at all, or could only be partially processed. | [optional]
**analyses_url** | Option<**String**> | The REST API URL for getting the analyses associated with the upload. | [optional][readonly]
**errors** | Option<**Vec<String>**> | Any errors that ocurred during processing of the delivery. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


