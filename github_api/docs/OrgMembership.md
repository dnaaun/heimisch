# OrgMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**state** | **String** | The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation. | 
**role** | **String** | The user's membership type in the organization. | 
**organization_url** | **String** |  | 
**organization** | [**models::OrganizationSimple**](organization-simple.md) |  | 
**user** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**permissions** | Option<[**models::OrgMembershipPermissions**](org_membership_permissions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


