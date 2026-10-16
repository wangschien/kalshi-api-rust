# StructuredTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the structured target. | [optional]
**name** | Option<**String**> | Name of the structured target. | [optional]
**r#type** | Option<**String**> | Type of the structured target. | [optional]
**details** | Option<[**serde_json::Value**](.md)> | Additional details about the structured target. Contains flexible JSON data specific to the target type. | [optional]
**source_id** | Option<**String**> | External source identifier for the structured target, if available (e.g., third-party data provider ID). | [optional]
**source_ids** | Option<**std::collections::HashMap<String, String>**> | Source ids of structured target if available. | [optional]
**last_updated_ts** | Option<**String**> | Timestamp when this structured target was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


