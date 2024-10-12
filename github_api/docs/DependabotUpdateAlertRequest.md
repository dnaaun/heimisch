# DependabotUpdateAlertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | **String** | The state of the Dependabot alert. A `dismissed_reason` must be provided when setting the state to `dismissed`. | 
**dismissed_reason** | Option<**String**> | **Required when `state` is `dismissed`.** A reason for dismissing the alert. | [optional]
**dismissed_comment** | Option<**String**> | An optional comment associated with dismissing the alert. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


