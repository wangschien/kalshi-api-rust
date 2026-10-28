# CreateRfqBatchChild

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**market_ticker** | Option<**String**> | Ticker of an existing combo market | [optional]
**selected_markets** | Option<[**Vec<models::TickerPair>**](TickerPair.md)> | Legs of the combo, at most 6, looked up or created in the request's mve_collection_ticker as POST /multivariate_event_collections/{collection_ticker} does. | [optional]
**contracts_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**target_cost_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


