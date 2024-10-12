# ReposUpdateBranchProtectionRequestRequiredStatusChecks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strict** | **bool** | Require branches to be up to date before merging. | 
**contexts** | **Vec<String>** | **Closing down notice**: The list of status checks to require in order to merge into this branch. If any of these checks have recently been set by a particular GitHub App, they will be required to come from that app in future for the branch to merge. Use `checks` instead of `contexts` for more fine-grained control. | 
**checks** | Option<[**Vec<models::ReposUpdateBranchProtectionRequestRequiredStatusChecksChecksInner>**](repos_update_branch_protection_request_required_status_checks_checks_inner.md)> | The list of status checks to require in order to merge into this branch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


