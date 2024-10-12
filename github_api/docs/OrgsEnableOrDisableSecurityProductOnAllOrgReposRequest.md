# OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_suite** | Option<**String**> | CodeQL query suite to be used. If you specify the `query_suite` parameter, the default setup will be configured with this query suite only on all repositories that didn't have default setup already configured. It will not change the query suite on repositories that already have default setup configured. If you don't specify any `query_suite` in your request, the preferred query suite of the organization will be applied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


