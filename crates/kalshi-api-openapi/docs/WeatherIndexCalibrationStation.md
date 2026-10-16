# WeatherIndexCalibrationStation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**station_id** | **String** | Member station ID (e.g. `KMIA1M`). | 
**weight** | **f64** | Base weight (weights sum to 1.0 across members). | 
**offset_c** | **f64** | Station offset in Celsius (positive = station normally runs warmer than its peers). | 
**update_note** | Option<**String**> | Weekly-calibration disposition, present only on weekly calibration records: `updated ...` with the residual count, target, and applied adjustment, or `insufficient ...` when the prior offset was retained. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


