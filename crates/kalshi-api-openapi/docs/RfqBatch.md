# RfqBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | UUID of the RFQ batch. Preserve the exact returned string. | 
**creator_id** | **String** | Public communications ID of the batch creator, obscured for other users like RFQ.creator_id when obscure_creator_id was set. It is revealed once a child RFQ executes. | 
**status** | [**models::RfqBatchStatus**](RFQBatchStatus.md) |  | 
**created_ts** | **String** | Timestamp when the RFQ batch was created | 
**accepted_ts** | Option<**String**> | Timestamp when a batch quote on the batch was accepted | [optional]
**closed_ts** | Option<**String**> | Timestamp when the RFQ batch closed | [optional]
**rfqs** | [**Vec<models::Rfq>**](RFQ.md) | The child RFQs, in the order they were requested | 
**updated_ts** | Option<**String**> | Timestamp when the RFQ batch was last updated (visible to the batch creator) | [optional]
**close_reason** | Option<**String**> | Why the RFQ batch closed, for example executed, execution failed, or cancelled. A batch closes partially executed only in the rare case that filling one child cancelled a maker order another child needed (visible to the batch creator) | [optional]
**client_batch_id** | Option<**String**> | The client_batch_id given at creation (visible to the batch creator) | [optional]
**creator_user_id** | Option<**String**> | User ID of the batch creator (private field) | [optional]
**creator_subaccount** | Option<**i64**> | Subaccount number of the batch creator (visible to the batch creator) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


