# EventData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_ticker** | **String** | Unique identifier for this event. | 
**series_ticker** | **String** | Unique identifier for the series this event belongs to. | 
**sub_title** | **String** | Shortened descriptive title for the event. | 
**title** | **String** | Full title of the event. | 
**collateral_return_type** | **String** | Collateral-return netting type for this event: `MECNET` for mutually exclusive markets, `DIRECNET` for directional netting, or an empty string for no collateral-return netting. Netting applies within this event, not across all events in its series, and also requires netting to be enabled for the trading account or subtrader. Use `with_nested_markets=true` to retrieve the markets associated with each event.  | 
**mutually_exclusive** | **bool** | True when `collateral_return_type` is `MECNET`: at most one market in this event can resolve to 'yes'. False for both `DIRECNET` and an empty collateral-return type. A false value does not mean collateral-return netting is unavailable. Use `collateral_return_type` to distinguish these cases.  | 
**category** | Option<**String**> | Event category (deprecated, use series-level category instead). | [optional]
**strike_date** | Option<**String**> | The specific date this event is based on. Only filled when the event uses a date strike (mutually exclusive with strike_period). | [optional]
**strike_period** | Option<**String**> | The time period this event covers (e.g., 'week', 'month'). Only filled when the event uses a period strike (mutually exclusive with strike_date). | [optional]
**markets** | Option<[**Vec<models::Market>**](Market.md)> | Array of markets associated with this event. Only populated when 'with_nested_markets=true' is specified in the request. | [optional]
**product_metadata** | Option<[**serde_json::Value**](.md)> | Additional metadata for the event. | [optional]
**settlement_sources** | Option<[**Vec<models::SettlementSource>**](SettlementSource.md)> | The official sources used for the determination of markets within this event. Methodology is defined in the rulebook. | 
**last_updated_ts** | Option<**String**> | Timestamp of when this event's metadata was last updated. | [optional]
**fee_type_override** | Option<**String**> | Fee type override for this event. When present, takes precedence over the series-level fee for this event's markets. | [optional]
**fee_multiplier_override** | Option<**f64**> | Fee multiplier override for this event. Paired with fee_type_override. | [optional]
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


