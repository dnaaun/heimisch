# CopilotIdeCodeCompletionsEditorsInnerModelsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the language used for Copilot code completion suggestions, for the given editor. | [optional]
**is_custom_model** | Option<**bool**> | Indicates whether a model is custom or default. | [optional]
**custom_model_training_date** | Option<**String**> | The training date for the custom model. | [optional]
**total_engaged_users** | Option<**i32**> | Number of users who accepted at least one Copilot code completion suggestion for the given editor, for the given language and model. Includes both full and partial acceptances. | [optional]
**languages** | Option<[**Vec<models::CopilotIdeCodeCompletionsEditorsInnerModelsInnerLanguagesInner>**](copilot_ide_code_completions_editors_inner_models_inner_languages_inner.md)> | Code completion metrics for active languages, for the given editor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


