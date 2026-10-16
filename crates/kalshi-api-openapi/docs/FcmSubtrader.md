# FcmSubtrader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subtrader_id** | **String** | Full subtrader identifier owned by the authenticated FCM. | 
**exchange_indices** | **Vec<i32>** | Exchange indices where this subtrader has been observed. | 
**trading_blocked** | **bool** | Effective trading block, including firm-wide and Kalshi restrictions. | 
**fcm_trading_blocked** | **bool** | Whether an FCM-owned per-subtrader trading block is configured. | 
**propagation_pending** | **bool** | Whether a configured per-subtrader block is awaiting engine application. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


