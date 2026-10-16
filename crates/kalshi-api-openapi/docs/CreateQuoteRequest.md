# CreateQuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_id** | **String** | The UUID of the RFQ to quote on. Pass the RFQ ID unchanged; malformed IDs return HTTP 400. | 
**yes_bid** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_bid** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**rest_remainder** | **bool** | Whether to rest the remainder of the quote after execution | 
**post_only** | Option<**bool**> | If true, the quote creator's resting order will be cancelled rather than crossed if it would take liquidity. Defaults to false. | [optional]
**subaccount** | Option<**i64**> | Optional subaccount number to place the quote under (0 for primary, 1-63 for subaccounts) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


