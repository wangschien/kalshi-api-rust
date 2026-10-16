# GetEventCandlesticksResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**market_tickers** | **Vec<String>** | Array of market tickers in the event. | 
**market_candlesticks** | [**Vec<Vec<models::MarketCandlestick>>**](Vec.md) | Array of market candlestick arrays, one for each market in the event. | 
**adjusted_end_ts** | **i64** | Adjusted end timestamp if the requested candlesticks would be larger than maxAggregateCandidates. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


