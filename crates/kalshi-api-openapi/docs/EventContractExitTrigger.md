# EventContractExitTrigger

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**slippage** | Option<[**models::EventContractExitSlippage**](EventContractExitSlippage.md)> |  | [optional]
**id** | **String** |  | 
**ticker** | **String** |  | 
**side** | **String** |  | 
**subaccount_number** | **i64** |  | 
**kind** | **String** |  | 
**stop_loss_price** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**take_profit_price** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**status** | **String** |  | 
**status_reason** | Option<**String**> |  | [optional]
**triggered_order_id** | Option<**String**> |  | [optional]
**triggered_leg** | Option<**String**> |  | [optional]
**created_time** | **String** |  | 
**updated_time** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


