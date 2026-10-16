# GetWeatherIndexCalibrationsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**city** | **String** | Index city ID. | 
**units** | **String** | Always `celsius` — offsets and the city reference are Celsius quantities from the index methodology (the published index value itself is Fahrenheit). | 
**calibrations** | [**Vec<models::WeatherIndexCalibration>**](WeatherIndexCalibration.md) | Configuration records, ascending by effective time. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


