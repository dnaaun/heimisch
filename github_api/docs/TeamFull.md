# TeamFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier of the team | 
**node_id** | **String** |  | 
**url** | **String** | URL for the team | 
**html_url** | **String** |  | 
**name** | **String** | Name of the team | 
**slug** | **String** |  | 
**description** | Option<**String**> |  | 
**privacy** | Option<**String**> | The level of privacy this team should have | [optional]
**notification_setting** | Option<**String**> | The notification setting the team has set | [optional]
**permission** | **String** | Permission that the team will have for its repositories | 
**members_url** | **String** |  | 
**repositories_url** | **String** |  | 
**parent** | Option<[**models::NullableTeamSimple**](nullable-team-simple.md)> |  | [optional]
**members_count** | **i32** |  | 
**repos_count** | **i32** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**organization** | [**models::TeamOrganization**](team-organization.md) |  | 
**ldap_dn** | Option<**String**> | Distinguished Name (DN) that team maps to within LDAP environment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


