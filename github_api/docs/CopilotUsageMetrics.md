# CopilotUsageMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day** | [**String**](string.md) | The date for which the usage metrics are reported, in `YYYY-MM-DD` format. | 
**total_suggestions_count** | Option<**i32**> | The total number of Copilot code completion suggestions shown to users. | [optional]
**total_acceptances_count** | Option<**i32**> | The total number of Copilot code completion suggestions accepted by users. | [optional]
**total_lines_suggested** | Option<**i32**> | The total number of lines of code completions suggested by Copilot. | [optional]
**total_lines_accepted** | Option<**i32**> | The total number of lines of code completions accepted by users. | [optional]
**total_active_users** | Option<**i32**> | The total number of users who were shown Copilot code completion suggestions during the day specified. | [optional]
**total_chat_acceptances** | Option<**i32**> | The total instances of users who accepted code suggested by Copilot Chat in the IDE (panel and inline). | [optional]
**total_chat_turns** | Option<**i32**> | The total number of chat turns (prompt and response pairs) sent between users and Copilot Chat in the IDE. | [optional]
**total_active_chat_users** | Option<**i32**> | The total number of users who interacted with Copilot Chat in the IDE during the day specified. | [optional]
**breakdown** | Option<[**Vec<models::CopilotUsageMetricsBreakdownInner>**](copilot_usage_metrics_breakdown_inner.md)> | Breakdown of Copilot code completions usage by language and editor | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


