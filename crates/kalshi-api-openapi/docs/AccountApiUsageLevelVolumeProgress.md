# AccountApiUsageLevelVolumeProgress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**computed_ts** | **i64** | Unix timestamp (seconds) when this progress was computed; trailing_30d_volume_fp covers the trailing 30 days ending at this time. | 
**trailing_30d_volume_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**goals** | [**Vec<models::AccountApiUsageLevelVolumeGoal>**](AccountApiUsageLevelVolumeGoal.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


