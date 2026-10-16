# EventFeeChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for this fee change | 
**event_ticker** | **String** | Event ticker this fee change applies to | 
**series_ticker** | **String** | Series ticker for the event | 
**fee_type_override** | Option<[**models::FeeType**](FeeType.md)> | New fee type override for the event. When null, the event clears any prior override and falls back to the parent series' fee structure. | 
**fee_multiplier_override** | Option<**f64**> | New fee multiplier override for the event. When null, the event clears any prior override and falls back to the parent series' fee multiplier. | 
**scheduled_ts** | **String** | Timestamp when this fee change is scheduled to take effect | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


