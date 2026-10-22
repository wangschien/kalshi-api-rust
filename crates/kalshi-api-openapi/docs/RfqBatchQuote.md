# RfqBatchQuote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | UUID of the batch quote. Preserve the exact returned string. | 
**rfq_batch_id** | **String** | UUID of the RFQ batch this batch quote prices | 
**creator_id** | **String** | Public communications ID of the batch quote creator | 
**rfq_creator_id** | **String** | Public communications ID of the RFQ batch creator, obscured like Quote.rfq_creator_id | 
**status** | [**models::RfqBatchQuoteStatus**](RFQBatchQuoteStatus.md) |  | 
**created_ts** | **String** | Timestamp when the batch quote was created | 
**updated_ts** | **String** | Timestamp when the batch quote was last updated | 
**accepted_side** | Option<**String**> | The side that was accepted on this maker's children (yes or no) | [optional]
**accepted_ts** | Option<**String**> | Timestamp when the batch quote was accepted | [optional]
**confirmed_ts** | Option<**String**> | Timestamp when the batch quote was confirmed | [optional]
**quotes** | [**Vec<models::Quote>**](Quote.md) | One quote per child RFQ, or once accepted one per child this maker won; match them to children by rfq_id. Each accepted child quote executes as its own trade. | 
**rest_remainder** | Option<**bool**> | Whether to rest the remainder of each child quote after execution (visible to the batch quote creator) | [optional]
**post_only** | Option<**bool**> | Whether the batch quote creator's orders are post-only (visible to the batch quote creator) | [optional]
**creator_user_id** | Option<**String**> | User ID of the batch quote creator (private field) | [optional]
**creator_subaccount** | Option<**i64**> | Subaccount number of the batch quote creator (visible to the batch quote creator) | [optional]
**rfq_creator_user_id** | Option<**String**> | User ID of the RFQ batch creator (private field) | [optional]
**rfq_creator_subaccount** | Option<**i64**> | Subaccount number of the RFQ batch creator (visible to the RFQ batch creator) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


