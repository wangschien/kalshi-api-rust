# CreateOrderGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount** | Option<**i64**> | Optional subaccount number to use for this order group (0 for primary, 1-63 for subaccounts). Subaccount-restricted API keys must omit this field or pass their locked subaccount. | [optional]
**contracts_limit** | Option<**i64**> | Specifies the maximum number of contracts that can be matched within this group over a rolling 15-second window. Whole contracts only. Provide contracts_limit or contracts_limit_fp; if both provided they must match. | [optional]
**contracts_limit_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. Defaults to 0. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


