# TeamsUpdateInOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the team. | [optional]
**description** | Option<**String**> | The description of the team. | [optional]
**privacy** | Option<**String**> | The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. When a team is nested, the `privacy` for parent teams cannot be `secret`. The options are:   **For a non-nested team:**    * `secret` - only visible to organization owners and members of this team.    * `closed` - visible to all members of this organization.   **For a parent or child team:**    * `closed` - visible to all members of this organization. | [optional]
**notification_setting** | Option<**String**> | The notification setting the team has chosen. Editing teams without specifying this parameter leaves `notification_setting` intact. The options are:   * `notifications_enabled` - team members receive notifications when the team is @mentioned.    * `notifications_disabled` - no one receives notifications. | [optional]
**permission** | Option<**String**> | **Closing down notice**. The permission that new repositories will be added to the team with when none is specified. | [optional][default to Pull]
**parent_team_id** | Option<**i32**> | The ID of a team to set as the parent team. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


