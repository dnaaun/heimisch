# PullsCreateReviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**commit_id** | Option<**String**> | The SHA of the commit that needs a review. Not using the latest commit SHA may render your review comment outdated if a subsequent commit modifies the line you specify as the `position`. Defaults to the most recent commit in the pull request when you do not specify a value. | [optional]
**body** | Option<**String**> | **Required** when using `REQUEST_CHANGES` or `COMMENT` for the `event` parameter. The body text of the pull request review. | [optional]
**event** | Option<**String**> | The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request) when you are ready. | [optional]
**comments** | Option<[**Vec<models::PullsCreateReviewRequestCommentsInner>**](pulls_create_review_request_comments_inner.md)> | Use the following table to specify the location, destination, and contents of the draft review comment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


