# GetWeatherIndexResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**city** | **String** | Index city ID. | 
**config_version** | Option<**String**> | Index configuration version of the newest returned point (e.g. `miami-temperature-v1.0`). Empty when no points matched the window. | [optional]
**units** | **String** | Always `fahrenheit`. | 
**timeseries** | [**Vec<models::WeatherIndexPoint>**](WeatherIndexPoint.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


