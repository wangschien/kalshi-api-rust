# \HistoricalApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_fills_historical**](HistoricalApi.md#get_fills_historical) | **GET** /historical/fills | Get Historical Fills
[**get_historical_cutoff**](HistoricalApi.md#get_historical_cutoff) | **GET** /historical/cutoff | Get Historical Cutoff Timestamps
[**get_historical_market**](HistoricalApi.md#get_historical_market) | **GET** /historical/markets/{ticker} | Get Historical Market
[**get_historical_markets**](HistoricalApi.md#get_historical_markets) | **GET** /historical/markets | Get Historical Markets
[**get_historical_orders**](HistoricalApi.md#get_historical_orders) | **GET** /historical/orders | Get Historical Orders
[**get_historical_positions**](HistoricalApi.md#get_historical_positions) | **GET** /historical/positions | Get Historical Positions
[**get_market_candlesticks_historical**](HistoricalApi.md#get_market_candlesticks_historical) | **GET** /historical/markets/{ticker}/candlesticks | Get Historical Market Candlesticks
[**get_trades_historical**](HistoricalApi.md#get_trades_historical) | **GET** /historical/trades | Get Historical Trades



## get_fills_historical

> models::GetFillsResponse get_fills_historical(ticker, min_ts, max_ts, limit, cursor, subaccount)
Get Historical Fills

Endpoint for getting all historical fills for the member. A fill is when a trade you have is matched. API keys restricted to a subaccount return only that subaccount's fills. If supplied, `subaccount` must match the key's restriction. Registered partners may also use a user OAuth access token with the explicitly granted read::compliance_partner scope. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Filter by market ticker |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |

### Return type

[**models::GetFillsResponse**](GetFillsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp), [kalshiOauthAccessToken](../README.md#kalshiOauthAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_historical_cutoff

> models::GetHistoricalCutoffResponse get_historical_cutoff()
Get Historical Cutoff Timestamps

Returns the archive's cutoff timestamps. For positions, the cutoff is the backfill horizon; visibility in the historical API waits for a buffered, whole-event handoff.  ## Cutoff fields - `market_settled_ts` : Markets that **settled** before this timestamp, and their candlesticks, must be accessed via `GET /historical/markets` and `GET /historical/markets/{ticker}/candlesticks`. - `trades_created_ts` : Trades that were **filled** before this timestamp must be accessed via `GET /historical/fills`. - `orders_updated_ts` : Orders that were **canceled or fully executed** before this timestamp must be accessed via `GET /historical/orders`. Resting (active) orders are always available in `GET /portfolio/orders`. - `market_positions_last_updated_ts` : Backfill horizon for settled positions, not a guaranteed archive-visibility boundary. An event moves to `GET /historical/positions` only after its buffered handoff completes. Until then, its settled positions remain available via `GET /portfolio/positions?settlement_status=settled`. Unsettled positions remain available in `GET /portfolio/positions`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetHistoricalCutoffResponse**](GetHistoricalCutoffResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_historical_market

> models::GetMarketResponse get_historical_market(ticker)
Get Historical Market

 Endpoint for getting data about a specific market by its ticker from the historical database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Market ticker | [required] |

### Return type

[**models::GetMarketResponse**](GetMarketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_historical_markets

> models::GetMarketsResponse get_historical_markets(limit, cursor, tickers, event_ticker, series_ticker, mve_filter)
Get Historical Markets

Endpoint for getting markets that have been archived to the historical database. Filters are mutually exclusive. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 1000. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**tickers** | Option<**String**> | Filter by specific market tickers. Comma-separated list of market tickers to retrieve. |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**series_ticker** | Option<**String**> | Filter by series ticker |  |
**mve_filter** | Option<**String**> | Filter by multivariate events (combos). By default, MVE markets are included. |  |

### Return type

[**models::GetMarketsResponse**](GetMarketsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_historical_orders

> models::GetOrdersResponse get_historical_orders(ticker, min_ts, max_ts, limit, cursor, subaccount)
Get Historical Orders

Endpoint for getting orders that have been archived to the historical database. API keys restricted to a subaccount return only that subaccount's orders. If supplied, `subaccount` must match the key's restriction. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Filter by market ticker |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |

### Return type

[**models::GetOrdersResponse**](GetOrdersResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_historical_positions

> models::GetPositionsResponse get_historical_positions(ticker, event_ticker, subaccount, limit, cursor)
Get Historical Positions

Returns settled market positions after their event completes the live-to-historical handoff. Until then, settled positions remain available via `GET /portfolio/positions?settlement_status=settled`. The `market_positions_last_updated_ts` value on `GET /historical/cutoff` is the backfill horizon, not a guarantee that every older position has moved. Unsettled positions are available via `GET /portfolio/positions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Filter by market ticker |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetPositionsResponse**](GetPositionsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_candlesticks_historical

> models::GetMarketCandlesticksHistoricalResponse get_market_candlesticks_historical(ticker, start_ts, end_ts, period_interval)
Get Historical Market Candlesticks

 Endpoint for fetching historical candlestick data for markets that have been archived from the live data set. Time period length of each candlestick in minutes. Valid values: 1 (1 minute), 60 (1 hour), 1440 (1 day).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Market ticker - unique identifier for the specific market | [required] |
**start_ts** | **i64** | Start timestamp (Unix timestamp). Candlesticks will include those ending on or after this time. | [required] |
**end_ts** | **i64** | End timestamp (Unix timestamp). Candlesticks will include those ending on or before this time. | [required] |
**period_interval** | **i64** | Time period length of each candlestick in minutes. Valid values are 1 (1 minute), 60 (1 hour), or 1440 (1 day). | [required] |

### Return type

[**models::GetMarketCandlesticksHistoricalResponse**](GetMarketCandlesticksHistoricalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trades_historical

> models::GetTradesResponse get_trades_historical(ticker, min_ts, max_ts, limit, cursor, is_block_trade)
Get Historical Trades

 Endpoint for getting all historical trades for all markets. Trades that were filled before the historical cutoff are available via this endpoint. Block trades are included by default and identified by the `is_block_trade` field; use the `is_block_trade` query parameter to filter by block / non-block. See [Historical Data](https://docs.kalshi.com/getting_started/historical_data) for details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Filter by market ticker |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 1000. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**is_block_trade** | Option<**bool**> | Filter trades by whether they are block trades. Omit to return all trades. Set to `true` to return only block trades. Set to `false` to return only non-block trades.  |  |

### Return type

[**models::GetTradesResponse**](GetTradesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

