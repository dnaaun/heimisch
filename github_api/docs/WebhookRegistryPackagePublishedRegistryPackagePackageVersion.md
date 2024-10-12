# WebhookRegistryPackagePublishedRegistryPackagePackageVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<[**models::WebhookRegistryPackagePublishedRegistryPackageOwner**](webhook_registry_package_published_registry_package_owner.md)> |  | [optional]
**body** | Option<[**models::WebhookPackagePublishedPackagePackageVersionBody**](webhook_package_published_package_package_version_body.md)> |  | [optional]
**body_html** | Option<**String**> |  | [optional]
**container_metadata** | Option<[**models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionContainerMetadata**](webhook_registry_package_published_registry_package_package_version_container_metadata.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**description** | **String** |  | 
**docker_metadata** | Option<[**Vec<models::WebhookPackagePublishedPackagePackageVersionDockerMetadataInner>**](webhook_package_published_package_package_version_docker_metadata_inner.md)> |  | [optional]
**draft** | Option<**bool**> |  | [optional]
**html_url** | **String** |  | 
**id** | **i32** |  | 
**installation_command** | **String** |  | 
**manifest** | Option<**String**> |  | [optional]
**metadata** | [**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md) |  | 
**name** | **String** |  | 
**npm_metadata** | Option<[**models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata**](webhook_registry_package_published_registry_package_package_version_npm_metadata.md)> |  | [optional]
**nuget_metadata** | Option<[**Vec<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInner>**](webhook_registry_package_published_registry_package_package_version_nuget_metadata_inner.md)> |  | [optional]
**package_files** | [**Vec<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionPackageFilesInner>**](webhook_registry_package_published_registry_package_package_version_package_files_inner.md) |  | 
**package_url** | **String** |  | 
**prerelease** | Option<**bool**> |  | [optional]
**release** | Option<[**models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionRelease**](webhook_registry_package_published_registry_package_package_version_release.md)> |  | [optional]
**rubygems_metadata** | Option<[**Vec<models::WebhookRubygemsMetadata>**](webhook-rubygems-metadata.md)> |  | [optional]
**summary** | **String** |  | 
**tag_name** | Option<**String**> |  | [optional]
**target_commitish** | Option<**String**> |  | [optional]
**target_oid** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**version** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


