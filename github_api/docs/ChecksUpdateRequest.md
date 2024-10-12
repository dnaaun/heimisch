# ChecksUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the check. For example, \"code-coverage\". | [optional]
**details_url** | Option<**String**> | The URL of the integrator's site that has the full details of the check. | [optional]
**external_id** | Option<**String**> | A reference for the run on the integrator's system. | [optional]
**started_at** | Option<**String**> | This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional]
**status** | Option<**String**> | The current status of the check run. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`. | [optional]
**conclusion** | Option<**String**> | **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check.  **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this. | [optional]
**completed_at** | Option<**String**> | The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional]
**output** | Option<[**models::ChecksUpdateRequestOutput**](checks_update_request_output.md)> |  | [optional]
**actions** | Option<[**Vec<models::ChecksCreateRequestActionsInner>**](checks_create_request_actions_inner.md)> | Possible further actions the integrator can perform, which a user may trigger. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/guides/using-the-rest-api-to-interact-with-checks#check-runs-and-requested-actions).\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


