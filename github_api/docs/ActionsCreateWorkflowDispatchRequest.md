# ActionsCreateWorkflowDispatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#ref** | **String** | The git reference for the workflow. The reference can be a branch or tag name. | 
**inputs** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Input keys and values configured in the workflow file. The maximum number of properties is 10. Any default properties configured in the workflow file will be used when `inputs` are omitted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


