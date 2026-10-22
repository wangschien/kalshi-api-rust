# \MilestoneApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_milestone**](MilestoneApi.md#get_milestone) | **GET** /milestones/{milestone_id} | Get Milestone
[**get_milestones**](MilestoneApi.md#get_milestones) | **GET** /milestones | Get Milestones



## get_milestone

> models::GetMilestoneResponse get_milestone(milestone_id)
Get Milestone

Endpoint for getting data about a specific milestone by its ID. The milestone's `details.status`, when present, is maintained by a periodic background job. It lags the game's live state and may be stale or inaccurate, for example remaining `live` after the game ends. For game status, prefer live data, which is updated far more often: `live_data.details.status` from `GET /live_data/milestone/{milestone_id}`, or each entry's `details.status` from `GET /live_data/batch`. Scores are in the sport-specific fields of that live data `details` object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **String** | Milestone ID | [required] |

### Return type

[**models::GetMilestoneResponse**](GetMilestoneResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_milestones

> models::GetMilestonesResponse get_milestones(limit, minimum_start_date, category, competition, source_id, r#type, related_event_ticker, cursor, min_updated_ts)
Get Milestones

Endpoint for listing milestones. A milestone's `details.status`, when present, is maintained by a periodic background job. It lags the game's live state and may be stale or inaccurate, for example remaining `live` after the game ends. For game status, prefer live data, which is updated far more often: `live_data.details.status` from `GET /live_data/milestone/{milestone_id}`, or each entry's `details.status` from `GET /live_data/batch`. Scores are in the sport-specific fields of that live data `details` object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i64** | Number of milestones to return per page | [required] |
**minimum_start_date** | Option<**String**> | Minimum start date to filter milestones. Format RFC3339 timestamp |  |
**category** | Option<**String**> | Filter by milestone category. E.g. Sports, Elections, Esports, Crypto. |  |
**competition** | Option<**String**> | Filter by competition. E.g. Pro Football, Pro Basketball (M), Pro Baseball, Pro Hockey, College Football. |  |
**source_id** | Option<**String**> | Filter by source id |  |
**r#type** | Option<**String**> | Filter by milestone type. E.g. football_game, basketball_game, soccer_tournament_multi_leg, baseball_game, hockey_match, political_race. |  |
**related_event_ticker** | Option<**String**> | Filter by related event ticker |  |
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results |  |
**min_updated_ts** | Option<**i64**> | Filter milestones with metadata updated after this Unix timestamp (in seconds). Use this to efficiently poll for changes. |  |

### Return type

[**models::GetMilestonesResponse**](GetMilestonesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

