# ProtectedBranchRequiredPullRequestReviews

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**dismiss_stale_reviews** | Option<**bool**> |  | [optional]
**require_code_owner_reviews** | Option<**bool**> |  | [optional]
**required_approving_review_count** | Option<**i32**> |  | [optional]
**require_last_push_approval** | Option<**bool**> | Whether the most recent push must be approved by someone other than the person who pushed it. | [optional][default to false]
**dismissal_restrictions** | Option<[**models::ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions**](protected_branch_required_pull_request_reviews_dismissal_restrictions.md)> |  | [optional]
**bypass_pull_request_allowances** | Option<[**models::ProtectedBranchRequiredPullRequestReviewsBypassPullRequestAllowances**](protected_branch_required_pull_request_reviews_bypass_pull_request_allowances.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


