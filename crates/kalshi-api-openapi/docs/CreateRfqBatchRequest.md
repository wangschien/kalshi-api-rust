# CreateRfqBatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**children** | [**Vec<models::CreateRfqBatchChild>**](CreateRFQBatchChild.md) | The combos to request quotes on, one child RFQ each. Children must be distinct combos. | 
**mve_collection_ticker** | Option<**String**> | Collection in which children given by selected_markets are looked up or created. Required when any child uses selected_markets. | [optional]
**contracts_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**target_cost_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**target_cost_excludes_fees** | Option<**bool**> | Same as on Create RFQ, applied to every child. Only valid together with target_cost_dollars. | [optional]
**rest_remainder** | Option<**bool**> | Whether to rest the remainder of each child RFQ after execution | [optional][default to false]
**replace_existing** | Option<**bool**> | Close your oldest open RFQ batches as needed so this one can be created. Single RFQs are never replaced. | [optional][default to false]
**obscure_creator_id** | Option<**bool**> | Same as on Create RFQ, applied to the batch and every child. | [optional][default to false]
**client_batch_id** | Option<**String**> | Optional key for this batch, unique among your RFQ batches, including those closed within the last seven days. Reusing one returns HTTP 409. | [optional]
**subtrader_id** | Option<**String**> | The subtrader to create the RFQ batch for (FCM members only) | [optional]
**subaccount** | Option<**i64**> | The subaccount number to create the RFQ batch for (direct members only; 0 for primary, 1-63 for subaccounts) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


