# ProtectedBranch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**required_status_checks** | Option<[**models::StatusCheckPolicy**](status-check-policy.md)> |  | [optional]
**required_pull_request_reviews** | Option<[**models::ProtectedBranchRequiredPullRequestReviews**](protected_branch_required_pull_request_reviews.md)> |  | [optional]
**required_signatures** | Option<[**models::BranchProtectionRequiredSignatures**](branch_protection_required_signatures.md)> |  | [optional]
**enforce_admins** | Option<[**models::ProtectedBranchEnforceAdmins**](protected_branch_enforce_admins.md)> |  | [optional]
**required_linear_history** | Option<[**models::ProtectedBranchRequiredLinearHistory**](protected_branch_required_linear_history.md)> |  | [optional]
**allow_force_pushes** | Option<[**models::ProtectedBranchRequiredLinearHistory**](protected_branch_required_linear_history.md)> |  | [optional]
**allow_deletions** | Option<[**models::ProtectedBranchRequiredLinearHistory**](protected_branch_required_linear_history.md)> |  | [optional]
**restrictions** | Option<[**models::BranchRestrictionPolicy**](branch-restriction-policy.md)> |  | [optional]
**required_conversation_resolution** | Option<[**models::ProtectedBranchRequiredConversationResolution**](protected_branch_required_conversation_resolution.md)> |  | [optional]
**block_creations** | Option<[**models::ProtectedBranchRequiredLinearHistory**](protected_branch_required_linear_history.md)> |  | [optional]
**lock_branch** | Option<[**models::ProtectedBranchLockBranch**](protected_branch_lock_branch.md)> |  | [optional]
**allow_fork_syncing** | Option<[**models::ProtectedBranchAllowForkSyncing**](protected_branch_allow_fork_syncing.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


