# PullsCreateReviewRequestCommentsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | **String** | The relative path to the file that necessitates a review comment. | 
**position** | Option<**i32**> | The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. The `position` value equals the number of lines down from the first \"@@\" hunk header in the file you want to add a comment. The line just below the \"@@\" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file. | [optional]
**body** | **String** | Text of the review comment. | 
**line** | Option<**i32**> |  | [optional]
**side** | Option<**String**> |  | [optional]
**start_line** | Option<**i32**> |  | [optional]
**start_side** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


