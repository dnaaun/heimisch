# ReposCreateOrgRulesetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the ruleset. | 
**target** | Option<**String**> | The target of the ruleset | [optional][default to Branch]
**enforcement** | [**models::RepositoryRuleEnforcement**](repository-rule-enforcement.md) |  | 
**bypass_actors** | Option<[**Vec<models::RepositoryRulesetBypassActor>**](repository-ruleset-bypass-actor.md)> | The actors that can bypass the rules in this ruleset | [optional]
**conditions** | Option<[**models::OrgRulesetConditions**](org-ruleset-conditions.md)> |  | [optional]
**rules** | Option<[**Vec<models::RepositoryRule>**](repository-rule.md)> | An array of rules within the ruleset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


