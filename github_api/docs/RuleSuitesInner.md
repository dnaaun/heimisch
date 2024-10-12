# RuleSuitesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier of the rule insight. | [optional]
**actor_id** | Option<**i32**> | The number that identifies the user. | [optional]
**actor_name** | Option<**String**> | The handle for the GitHub user account. | [optional]
**before_sha** | Option<**String**> | The first commit sha before the push evaluation. | [optional]
**after_sha** | Option<**String**> | The last commit sha in the push evaluation. | [optional]
**r#ref** | Option<**String**> | The ref name that the evaluation ran on. | [optional]
**repository_id** | Option<**i32**> | The ID of the repository associated with the rule evaluation. | [optional]
**repository_name** | Option<**String**> | The name of the repository without the `.git` extension. | [optional]
**pushed_at** | Option<**String**> |  | [optional]
**result** | Option<**String**> | The result of the rule evaluations for rules with the `active` enforcement status. | [optional]
**evaluation_result** | Option<**String**> | The result of the rule evaluations for rules with the `active` and `evaluate` enforcement statuses, demonstrating whether rules would pass or fail if all rules in the rule suite were `active`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


