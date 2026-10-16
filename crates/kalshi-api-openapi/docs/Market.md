# Market

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** |  | 
**event_ticker** | **String** |  | 
**market_type** | **String** | Identifies the type of market | 
**title** | Option<**String**> |  | [optional]
**subtitle** | Option<**String**> |  | [optional]
**yes_sub_title** | **String** | Shortened title for the yes side of this market | 
**no_sub_title** | **String** | Shortened title for the no side of this market | 
**created_time** | **String** |  | 
**updated_time** | **String** | Time of the last non-trading metadata update. | 
**open_time** | **String** |  | 
**close_time** | **String** |  | 
**expected_expiration_time** | Option<**String**> | Time when this market is expected to expire | [optional]
**expiration_time** | Option<**String**> |  | [optional]
**latest_expiration_time** | **String** | Latest possible time for this market to expire | 
**settlement_timer_seconds** | **i64** | The amount of time after determination that the market settles | 
**status** | **String** | The current status of the market in its lifecycle. | 
**yes_bid_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**yes_bid_size_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**yes_ask_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**yes_ask_size_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**no_bid_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_ask_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**last_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**volume_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**volume_24h_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**result** | **String** |  | 
**can_close_early** | **bool** |  | 
**open_interest_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**notional_value_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**previous_yes_bid_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**previous_yes_ask_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**previous_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**settlement_value_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**settlement_ts** | Option<**String**> | Timestamp when the market was settled. Only filled for settled markets | [optional]
**settlement_bounds_type** | **String** | Which settlement bounds apply to this market. default means the market has no settlement bounds | 
**settlement_floor_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**expiration_value** | **String** | The value that was considered for the settlement | 
**occurrence_datetime** | Option<**String**> | The recorded datetime when the underlying event occurred, if available | [optional]
**fee_waiver_expiration_time** | Option<**String**> | Time when this market's fee waiver expires | [optional]
**early_close_condition** | Option<**String**> | The condition under which the market can close early | [optional]
**strike_type** | Option<**String**> | Strike type defines how the market strike is defined and evaluated | [optional]
**floor_strike** | Option<**f64**> | Minimum expiration value that leads to a YES settlement | [optional]
**cap_strike** | Option<**f64**> | Maximum expiration value that leads to a YES settlement | [optional]
**functional_strike** | Option<**String**> | Mapping from expiration values to settlement values | [optional]
**custom_strike** | Option<[**serde_json::Value**](.md)> | Expiration value for each target that leads to a YES settlement | [optional]
**rules_primary** | **String** | A plain language description of the most important market terms | 
**rules_secondary** | **String** | A plain language description of secondary market terms | 
**mve_collection_ticker** | Option<**String**> | The ticker of the multivariate event collection | [optional]
**mve_selected_legs** | Option<[**Vec<models::MveSelectedLeg>**](MveSelectedLeg.md)> |  | [optional]
**primary_participant_key** | Option<**String**> |  | [optional]
**price_level_structure** | **String** | Price level structure for this market, defining price ranges and tick sizes | 
**price_ranges** | [**Vec<models::PriceRange>**](PriceRange.md) | Valid price ranges for orders on this market | 
**is_provisional** | Option<**bool**> | If true, the market may be removed after determination if there is no activity on it | [optional]
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


