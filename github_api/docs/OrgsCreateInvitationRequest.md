# OrgsCreateInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invitee_id** | Option<**i32**> | **Required unless you provide `email`**. GitHub user ID for the person you are inviting. | [optional]
**email** | Option<**String**> | **Required unless you provide `invitee_id`**. Email address of the person you are inviting, which can be an existing GitHub user. | [optional]
**role** | Option<**String**> | The role for the new member.   * `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.    * `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.    * `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.   * `reinstate` - The previous role assigned to the invitee before they were removed from your organization. Can be one of the roles listed above. Only works if the invitee was previously part of your organization. | [optional][default to DirectMember]
**team_ids** | Option<**Vec<i32>**> | Specify IDs for the teams you want to invite new members to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


