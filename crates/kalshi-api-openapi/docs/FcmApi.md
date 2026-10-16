# \FcmApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_fcm_position_transfer**](FcmApi.md#create_fcm_position_transfer) | **POST** /fcm/positions/transfers | Create FCM Position Transfer
[**create_fcm_subtrader**](FcmApi.md#create_fcm_subtrader) | **POST** /fcm/subtraders | Create FCM Subtrader
[**delete_fcm_event_contract_daily_cap**](FcmApi.md#delete_fcm_event_contract_daily_cap) | **DELETE** /fcm/subtraders/event_contract_daily_cap | Delete FCM Subtrader Event Contract Daily Cap
[**get_fcm_event_contract_daily_cap**](FcmApi.md#get_fcm_event_contract_daily_cap) | **GET** /fcm/subtraders/event_contract_daily_cap | Get FCM Subtrader Event Contract Daily Cap
[**get_fcm_fills**](FcmApi.md#get_fcm_fills) | **GET** /fcm/fills | Get FCM Fills
[**get_fcm_orders**](FcmApi.md#get_fcm_orders) | **GET** /fcm/orders | Get FCM Orders
[**get_fcm_positions**](FcmApi.md#get_fcm_positions) | **GET** /fcm/positions | Get FCM Positions
[**get_fcm_subtrader_blocked_categories**](FcmApi.md#get_fcm_subtrader_blocked_categories) | **GET** /fcm/subtraders/blocked_categories | Get FCM Subtrader Blocked Categories
[**list_fcm_subtraders**](FcmApi.md#list_fcm_subtraders) | **GET** /fcm/subtraders | List FCM Subtraders
[**update_fcm_event_contract_daily_cap**](FcmApi.md#update_fcm_event_contract_daily_cap) | **PUT** /fcm/subtraders/event_contract_daily_cap | Update FCM Subtrader Event Contract Daily Cap
[**update_fcm_subtrader_blocked_categories**](FcmApi.md#update_fcm_subtrader_blocked_categories) | **PUT** /fcm/subtraders/blocked_categories | Update FCM Subtrader Blocked Categories



## create_fcm_position_transfer

> models::CreateFcmPositionTransferResponse create_fcm_position_transfer(create_fcm_position_transfer_request)
Create FCM Position Transfer

Endpoint for FCM members to move a position in one market from one of their subtraders into a subtrader under their FCM's error account.  **This endpoint must be enabled for your FCM by Kalshi.** Kalshi enables it by configuring the error account that receives transferred positions and gives you that account's user id, which prefixes `target_subtrader_id`. Contact Kalshi to enable it; until then, requests return 400.  - The destination is always your FCM's error account. `target_subtrader_id` picks the subtrader   under it, which is created on first use. - Each request moves one market for one source subtrader. To move many positions, send one   request per subtrader and market. - The transfer is booked like a trade at `price_centicents`, which is always the YES price, even   when moving NO. No fee is charged and no fill is created. For YES, the error account pays the   price per contract; for NO, it pays $1 minus the price per contract. The FCM receives the same   amount. Use 0 for YES or 10000 for NO to move no cash. These amounts assume the source holds   the side being moved and the target subtrader holds no opposite position in that market;   otherwise the positions net like a trade and the balance changes differ. - The source's current position is not checked. A request for more than the source holds, or   with the opposite sign, flips or grows the source's position. - Requests are not idempotent. After a timeout or 5xx, check the position before retrying. - The market must be open, neither account may be blocked, and the error account must have   enough available balance on the market's exchange index to pay for the transfer. Otherwise   the request returns 409.  **Example: moving YES.** A customer subtrader holds 10 YES contracts. Send `position_centicount` 1000 and `price_centicents` 4500 (YES at 45 cents): - The customer subtrader goes from 10 YES to 0, and the error account subtrader from 0 to 10 YES. - The error account pays 10 x $0.45 = $4.50 and the FCM receives $4.50. - Sending `price_centicents` 0 instead moves the position with no cash.  **Example: moving NO.** A customer subtrader holds 5 NO contracts. Send `position_centicount` -500 and `price_centicents` 7000 (YES at 70 cents, so NO at 30 cents): - The customer subtrader goes from 5 NO to 0, and the error account subtrader from 0 to 5 NO. - The error account pays 5 x $0.30 = $1.50 and the FCM receives $1.50. - Sending `price_centicents` 10000 instead moves the position with no cash. Sending 0 makes the   error account pay the full $1 per contract ($5.00). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_fcm_position_transfer_request** | [**CreateFcmPositionTransferRequest**](CreateFcmPositionTransferRequest.md) |  | [required] |

### Return type

[**models::CreateFcmPositionTransferResponse**](CreateFCMPositionTransferResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_fcm_subtrader

> models::CreateFcmSubtraderResponse create_fcm_subtrader(create_fcm_subtrader_request)
Create FCM Subtrader

Endpoint for FCM members to create a subtrader on the event-contract exchange. The subtrader is registered on every exchange shard that knows the FCM; a shard provisioned later registers it lazily on the subtrader's first order there. Re-creating an existing subtrader returns 409. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_fcm_subtrader_request** | [**CreateFcmSubtraderRequest**](CreateFcmSubtraderRequest.md) |  | [required] |

### Return type

[**models::CreateFcmSubtraderResponse**](CreateFCMSubtraderResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_fcm_event_contract_daily_cap

> serde_json::Value delete_fcm_event_contract_daily_cap(subtrader_id)
Delete FCM Subtrader Event Contract Daily Cap

Removes the event-contract daily premium cap for an FCM member's subtrader. Removal closes the subtrader to new orders on cap-reservation sessions — every subtrader-bound API key — until a cap is set again. It does not stop orders your own unbound sessions submit on the subtrader's behalf; to stop the account entirely, block subtrader trading. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subtrader_id** | **String** | The subtrader whose daily cap should be removed. Must belong to the requesting FCM. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fcm_event_contract_daily_cap

> models::GetFcmEventContractDailyCapResponse get_fcm_event_contract_daily_cap(subtrader_id)
Get FCM Subtrader Event Contract Daily Cap

Returns the event-contract daily premium cap configured for an FCM member's subtrader, together with its live utilization. Executed utilization is the net premium deployed by fills today (sells and settlements credit back); resting and pending utilization reserve open and in-flight orders at their full worst-case cost plus fees. Executed utilization resets at midnight New York time on the returned cap date; resting and pending reservations persist for as long as their orders remain open, including across the reset. Returns 404 when the subtrader has no cap configured — in that state every order placed through the subtrader's bound API keys is rejected. API keys bound to a single FCM subtrader may also call this endpoint: `subtrader_id` may be omitted and defaults to the key's bound subtrader, and if supplied it must equal the bound subtrader or the request is rejected. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subtrader_id** | Option<**String**> | The subtrader whose daily cap should be returned. Must belong to the requesting FCM. Required unless the API key is bound to a subtrader, in which case it defaults to the bound subtrader when omitted and must equal it when supplied. |  |

### Return type

[**models::GetFcmEventContractDailyCapResponse**](GetFCMEventContractDailyCapResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fcm_fills

> models::GetFcmFillsResponse get_fcm_fills(min_ts, max_ts, cursor)
Get FCM Fills

Returns fills across the authenticated FCM's subtraders. Requires FCM member access. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetFcmFillsResponse**](GetFcmFillsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fcm_orders

> models::GetOrdersResponse get_fcm_orders(subtrader_id, client_order_ids, cursor, event_ticker, ticker, min_ts, max_ts, status, limit)
Get FCM Orders

Endpoint for FCM members to get orders for their subtraders. This endpoint requires FCM member access level. At least one of `subtrader_id` or `client_order_ids` is required; supplying both returns only the orders matching both filters. API keys bound to a single FCM subtrader may also call this endpoint: `subtrader_id` may be omitted and defaults to the key's bound subtrader, and if supplied it must equal the bound subtrader or the request is rejected. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subtrader_id** | Option<**String**> | Restricts the response to orders for a specific subtrader (FCM members only). Required unless client_order_ids is supplied. For an API key bound to a subtrader, defaults to the bound subtrader when omitted and must equal it when supplied. |  |
**client_order_ids** | Option<**String**> | Client order IDs to filter by, as a comma-separated list (maximum 100). Only orders created within the last 24 hours are searched, and a min_ts earlier than that is raised to 24 hours ago. Client order IDs are only unique within a subtrader among live and recent orders, so a single ID can match orders across subtraders or across time. Required unless subtrader_id is supplied. |  |
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**ticker** | Option<**String**> | Filter by market ticker |  |
**min_ts** | Option<**i64**> | Restricts the response to orders after a timestamp, formatted as a Unix Timestamp |  |
**max_ts** | Option<**i64**> | Restricts the response to orders before a timestamp, formatted as a Unix Timestamp |  |
**status** | Option<**String**> | Restricts the response to orders that have a certain status |  |
**limit** | Option<**i64**> | Parameter to specify the number of results per page. Defaults to 100 |  |

### Return type

[**models::GetOrdersResponse**](GetOrdersResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fcm_positions

> models::GetPositionsResponse get_fcm_positions(subtrader_id, ticker, event_ticker, count_filter, settlement_status, limit, cursor)
Get FCM Positions

Endpoint for FCM members to get market positions filtered by subtrader ID. This endpoint requires FCM member access level and allows filtering positions by subtrader ID. API keys bound to a single FCM subtrader may also call this endpoint: `subtrader_id` may be omitted and defaults to the key's bound subtrader, and if supplied it must equal the bound subtrader or the request is rejected. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subtrader_id** | Option<**String**> | Restricts the response to positions for a specific subtrader (FCM members only). Required unless the API key is bound to a subtrader, in which case it defaults to the bound subtrader when omitted and must equal it when supplied. |  |
**ticker** | Option<**String**> | Ticker of desired positions |  |
**event_ticker** | Option<**String**> | Event ticker of desired positions |  |
**count_filter** | Option<**String**> | Restricts the positions to those with any of following fields with non-zero values, as a comma separated list |  |
**settlement_status** | Option<**String**> | Settlement status of the markets to return. Defaults to unsettled |  |
**limit** | Option<**i64**> | Parameter to specify the number of results per page. Defaults to 100 |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination |  |

### Return type

[**models::GetPositionsResponse**](GetPositionsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fcm_subtrader_blocked_categories

> models::GetFcmSubtraderBlockedCategoriesResponse get_fcm_subtrader_blocked_categories(subtrader_id)
Get FCM Subtrader Blocked Categories

Returns the event categories an FCM member has blocked for one of its subtraders. The subtrader must belong to the requesting FCM. A subtrader with no blocks returns an empty list. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subtrader_id** | **String** | The subtrader whose blocked categories should be returned. Must belong to the requesting FCM. | [required] |

### Return type

[**models::GetFcmSubtraderBlockedCategoriesResponse**](GetFCMSubtraderBlockedCategoriesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_fcm_subtraders

> models::ListFcmSubtradersResponse list_fcm_subtraders()
List FCM Subtraders

Lists the authenticated FCM's event-contract subtraders, including accounts with no trades. Exchange metadata is asynchronous, so newly created accounts may not appear immediately. Trading block status includes firm-wide and Kalshi restrictions; fcm_trading_blocked identifies the FCM's own per-subtrader restriction. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListFcmSubtradersResponse**](ListFCMSubtradersResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_fcm_event_contract_daily_cap

> serde_json::Value update_fcm_event_contract_daily_cap(update_fcm_event_contract_daily_cap_request)
Update FCM Subtrader Event Contract Daily Cap

Sets the event-contract daily premium cap for an FCM member's subtrader. The cap bounds the subtrader's net premium at risk per trading day. It is enforced on cap-reservation sessions — which every subtrader-bound API key uses — and is mandatory there: a subtrader with no cap cannot trade through its bound keys. Cap changes take effect immediately; existing resting orders are never cancelled by a cap change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_fcm_event_contract_daily_cap_request** | [**UpdateFcmEventContractDailyCapRequest**](UpdateFcmEventContractDailyCapRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_fcm_subtrader_blocked_categories

> models::UpdateFcmSubtraderBlockedCategoriesResponse update_fcm_subtrader_blocked_categories(update_fcm_subtrader_blocked_categories_request)
Update FCM Subtrader Blocked Categories

Adds one event category to, or removes one from, the set an FCM member has blocked for one of its subtraders. The subtrader must belong to the requesting FCM. Blocks apply prospectively: new orders the subtrader places through its bound API credentials are rejected in markets whose event belongs to a blocked category, while orders already resting are not cancelled. Returns the subtrader's full resulting blocked set. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_fcm_subtrader_blocked_categories_request** | [**UpdateFcmSubtraderBlockedCategoriesRequest**](UpdateFcmSubtraderBlockedCategoriesRequest.md) |  | [required] |

### Return type

[**models::UpdateFcmSubtraderBlockedCategoriesResponse**](UpdateFCMSubtraderBlockedCategoriesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

