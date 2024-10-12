# ReposUpdatePullRequestReviewProtectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dismissal_restrictions** | Option<[**models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviewsDismissalRestrictions**](repos_update_branch_protection_request_required_pull_request_reviews_dismissal_restrictions.md)> |  | [optional]
**dismiss_stale_reviews** | Option<**bool**> | Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit. | [optional]
**require_code_owner_reviews** | Option<**bool**> | Blocks merging pull requests until [code owners](https://docs.github.com/articles/about-code-owners/) have reviewed. | [optional]
**required_approving_review_count** | Option<**i32**> | Specifies the number of reviewers required to approve pull requests. Use a number between 1 and 6 or 0 to not require reviewers. | [optional]
**require_last_push_approval** | Option<**bool**> | Whether the most recent push must be approved by someone other than the person who pushed it. Default: `false` | [optional][default to false]
**bypass_pull_request_allowances** | Option<[**models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviewsBypassPullRequestAllowances**](repos_update_branch_protection_request_required_pull_request_reviews_bypass_pull_request_allowances.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


