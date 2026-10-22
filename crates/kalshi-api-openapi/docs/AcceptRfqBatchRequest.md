# AcceptRfqBatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_side** | **String** | The side to accept on every child (yes or no) | 
**min_bids** | [**Vec<models::RfqBatchAcceptLimit>**](RFQBatchAcceptLimit.md) | Price protection, exactly one entry per child. A child whose best bid on the accepted side is now below its entry fails the accept. | 
**client_order_id** | Option<**String**> | Optional idempotency key. Retrying an accept with the same client_order_id returns the result of the accept that succeeded. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


