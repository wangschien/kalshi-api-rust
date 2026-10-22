# \LiveDataApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event_live_data**](LiveDataApi.md#get_event_live_data) | **GET** /live_data/events/{event_ticker} | Get Event Live Data
[**get_game_stats**](LiveDataApi.md#get_game_stats) | **GET** /live_data/milestone/{milestone_id}/game_stats | Get Game Stats
[**get_live_data**](LiveDataApi.md#get_live_data) | **GET** /live_data/{type}/milestone/{milestone_id} | Get Live Data (with type)
[**get_live_data_by_milestone**](LiveDataApi.md#get_live_data_by_milestone) | **GET** /live_data/milestone/{milestone_id} | Get Live Data
[**get_live_datas**](LiveDataApi.md#get_live_datas) | **GET** /live_data/batch | Get Multiple Live Data
[**get_weather_index**](LiveDataApi.md#get_weather_index) | **GET** /live_data/weather/{city} | Get Weather Index
[**get_weather_index_calibrations**](LiveDataApi.md#get_weather_index_calibrations) | **GET** /live_data/weather/{city}/calibrations | Get Weather Index Calibrations



## get_event_live_data

> models::GetEventLiveDataResponse get_event_live_data(event_ticker, range)
Get Event Live Data

Get live data for an event by its event ticker. Serves event-keyed live data such as crypto price charts, commodity price timeseries, and weather observations. The `type` field in the response names the schema of the `details` object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_ticker** | **String** | Event ticker | [required] |
**range** | Option<**String**> | Optional chart range hint (e.g. `15min`, `1h`, `1d`). When the underlying live data type supports it, restricts the returned timeseries to the requested window. |  |

### Return type

[**models::GetEventLiveDataResponse**](GetEventLiveDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_stats

> models::GetGameStatsResponse get_game_stats(milestone_id)
Get Game Stats

Get play-by-play game statistics for a specific milestone. Supported sports: Pro Football, College Football, Pro Basketball, College Men's Basketball, College Women's Basketball, WNBA, Soccer, Pro Hockey, and Pro Baseball. Returns null for unsupported milestone types or milestones without a Sportradar ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **String** | Milestone ID | [required] |

### Return type

[**models::GetGameStatsResponse**](GetGameStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_data

> models::GetLiveDataResponse get_live_data(r#type, milestone_id, include_player_stats)
Get Live Data (with type)

Get live data for a specific milestone. This is the legacy endpoint that requires a type path parameter. Prefer using `/live_data/milestone/{milestone_id}` instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Type of live data | [required] |
**milestone_id** | **String** | Milestone ID | [required] |
**include_player_stats** | Option<**bool**> | When true, includes player-level statistics in the live data response. Supported for Pro Football, Pro Basketball, and College Men's Basketball milestones that have player ID mappings configured. Has no effect for other sports or milestones without player mappings. |  |[default to false]

### Return type

[**models::GetLiveDataResponse**](GetLiveDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_data_by_milestone

> models::GetLiveDataResponse get_live_data_by_milestone(milestone_id, include_player_stats)
Get Live Data

Get live data for a specific milestone. The response's `live_data.details.status` is updated far more often than the milestone's own `details.status` and is the preferred source for game status. It is not guaranteed to be current: if the provider feed falls behind, a cached board may still be served, with `live_data.details.live_data_delayed` set to `true` only once it is significantly stale (some feeds never set it), so a missing flag does not by itself mean the data is current.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **String** | Milestone ID | [required] |
**include_player_stats** | Option<**bool**> | When true, includes player-level statistics in the live data response. Supported for Pro Football, Pro Basketball, and College Men's Basketball milestones that have player ID mappings configured. Has no effect for other sports or milestones without player mappings. |  |[default to false]

### Return type

[**models::GetLiveDataResponse**](GetLiveDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_datas

> models::GetLiveDatasResponse get_live_datas(milestone_ids, include_player_stats)
Get Multiple Live Data

Get live data for multiple milestones. Each entry's `details.status` is updated far more often than the milestone's own `details.status` and is the preferred source for game status. It is not guaranteed to be current: if the provider feed falls behind, a cached board may still be served, with `details.live_data_delayed` set to `true` only once it is significantly stale (some feeds never set it), so a missing flag does not by itself mean the data is current.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_ids** | [**Vec<String>**](String.md) | Array of milestone IDs | [required] |
**include_player_stats** | Option<**bool**> | When true, includes player-level statistics in the live data response. Supported for Pro Football, Pro Basketball, and College Men's Basketball milestones that have player ID mappings configured. Has no effect for other sports or milestones without player mappings. |  |[default to false]

### Return type

[**models::GetLiveDatasResponse**](GetLiveDatasResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_weather_index

> models::GetWeatherIndexResponse get_weather_index(city, from, to, last_sec, detailed)
Get Weather Index

Get the Kalshi-computed city temperature index: the canonical minute-resolution series behind hourly temperature markets. City-keyed and independent of any event — the series exists whenever the city's index is configured. Values are Fahrenheit rounded to 0.01. Minutes where the index quorum failed carry no value and are never returned as points, so gaps in the series are real gaps. With `detailed=true` each point additionally carries every member station's reported reading and quality-control disposition — the pre-incorporation breakdown.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**city** | **String** | Index city ID (e.g. `miami`) | [required] |
**from** | Option<**i64**> | Window start, unix milliseconds (inclusive). Defaults to `to` minus 24 hours. Must be paired with `to` unless `last_sec` is used. |  |
**to** | Option<**i64**> | Window end, unix milliseconds (inclusive). Defaults to now. |  |
**last_sec** | Option<**i64**> | Trailing window in seconds; equivalent to `from=now-last_sec`, `to=now`. Mutually exclusive with `from`/`to`. |  |
**detailed** | Option<**bool**> | Include per-station audit readings on every point. |  |

### Return type

[**models::GetWeatherIndexResponse**](GetWeatherIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_weather_index_calibrations

> models::GetWeatherIndexCalibrationsResponse get_weather_index_calibrations(city)
Get Weather Index Calibrations

Get a city's published weather-index configuration timeline: the launch configuration plus every weekly offset calibration and methodology update, ascending by effective time. Each record carries the station weights, station offsets (Celsius), and the city reference used to compute index values from the record's effective time until the next record — everything needed to reproduce published index values for minutes computed under that configuration version. Offsets are re-estimated weekly after each complete UTC week; weights never change through calibration. The timeline is append-only and complete: it is never trimmed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**city** | **String** | Index city ID (e.g. `miami`) | [required] |

### Return type

[**models::GetWeatherIndexCalibrationsResponse**](GetWeatherIndexCalibrationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

