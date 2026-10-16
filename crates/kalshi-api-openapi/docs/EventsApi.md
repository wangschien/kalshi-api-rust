# \EventsApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event**](EventsApi.md#get_event) | **GET** /events/{event_ticker} | Get Event
[**get_event_fee_changes**](EventsApi.md#get_event_fee_changes) | **GET** /events/fee_changes | Get Event Fee Changes
[**get_event_forecast_percentiles_history**](EventsApi.md#get_event_forecast_percentiles_history) | **GET** /series/{series_ticker}/events/{ticker}/forecast_percentile_history | Get Event Forecast Percentile History
[**get_event_metadata**](EventsApi.md#get_event_metadata) | **GET** /events/{event_ticker}/metadata | Get Event Metadata
[**get_events**](EventsApi.md#get_events) | **GET** /events | Get Events
[**get_market_candlesticks_by_event**](EventsApi.md#get_market_candlesticks_by_event) | **GET** /series/{series_ticker}/events/{ticker}/candlesticks | Get Event Candlesticks
[**get_multivariate_events**](EventsApi.md#get_multivariate_events) | **GET** /events/multivariate | Get Multivariate Events



## get_event

> models::GetEventResponse get_event(event_ticker, exclude_markets, with_nested_markets)
Get Event

Endpoint for getting data about an event by its ticker. An event represents a real-world occurrence that can be traded on, such as an election, sports game, or economic indicator release. Events contain one or more markets where users can place trades on different outcomes. All events are accessible through this endpoint, even if their associated markets are older than the historical cutoff. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_ticker** | **String** | Event ticker | [required] |
**exclude_markets** | Option<**bool**> | Return event metadata without loading child markets. Use GET /markets with event_ticker, limit and cursor for bounded market pages. Cannot be combined with with_nested_markets=true. |  |[default to false]
**with_nested_markets** | Option<**bool**> | If true, markets are included within the event object. If false (default), markets are returned as a separate top-level field in the response. Historical markets settled before the historical cutoff will not be included. |  |[default to false]

### Return type

[**models::GetEventResponse**](GetEventResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_fee_changes

> models::GetEventFeeChangesResponse get_event_fee_changes(event_ticker, limit, cursor)
Get Event Fee Changes

Event fees are an override layered on top of the parent series' fee structure. If `fee_type_override` and `fee_multiplier_override` are null, that indicates the override is cleared. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_ticker** | Option<**String**> |  |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetEventFeeChangesResponse**](GetEventFeeChangesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_forecast_percentiles_history

> models::GetEventForecastPercentilesHistoryResponse get_event_forecast_percentiles_history(ticker, series_ticker, percentiles, start_ts, end_ts, period_interval)
Get Event Forecast Percentile History

Endpoint for getting the historical raw and formatted forecast numbers for an event at specific percentiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | The event ticker | [required] |
**series_ticker** | **String** | The series ticker | [required] |
**percentiles** | [**Vec<i32>**](i32.md) | Array of percentile values to retrieve (0-9999, max 10 values) | [required] |
**start_ts** | **i64** | Start timestamp for the range | [required] |
**end_ts** | **i64** | End timestamp for the range | [required] |
**period_interval** | **i32** | Specifies the length of each forecast period, in minutes. 0 for 5-second intervals, or 1, 60, or 1440 for minute-based intervals. | [required] |

### Return type

[**models::GetEventForecastPercentilesHistoryResponse**](GetEventForecastPercentilesHistoryResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_metadata

> models::GetEventMetadataResponse get_event_metadata(event_ticker)
Get Event Metadata

 Endpoint for getting metadata about an event by its ticker.  Returns only the metadata information for an event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_ticker** | **String** | Event ticker | [required] |

### Return type

[**models::GetEventMetadataResponse**](GetEventMetadataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events

> models::GetEventsResponse get_events(limit, cursor, with_nested_markets, with_milestones, status, series_ticker, tickers, min_close_ts, min_updated_ts)
Get Events

Get all events. This endpoint excludes multivariate events. To retrieve multivariate events, use the GET /events/multivariate endpoint. All events are accessible through this endpoint, even if their associated markets are older than the historical cutoff. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Parameter to specify the number of results per page. Defaults to 200. Maximum value is 200. |  |[default to 200]
**cursor** | Option<**String**> | Parameter to specify the pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**with_nested_markets** | Option<**bool**> | Parameter to specify if nested markets should be included in the response. When true, each event will include a 'markets' field containing a list of Market objects associated with that event. Historical markets settled before the historical cutoff will not be included. |  |[default to false]
**with_milestones** | Option<**bool**> | If true, includes related milestones as a field alongside events. |  |[default to false]
**status** | Option<**String**> | Filter by event status. Possible values are 'unopened', 'open', 'closed', 'settled'. Leave empty to return events with any status. |  |
**series_ticker** | Option<**String**> | Filter by series ticker |  |
**tickers** | Option<**String**> | Filter by specific event tickers. Comma-separated list of event tickers to retrieve. |  |
**min_close_ts** | Option<**i64**> | Filter events with at least one market with close timestamp greater than this Unix timestamp (in seconds). |  |
**min_updated_ts** | Option<**i64**> | Filter events with metadata updated after this Unix timestamp (in seconds). Use this to efficiently poll for changes. |  |

### Return type

[**models::GetEventsResponse**](GetEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_candlesticks_by_event

> models::GetEventCandlesticksResponse get_market_candlesticks_by_event(ticker, series_ticker, start_ts, end_ts, period_interval)
Get Event Candlesticks

 End-point for returning aggregated data across all markets corresponding to an event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | The event ticker | [required] |
**series_ticker** | **String** | The series ticker | [required] |
**start_ts** | **i64** | Start timestamp for the range | [required] |
**end_ts** | **i64** | End timestamp for the range | [required] |
**period_interval** | **i32** | Specifies the length of each candlestick period, in minutes. Must be one minute, one hour, or one day. | [required] |

### Return type

[**models::GetEventCandlesticksResponse**](GetEventCandlesticksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multivariate_events

> models::GetMultivariateEventsResponse get_multivariate_events(limit, cursor, series_ticker, collection_ticker, tickers, with_nested_markets)
Get Multivariate Events

Retrieve multivariate (combo) events. These are dynamically created events from multivariate event collections. Supports filtering by series ticker, collection ticker, or a comma-separated list of event tickers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 200. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. |  |
**series_ticker** | Option<**String**> | Filter by series ticker |  |
**collection_ticker** | Option<**String**> | Filter events by collection ticker. Returns only multivariate events belonging to the specified collection. Cannot be used together with series_ticker. |  |
**tickers** | Option<**String**> | Filter by specific event tickers. Comma-separated list of event tickers to retrieve. |  |
**with_nested_markets** | Option<**bool**> | Parameter to specify if nested markets should be included in the response. When true, each event will include a 'markets' field containing a list of Market objects associated with that event. |  |[default to false]

### Return type

[**models::GetMultivariateEventsResponse**](GetMultivariateEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

