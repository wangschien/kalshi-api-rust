# ForecastPercentilesPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_ticker** | **String** | The event ticker this forecast is for. | 
**end_period_ts** | **i64** | Unix timestamp for the inclusive end of the forecast period. | 
**period_interval** | **i32** | Length of the forecast period in minutes. | 
**percentile_points** | [**Vec<models::PercentilePoint>**](PercentilePoint.md) | Array of forecast values at different percentiles. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


