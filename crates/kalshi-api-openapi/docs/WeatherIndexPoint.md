# WeatherIndexPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**t** | **i64** | Event minute, unix milliseconds UTC. | 
**v** | Option<**f64**> | Published index value, Fahrenheit rounded to 0.01. Absent on `incomplete` points, which have no canonical value yet. | [optional]
**status** | **String** | `normal` (every member contributed its exact-minute primary observation) or `degraded` (a member was absent, fallback-fed, or substituted by quality control; the value is equally settlement-eligible). With `detailed=true`, trailing minutes still inside their receipt deadline are additionally served as `incomplete`: no value, and stations carrying the raw readings recorded so far (code `pending`, not yet quality-controlled — more readings may still arrive, and the canonical value may differ). | 
**contributors** | Option<**i64**> | Number of accepted member stations backing the point. Absent on `incomplete` points. | [optional]
**receipt_basis** | Option<**String**> | Present only on points produced by the labelled historical backfill that seeds a city's series for the period before it went live. `synoptic_latency` means the receipt-deadline test used `observation_time + Synoptic ingest latency` in place of Kalshi's local receipt clock. Absent on canonical points, which are the only settlement-eligible ones. | [optional]
**stations** | Option<[**Vec<models::WeatherIndexStationReading>**](WeatherIndexStationReading.md)> | Per-station audit readings (only with `detailed=true`), sorted by station ID — every configured member's reported value and QC disposition before incorporation into the index. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


