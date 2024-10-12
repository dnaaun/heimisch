# CodeownersErrorsErrorsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line** | **i32** | The line number where this errors occurs. | 
**column** | **i32** | The column number where this errors occurs. | 
**source** | Option<**String**> | The contents of the line where the error occurs. | [optional]
**kind** | **String** | The type of error. | 
**suggestion** | Option<**String**> | Suggested action to fix the error. This will usually be `null`, but is provided for some common errors. | [optional]
**message** | **String** | A human-readable description of the error, combining information from multiple fields, laid out for display in a monospaced typeface (for example, a command-line setting). | 
**path** | **String** | The path of the file where the error occured. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


