# Dependency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_url** | Option<**String**> | Package-url (PURL) of dependency. See https://github.com/package-url/purl-spec for more details. | [optional]
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue>**](metadata_value.md)> | User-defined metadata to store domain-specific information limited to 8 keys with scalar values. | [optional]
**relationship** | Option<**String**> | A notation of whether a dependency is requested directly by this manifest or is a dependency of another dependency. | [optional]
**scope** | Option<**String**> | A notation of whether the dependency is required for the primary build artifact (runtime) or is only used for development. Future versions of this specification may allow for more granular scopes. | [optional]
**dependencies** | Option<**Vec<String>**> | Array of package-url (PURLs) of direct child dependencies. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


