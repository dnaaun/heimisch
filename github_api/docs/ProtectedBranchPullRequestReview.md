# ProtectedBranchPullRequestReview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> |  | [optional]
**dismissal_restrictions** | Option<[**models::ProtectedBranchPullRequestReviewDismissalRestrictions**](protected_branch_pull_request_review_dismissal_restrictions.md)> |  | [optional]
**bypass_pull_request_allowances** | Option<[**models::ProtectedBranchPullRequestReviewBypassPullRequestAllowances**](protected_branch_pull_request_review_bypass_pull_request_allowances.md)> |  | [optional]
**dismiss_stale_reviews** | **bool** |  | 
**require_code_owner_reviews** | **bool** |  | 
**required_approving_review_count** | Option<**i32**> |  | [optional]
**require_last_push_approval** | Option<**bool**> | Whether the most recent push must be approved by someone other than the person who pushed it. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


