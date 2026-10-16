# WeatherIndexCalibration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_version** | **String** | Configuration version (e.g. `miami-temperature-v1.0-cal-20260831`). Index points report the version they were computed under in their `config_version` field. | 
**published_at_ms** | Option<**i64**> | When the record was published, unix milliseconds UTC. | [optional]
**effective_at_ms** | **i64** | The record governs event minutes at or after this time (unix milliseconds UTC), until superseded by the next record. | 
**change_reason** | Option<**String**> | Why the configuration changed. | [optional]
**calibration_window_start_ms** | Option<**i64**> | Start of the trailing observation window the offsets were estimated from. Absent on records not derived from a calibration window (the launch configuration). | [optional]
**calibration_window_end_ms** | Option<**i64**> | End of the calibration window (exclusive). | [optional]
**city_reference_c** | **f64** | City reference B_c in Celsius: the weight-dot-offset sum over all configured member stations. | 
**stations** | [**Vec<models::WeatherIndexCalibrationStation>**](WeatherIndexCalibrationStation.md) | Configured member stations, in configuration order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


