# GetMarketIndexHistoryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Market index ID. | 
**source** | **String** | Data source of `timeseries`. | 
**status** | [**models::MarketIndexStatus**](MarketIndexStatus.md) |  | 
**last_tick_ms** | Option<**i64**> | Time of the index's latest value, unix milliseconds. | [optional]
**interval** | **String** | Point spacing of `timeseries` (`1s` or `1m`). | 
**start_ms** | **i64** | Window start, unix milliseconds. | 
**end_ms** | **i64** | Window end, unix milliseconds. | 
**timeseries** | [**Vec<models::MarketIndexTimeseriesPoint>**](MarketIndexTimeseriesPoint.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


