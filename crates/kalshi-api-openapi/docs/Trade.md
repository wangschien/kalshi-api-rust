# Trade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_id** | **String** | Unique identifier for this trade | 
**ticker** | **String** | Unique identifier for the market | 
**count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**yes_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**taker_side** | Option<**String**> | Deprecated. Use `taker_outcome_side` (or `taker_book_side`) instead. See [Order direction](/getting_started/order_direction). This field will not be removed before May 14, 2026.  | [optional]
**taker_outcome_side** | **String** | The outcome side the taker is positioned for. buy-yes and sell-no produce 'yes'; buy-no and sell-yes produce 'no'.  `taker_outcome_side` describes directional exposure only; it does not change the trade's price. A trade at price `p` with `taker_outcome_side=no` is matched against the maker at the same price `p` with the opposite direction — both parties trade at the same price.  `taker_outcome_side` and `taker_book_side` will become the canonical way to determine trade direction. The legacy `taker_side` field will be deprecated in a future release — please migrate to these new fields.  | 
**taker_book_side** | [**models::BookSide**](BookSide.md) |  | 
**created_time** | **String** | Timestamp when this trade was executed | 
**is_block_trade** | **bool** | True if this trade was matched off-book as a block trade (e.g. via RFQ / negotiated block proposal); false for trades that filled on the standard order book. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


