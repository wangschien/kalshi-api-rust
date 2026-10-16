# CreateRfqRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**market_ticker** | **String** | The ticker of the market for which to create an RFQ | 
**contracts** | Option<**i64**> | Whole-contract count for the RFQ. Use contracts_fp for partial contract values; if both are provided, they must match. | [optional]
**contracts_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**target_cost_centi_cents** | Option<**i64**> | DEPRECATED: The target cost for the RFQ in centi-cents. Use target_cost_dollars instead. | [optional]
**target_cost_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**target_cost_excludes_fees** | Option<**bool**> | Sizes quotes against the target cost as principal only (contracts = target cost / price), with your taker fees charged on top of the target cost. By default (false) the target cost caps principal plus Kalshi fees, and quote sizes are reduced to make room for the fees. Only valid together with a target cost. | [optional]
**obscure_creator_id** | Option<**bool**> | Replace the RFQ creator's public communications ID with a shared empty-input SHA-256 hash for other users. Applies to RFQ responses and broadcasts, and to quotes before successful execution. Successful execution reveals the normal ID in subsequent RFQ responses and in the executed quote. Accepting or confirming a quote does not reveal the ID. The creator always sees their normal ID. See the RFQ guide for the placeholder and visibility rules. | [optional][default to false]
**rest_remainder** | **bool** | Whether to rest the remainder of the RFQ after execution | 
**replace_existing** | Option<**bool**> | Whether to delete existing RFQs as part of this RFQ's creation | [optional][default to false]
**subtrader_id** | Option<**String**> | The subtrader to create the RFQ for (FCM members only) | [optional]
**subaccount** | Option<**i64**> | The subaccount number to create the RFQ for (direct members only; 0 for primary, 1-63 for subaccounts) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


