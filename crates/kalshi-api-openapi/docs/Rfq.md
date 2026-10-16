# Rfq

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | UUID of the RFQ. Preserve the exact returned string. | 
**creator_id** | **String** | Public communications ID of the RFQ creator (pseudonymous). When obscure_creator_id is enabled, other users receive the shared placeholder \"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\" (SHA-256 of empty input). The creator sees their normal ID. After successful quote execution, subsequent RFQ responses show the normal public communications ID to all viewers. | 
**market_ticker** | **String** | The ticker of the market this RFQ is for | 
**contracts_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**target_cost_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**target_cost_excludes_fees** | Option<**bool**> | True when the target cost is principal-only and quote sizes are computed without reserving taker fees (fees are charged on top). | [optional]
**status** | **String** | Current status of the RFQ (open, closed) | 
**created_ts** | **String** | Timestamp when the RFQ was created | 
**mve_collection_ticker** | Option<**String**> | Ticker of the MVE collection this market belongs to | [optional]
**mve_selected_legs** | Option<[**Vec<models::MveSelectedLeg>**](MveSelectedLeg.md)> | Selected legs for the MVE collection | [optional]
**rest_remainder** | Option<**bool**> | Whether to rest the remainder of the RFQ after execution | [optional]
**cancellation_reason** | Option<**String**> | Reason for RFQ cancellation if cancelled | [optional]
**creator_user_id** | Option<**String**> | User ID of the RFQ creator (private field) | [optional]
**creator_subaccount** | Option<**i64**> | Subaccount number of the RFQ creator (visible when the caller is the RFQ creator) | [optional]
**cancelled_ts** | Option<**String**> | Timestamp when the RFQ was cancelled | [optional]
**updated_ts** | Option<**String**> | Timestamp when the RFQ was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


