# ReposCreateAutolinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_prefix** | **String** | This prefix appended by certain characters will generate a link any time it is found in an issue, pull request, or commit. | 
**url_template** | **String** | The URL must contain `<num>` for the reference number. `<num>` matches different characters depending on the value of `is_alphanumeric`. | 
**is_alphanumeric** | Option<**bool**> | Whether this autolink reference matches alphanumeric characters. If true, the `<num>` parameter of the `url_template` matches alphanumeric characters `A-Z` (case insensitive), `0-9`, and `-`. If false, this autolink reference only matches numeric characters. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


