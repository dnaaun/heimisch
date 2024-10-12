# ReposUpdateBranchProtectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**required_status_checks** | Option<[**models::ReposUpdateBranchProtectionRequestRequiredStatusChecks**](repos_update_branch_protection_request_required_status_checks.md)> |  | 
**enforce_admins** | Option<**bool**> | Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable. | 
**required_pull_request_reviews** | Option<[**models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviews**](repos_update_branch_protection_request_required_pull_request_reviews.md)> |  | 
**restrictions** | Option<[**models::ReposUpdateBranchProtectionRequestRestrictions**](repos_update_branch_protection_request_restrictions.md)> |  | 
**required_linear_history** | Option<**bool**> | Enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch. Set to `true` to enforce a linear commit history. Set to `false` to disable a linear commit Git history. Your repository must allow squash merging or rebase merging before you can enable a linear commit history. Default: `false`. For more information, see \"[Requiring a linear commit history](https://docs.github.com/github/administering-a-repository/requiring-a-linear-commit-history)\" in the GitHub Help documentation. | [optional]
**allow_force_pushes** | Option<**bool**> | Permits force pushes to the protected branch by anyone with write access to the repository. Set to `true` to allow force pushes. Set to `false` or `null` to block force pushes. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://docs.github.com/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.\" | [optional]
**allow_deletions** | Option<**bool**> | Allows deletion of the protected branch by anyone with write access to the repository. Set to `false` to prevent deletion of the protected branch. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://docs.github.com/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation. | [optional]
**block_creations** | Option<**bool**> | If set to `true`, the `restrictions` branch protection settings which limits who can push will also block pushes which create new branches, unless the push is initiated by a user, team, or app which has the ability to push. Set to `true` to restrict new branch creation. Default: `false`. | [optional]
**required_conversation_resolution** | Option<**bool**> | Requires all conversations on code to be resolved before a pull request can be merged into a branch that matches this rule. Set to `false` to disable. Default: `false`. | [optional]
**lock_branch** | Option<**bool**> | Whether to set the branch as read-only. If this is true, users will not be able to push to the branch. Default: `false`. | [optional][default to false]
**allow_fork_syncing** | Option<**bool**> | Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing. Default: `false`. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


