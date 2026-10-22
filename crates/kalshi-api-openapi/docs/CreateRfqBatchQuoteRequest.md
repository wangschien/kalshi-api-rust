# CreateRfqBatchQuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quotes** | [**Vec<models::RfqBatchQuoteEntry>**](RFQBatchQuoteEntry.md) | Prices for every child RFQ of the batch, one entry per child. | 
**rest_remainder** | **bool** | Whether to rest the remainder of each child quote after execution | 
**post_only** | Option<**bool**> | If true, your resting orders are cancelled rather than crossed if they would take liquidity. Defaults to false. | [optional]
**subaccount** | Option<**i64**> | Optional subaccount number to place the batch quote under (0 for primary, 1-63 for subaccounts) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


