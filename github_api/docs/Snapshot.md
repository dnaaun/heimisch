# Snapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **i32** | The version of the repository snapshot submission. | 
**job** | [**models::SnapshotJob**](snapshot_job.md) |  | 
**sha** | **String** | The commit SHA associated with this dependency snapshot. Maximum length: 40 characters. | 
**r#ref** | **String** | The repository branch that triggered this snapshot. | 
**detector** | [**models::SnapshotDetector**](snapshot_detector.md) |  | 
**metadata** | Option<[**std::collections::HashMap<String, models::MetadataValue>**](metadata_value.md)> | User-defined metadata to store domain-specific information limited to 8 keys with scalar values. | [optional]
**manifests** | Option<[**std::collections::HashMap<String, models::Manifest>**](manifest.md)> | A collection of package manifests, which are a collection of related dependencies declared in a file or representing a logical group of dependencies. | [optional]
**scanned** | **String** | The time at which the snapshot was scanned. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


