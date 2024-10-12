# CopilotIdeChatEditorsInnerModelsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the language used for Copilot code completion suggestions, for the given editor. | [optional]
**is_custom_model** | Option<**bool**> | Indicates whether a model is custom or default. | [optional]
**custom_model_training_date** | Option<**String**> | The training date for the custom model. | [optional]
**total_engaged_users** | Option<**i32**> | The number of users who prompted Copilot Chat in the given editor and model. | [optional]
**total_chats** | Option<**i32**> | The total number of chats initiated by users in the given editor and model. | [optional]
**total_chat_insertion_events** | Option<**i32**> | The number of times users accepted a code suggestion from Copilot Chat using the 'Insert Code' UI element, for the given editor. | [optional]
**total_chat_copy_events** | Option<**i32**> | The number of times users copied a code suggestion from Copilot Chat using the keyboard, or the 'Copy' UI element, for the given editor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


