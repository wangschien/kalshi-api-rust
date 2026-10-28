# CreditHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credit_id** | **String** | Unique credit identifier. | 
**r#type** | **String** | Credit category. | 
**status** | **String** | Credit processing status. | 
**amount_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**reason** | **String** | Credit description, which may include a market or event ticker. | 
**created_time** | **String** | Time the credit was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


