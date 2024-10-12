# ChecksCreateRequestOutputAnnotationsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | **String** | The path of the file to add an annotation to. For example, `assets/css/main.css`. | 
**start_line** | **i32** | The start line of the annotation. Line numbers start at 1. | 
**end_line** | **i32** | The end line of the annotation. | 
**start_column** | Option<**i32**> | The start column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values. Column numbers start at 1. | [optional]
**end_column** | Option<**i32**> | The end column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values. | [optional]
**annotation_level** | **String** | The level of the annotation. | 
**message** | **String** | A short description of the feedback for these lines of code. The maximum size is 64 KB. | 
**title** | Option<**String**> | The title that represents the annotation. The maximum size is 255 characters. | [optional]
**raw_details** | Option<**String**> | Details about this annotation. The maximum size is 64 KB. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


