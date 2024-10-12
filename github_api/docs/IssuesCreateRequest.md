# IssuesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | [**models::IssuesCreateRequestTitle**](issues_create_request_title.md) |  | 
**body** | Option<**String**> | The contents of the issue. | [optional]
**assignee** | Option<**String**> | Login for the user that this issue should be assigned to. _NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. **This field is closing down.**_ | [optional]
**milestone** | Option<[**models::IssuesCreateRequestMilestone**](issues_create_request_milestone.md)> |  | [optional]
**labels** | Option<[**Vec<models::IssuesCreateRequestLabelsInner>**](issues_create_request_labels_inner.md)> | Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._ | [optional]
**assignees** | Option<**Vec<String>**> | Logins for Users to assign to this issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._ | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


