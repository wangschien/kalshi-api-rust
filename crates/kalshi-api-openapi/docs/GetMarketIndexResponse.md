# GetMarketIndexResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | [**models::MarketIndex**](MarketIndex.md) |  | 
**source** | **String** | Data source of `timeseries`. | 
**status** | [**models::MarketIndexStatus**](MarketIndexStatus.md) |  | 
**value** | Option<**f64**> | Latest index value. Absent when no value is available. | [optional]
**last_tick_ms** | Option<**i64**> | Time of `value`, unix milliseconds. | [optional]
**interval** | **String** | Point spacing of `timeseries` (`1m`). | 
**start_ms** | **i64** | Window start, unix milliseconds. | 
**end_ms** | **i64** | Window end, unix milliseconds. | 
**timeseries** | [**Vec<models::MarketIndexTimeseriesPoint>**](MarketIndexTimeseriesPoint.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


