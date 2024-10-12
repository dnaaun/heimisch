# PullsSubmitReviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**String**> | The body text of the pull request review | [optional]
**event** | **String** | The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


