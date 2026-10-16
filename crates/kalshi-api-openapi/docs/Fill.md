# Fill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fill_id** | **String** | Unique identifier for this fill | 
**exchange_index** | **i64** | Identifier for an exchange shard. | 
**trade_id** | **String** | Unique identifier for this fill (legacy field name, same as fill_id) | 
**order_id** | **String** | Unique identifier for the order that resulted in this fill | 
**ticker** | **String** | Unique identifier for the market | 
**market_ticker** | **String** | Unique identifier for the market (legacy field name, same as ticker) | 
**side** | Option<**String**> | Deprecated. Use `outcome_side` (or `book_side`) instead. See [Order direction](/getting_started/order_direction). This field will not be removed before May 14, 2026.  | [optional]
**action** | Option<**String**> | Deprecated. Use `outcome_side` (or `book_side`) instead. See [Order direction](/getting_started/order_direction). This field will not be removed before May 14, 2026.  | [optional]
**outcome_side** | [**models::OutcomeSide**](OutcomeSide.md) |  | 
**book_side** | [**models::BookSide**](BookSide.md) |  | 
**count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**yes_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**is_taker** | **bool** | If true, this fill was a taker (removed liquidity from the order book) | 
**created_time** | Option<**String**> | Timestamp when this fill was executed | [optional]
**fee_cost** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**subaccount_number** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Present for direct users. | [optional]
**ts** | Option<**i64**> | Unix timestamp when this fill was executed (legacy field name) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


