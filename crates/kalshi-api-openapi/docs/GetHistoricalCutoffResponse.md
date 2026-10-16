# GetHistoricalCutoffResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**market_settled_ts** | **String** | Cutoff based on **market settlement time**. Markets and their candlesticks that settled before this timestamp must be accessed via `GET /historical/markets` and `GET /historical/markets/{ticker}/candlesticks`.  | 
**trades_created_ts** | **String** | Cutoff based on **trade fill time**. Fills that occurred before this timestamp must be accessed via `GET /historical/fills`.  | 
**orders_updated_ts** | **String** | Cutoff based on **order cancellation or execution time**. Orders canceled or fully executed before this timestamp must be accessed via `GET /historical/orders`. Resting (active) orders are always available in `GET /portfolio/orders`.  | 
**market_positions_last_updated_ts** | Option<**String**> | Cutoff based on **position last-update time**. Settled positions archived from the live data set before this timestamp are served through the historical section of position reads.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


