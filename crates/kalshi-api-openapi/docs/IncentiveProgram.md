# IncentiveProgram

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the incentive program | 
**market_id** | **String** | The unique identifier of the market associated with this incentive program | 
**market_ticker** | **String** | The ticker symbol of the market associated with this incentive program | 
**incentive_type** | **String** | Type of incentive program | 
**incentive_description** | **String** | Plain text description of the incentive program | 
**start_date** | **String** | Start date of the incentive program | 
**end_date** | **String** | End date of the incentive program | 
**period_reward** | **i64** | Total reward for the period in centi-cents | 
**paid_out** | **bool** | Whether the incentive has been paid out | 
**discount_factor_bps** | Option<**i32**> | Discount factor in basis points (optional) | [optional]
**target_size_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**max_reward_per_account** | Option<**i64**> | Maximum reward per account in centi-cents (optional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


