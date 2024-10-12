# RepositoryRulesetBypassActor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_id** | Option<**i32**> | The ID of the actor that can bypass a ruleset. If `actor_type` is `OrganizationAdmin`, this should be `1`. If `actor_type` is `DeployKey`, this should be null. `OrganizationAdmin` is not applicable for personal repositories. | [optional]
**actor_type** | **String** | The type of actor that can bypass a ruleset. | 
**bypass_mode** | Option<**String**> | When the specified actor can bypass the ruleset. `pull_request` means that an actor can only bypass rules on pull requests. `pull_request` is not applicable for the `DeployKey` actor type. Also, `pull_request` is only applicable to branch rulesets. | [optional][default to Always]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


