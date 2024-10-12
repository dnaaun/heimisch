# CopilotSeatDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignee** | [**models::SimpleUser**](simple-user.md) |  | 
**organization** | Option<[**models::NullableOrganizationSimple**](nullable-organization-simple.md)> |  | [optional]
**assigning_team** | Option<[**models::CopilotSeatDetailsAssigningTeam**](copilot_seat_details_assigning_team.md)> |  | [optional]
**pending_cancellation_date** | Option<[**String**](string.md)> | The pending cancellation date for the seat, in `YYYY-MM-DD` format. This will be null unless the assignee's Copilot access has been canceled during the current billing cycle. If the seat has been cancelled, this corresponds to the start of the organization's next billing cycle. | [optional]
**last_activity_at** | Option<**String**> | Timestamp of user's last GitHub Copilot activity, in ISO 8601 format. | [optional]
**last_activity_editor** | Option<**String**> | Last editor that was used by the user for a GitHub Copilot completion. | [optional]
**created_at** | **String** | Timestamp of when the assignee was last granted access to GitHub Copilot, in ISO 8601 format. | 
**updated_at** | Option<**String**> | Timestamp of when the assignee's GitHub Copilot access was last updated, in ISO 8601 format. | [optional]
**plan_type** | Option<**String**> | The Copilot plan of the organization, or the parent enterprise, when applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


