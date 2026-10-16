# Series

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | Ticker that identifies this series. | 
**frequency** | **String** | Description of the frequency of the series. There is no fixed value set here, but will be something human-readable like weekly, daily, one-off. | 
**title** | **String** | Title describing the series. For full context use you should use this field with the title field of the events belonging to this series. | 
**category** | **String** | Category is the primary category of this series. | 
**categories** | **Vec<String>** | Categories is the list of discovery categories for this series. The `category` filter on Get Series List matches any entry in this list. May be empty. | 
**tags** | Option<**Vec<String>**> | Tags specifies the subjects that this series relates to, multiple series from different categories can have the same tags. | 
**settlement_sources** | Option<[**Vec<models::SettlementSource>**](SettlementSource.md)> | SettlementSources specifies the official sources used for the determination of markets within the series. Methodology is defined in the rulebook. | 
**contract_url** | **String** | ContractUrl provides a direct link to the original filing of the contract which underlies the series. | 
**contract_terms_url** | **String** | ContractTermsUrl is the URL to the current terms of the contract underlying the series. | 
**product_metadata** | Option<[**serde_json::Value**](.md)> | Internal product metadata of the series. | [optional]
**fee_type** | [**models::FeeType**](FeeType.md) | FeeType is a string representing the series' fee structure. Fee structures can be found at https://kalshi.com/docs/kalshi-fee-schedule.pdf. 'quadratic' is described by the General Trading Fees Table, 'quadratic_with_maker_fees' is described by the General Trading Fees Table with maker fees described in the Maker Fees section, 'quadratic_with_combo_maker_fees' is the same maker-fee structure with a 0.5 maker multiplier instead of 0.25, 'flat' is described by the Specific Trading Fees Table. | 
**fee_multiplier** | **f64** | FeeMultiplier is a floating point multiplier applied to the fee calculations. | 
**additional_prohibitions** | Option<**Vec<String>**> | AdditionalProhibitions is a list of additional trading prohibitions for this series. | 
**volume_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**last_updated_ts** | Option<**String**> | Timestamp of when this series' metadata was last updated. | [optional]
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


