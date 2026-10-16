# Settlement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | The ticker symbol of the market that was settled. | 
**exchange_index** | **i64** | Identifier for an exchange shard. | 
**event_ticker** | **String** | The event ticker symbol of the market that was settled. | 
**market_result** | **String** | The outcome of the market settlement. 'yes' = market resolved to YES, 'no' = market resolved to NO, 'scalar' = scalar market settled at a specific value. | 
**yes_count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**yes_total_cost_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**no_total_cost_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**revenue** | **i64** | Total revenue earned from this settlement in cents (winning contracts pay out 100 cents each). | 
**settled_time** | **String** | Timestamp when the market was settled and payouts were processed. | 
**fee_cost** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**value** | Option<**i64**> | Payout of a single yes contract in cents. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


