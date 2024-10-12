# IssuesUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<[**models::IssuesUpdateRequestTitle**](issues_update_request_title.md)> |  | [optional]
**body** | Option<**String**> | The contents of the issue. | [optional]
**assignee** | Option<**String**> | Username to assign to this issue. **This field is closing down.** | [optional]
**state** | Option<**String**> | The open or closed state of the issue. | [optional]
**state_reason** | Option<**String**> | The reason for the state change. Ignored unless `state` is changed. | [optional]
**milestone** | Option<[**models::IssuesUpdateRequestMilestone**](issues_update_request_milestone.md)> |  | [optional]
**labels** | Option<[**Vec<models::IssuesCreateRequestLabelsInner>**](issues_create_request_labels_inner.md)> | Labels to associate with this issue. Pass one or more labels to _replace_ the set of labels on this issue. Send an empty array (`[]`) to clear all labels from the issue. Only users with push access can set labels for issues. Without push access to the repository, label changes are silently dropped. | [optional]
**assignees** | Option<**Vec<String>**> | Usernames to assign to this issue. Pass one or more user logins to _replace_ the set of assignees on this issue. Send an empty array (`[]`) to clear all assignees from the issue. Only users with push access can set assignees for new issues. Without push access to the repository, assignee changes are silently dropped. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


