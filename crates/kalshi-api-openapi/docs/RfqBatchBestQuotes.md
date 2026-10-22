# RfqBatchBestQuotes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | UUID of the RFQ batch | 
**status** | [**models::RfqBatchStatus**](RFQBatchStatus.md) |  | 
**version** | **i64** | Increases whenever a best bid changes | 
**updated_ts** | Option<**String**> | When a best bid last changed | [optional]
**yes_complete** | **bool** | Whether every child has a YES bid, so the batch can be accepted on the yes side | 
**no_complete** | **bool** | Whether every child has a NO bid, so the batch can be accepted on the no side | 
**rfqs** | [**Vec<models::RfqBatchBestQuote>**](RFQBatchBestQuote.md) | One entry per child RFQ, in batch order | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


