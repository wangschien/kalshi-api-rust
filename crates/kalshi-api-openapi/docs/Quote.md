# Quote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | UUID of the quote. Preserve the exact returned string. | 
**rfq_id** | **String** | UUID of the RFQ this quote is responding to. | 
**creator_id** | **String** | Public communications ID of the quote creator | 
**rfq_creator_id** | **String** | Public communications ID of the RFQ creator (pseudonymous). When obscure_creator_id is enabled, other users receive the shared placeholder \"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\" (SHA-256 of empty input). The creator sees their normal ID. After this quote executes successfully, its counterparty sees the normal public communications ID. | 
**market_ticker** | **String** | The ticker of the market this quote is for | 
**contracts_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**yes_bid_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_bid_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**created_ts** | **String** | Timestamp when the quote was created | 
**updated_ts** | **String** | Timestamp when the quote was last updated | 
**status** | **String** | Current status of the quote | 
**accepted_side** | Option<**String**> | The side that was accepted (yes or no) | [optional]
**accepted_ts** | Option<**String**> | Timestamp when the quote was accepted | [optional]
**confirmed_ts** | Option<**String**> | Timestamp when the quote was confirmed | [optional]
**executed_ts** | Option<**String**> | Timestamp when the quote was executed | [optional]
**cancelled_ts** | Option<**String**> | Timestamp when the quote was cancelled | [optional]
**rest_remainder** | Option<**bool**> | Whether to rest the remainder of the quote after execution | [optional]
**post_only** | Option<**bool**> | Whether the quote creator's order is post-only (visible when the caller is the quote creator) | [optional]
**cancellation_reason** | Option<**String**> | Reason for quote cancellation if cancelled | [optional]
**creator_user_id** | Option<**String**> | User ID of the quote creator (private field) | [optional]
**rfq_creator_user_id** | Option<**String**> | User ID of the RFQ creator (private field) | [optional]
**rfq_target_cost_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**target_cost_excludes_fees** | Option<**bool**> | True when the RFQ's target cost is principal-only and the contracts-offered sizes were computed without reserving taker fees (fees are charged on top of the target cost). | [optional]
**rfq_creator_order_id** | Option<**String**> | Order ID for the RFQ creator (private field) | [optional]
**creator_order_id** | Option<**String**> | Order ID for the quote creator (private field) | [optional]
**creator_subaccount** | Option<**i64**> | Subaccount number of the quote creator (visible when the caller is the quote creator) | [optional]
**rfq_creator_subaccount** | Option<**i64**> | Subaccount number of the RFQ creator (visible when the caller is the RFQ creator) | [optional]
**yes_contracts_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**no_contracts_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


