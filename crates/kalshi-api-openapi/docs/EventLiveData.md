# EventLiveData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of live data. Names the schema of the details object. | 
**details** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Live data details as a flexible object whose shape depends on the type. | 
**is_historical** | Option<**bool**> | Present for crypto live data. True when the event has matured and the payload is a frozen historical snapshot. | [optional]
**default_range** | Option<**String**> | Chart range the client should default to (e.g. `15min`, `1h`). Omitted when unset. | [optional]
**range_options** | Option<**Vec<String>**> | Chart range menu options. Omitted when unset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


