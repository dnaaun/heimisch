# TeamsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the team. | 
**description** | Option<**String**> | The description of the team. | [optional]
**maintainers** | Option<**Vec<String>**> | List GitHub IDs for organization members who will become team maintainers. | [optional]
**repo_names** | Option<**Vec<String>**> | The full name (e.g., \"organization-name/repository-name\") of repositories to add the team to. | [optional]
**privacy** | Option<**String**> | The level of privacy this team should have. The options are:   **For a non-nested team:**    * `secret` - only visible to organization owners and members of this team.    * `closed` - visible to all members of this organization.   Default: `secret`   **For a parent or child team:**    * `closed` - visible to all members of this organization.   Default for child team: `closed` | [optional]
**notification_setting** | Option<**String**> | The notification setting the team has chosen. The options are:    * `notifications_enabled` - team members receive notifications when the team is @mentioned.    * `notifications_disabled` - no one receives notifications.   Default: `notifications_enabled` | [optional]
**permission** | Option<**String**> | **Closing down notice**. The permission that new repositories will be added to the team with when none is specified. | [optional][default to Pull]
**parent_team_id** | Option<**i32**> | The ID of a team to set as the parent team. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


