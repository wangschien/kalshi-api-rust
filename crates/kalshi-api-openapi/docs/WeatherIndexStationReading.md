# WeatherIndexStationReading

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**station_id** | **String** | Member station (e.g. `KMIA1M`) or its official fallback ID. | 
**code** | **String** | Disposition: `ok` (accepted), `missing` (no eligible observation), `late` (received after the deadline; diagnostic only), a QC rejection (`range`, `rate_spatial`, `extreme`), or `pending` (raw reading on an `incomplete` minute, not yet quality-controlled). | 
**source** | Option<**String**> | `hf_asos` (exact-minute primary) or `metar` (carried-forward official observation). Absent when no reading was available. | [optional]
**temp_f** | Option<**f64**> | Raw reported temperature in Fahrenheit (unrounded — only the published index value carries output rounding). Absent for `missing` members. | [optional]
**obs_time_ms** | Option<**i64**> | Observation time for carried-forward fallbacks (differs from the event minute). Absent for exact-minute primaries. | [optional]
**received_at_ms** | Option<**i64**> | Local wire-receipt time backing the eligibility deadline. | [optional]
**primary_code** | Option<**String**> | Why the primary observation was passed over when a fallback was selected instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


