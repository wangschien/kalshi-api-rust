# MarketIndexLatest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | **String** | Data source of the value, e.g. `spdji` (S&P Dow Jones Indices) or `massive`. | 
**status** | [**models::MarketIndexStatus**](MarketIndexStatus.md) |  | 
**value** | Option<**f64**> | Latest index value. Absent when no value is available. | [optional]
**last_tick_ms** | Option<**i64**> | Time of `value`, unix milliseconds. Absent with `value`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


