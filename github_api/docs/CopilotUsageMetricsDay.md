# CopilotUsageMetricsDay

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | [**String**](string.md) | The date for which the usage metrics are aggregated, in `YYYY-MM-DD` format. | 
**total_active_users** | Option<**i32**> | The total number of Copilot users with activity belonging to any Copilot feature, globally, for the given day. Includes passive activity such as receiving a code suggestion, as well as engagement activity such as accepting a code suggestion or prompting chat. Does not include authentication events. Is not limited to the individual features detailed on the endpoint. | [optional]
**total_engaged_users** | Option<**i32**> | The total number of Copilot users who engaged with any Copilot feature, for the given day. Examples include but are not limited to accepting a code suggestion, prompting Copilot chat, or triggering a PR Summary. Does not include authentication events. Is not limited to the individual features detailed on the endpoint. | [optional]
**copilot_ide_code_completions** | Option<[**models::CopilotIdeCodeCompletions**](copilot-ide-code-completions.md)> |  | [optional]
**copilot_ide_chat** | Option<[**models::CopilotIdeChat**](copilot-ide-chat.md)> |  | [optional]
**copilot_dotcom_chat** | Option<[**models::CopilotDotcomChat**](copilot-dotcom-chat.md)> |  | [optional]
**copilot_dotcom_pull_requests** | Option<[**models::CopilotDotcomPullRequests**](copilot-dotcom-pull-requests.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


