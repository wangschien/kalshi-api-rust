# UpdateOrderGroupLimitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contracts_limit** | Option<**i64**> | New maximum number of contracts that can be matched within this group over a rolling 15-second window. Whole contracts only. Provide contracts_limit or contracts_limit_fp; if both provided they must match. | [optional]
**contracts_limit_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


