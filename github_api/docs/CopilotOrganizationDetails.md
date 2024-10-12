# CopilotOrganizationDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seat_breakdown** | [**models::CopilotSeatBreakdown**](copilot-seat-breakdown.md) |  | 
**public_code_suggestions** | **String** | The organization policy for allowing or disallowing Copilot to make suggestions that match public code. | 
**ide_chat** | Option<**String**> | The organization policy for allowing or disallowing organization members to use Copilot Chat within their editor. | [optional]
**platform_chat** | Option<**String**> | The organization policy for allowing or disallowing organization members to use Copilot features within github.com. | [optional]
**cli** | Option<**String**> | The organization policy for allowing or disallowing organization members to use Copilot within their CLI. | [optional]
**seat_management_setting** | **String** | The mode of assigning new seats. | 
**plan_type** | Option<**String**> | The Copilot plan of the organization, or the parent enterprise, when applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


