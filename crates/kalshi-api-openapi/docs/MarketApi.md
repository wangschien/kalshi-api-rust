# \MarketApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_get_market_candlesticks**](MarketApi.md#batch_get_market_candlesticks) | **GET** /markets/candlesticks | Batch Get Market Candlesticks
[**get_market**](MarketApi.md#get_market) | **GET** /markets/{ticker} | Get Market
[**get_market_candlesticks**](MarketApi.md#get_market_candlesticks) | **GET** /series/{series_ticker}/markets/{ticker}/candlesticks | Get Market Candlesticks
[**get_market_orderbook**](MarketApi.md#get_market_orderbook) | **GET** /markets/{ticker}/orderbook | Get Market Orderbook
[**get_market_orderbooks**](MarketApi.md#get_market_orderbooks) | **GET** /markets/orderbooks | Get Multiple Market Orderbooks
[**get_markets**](MarketApi.md#get_markets) | **GET** /markets | Get Markets
[**get_series**](MarketApi.md#get_series) | **GET** /series/{series_ticker} | Get Series
[**get_series_list**](MarketApi.md#get_series_list) | **GET** /series | Get Series List
[**get_trades**](MarketApi.md#get_trades) | **GET** /markets/trades | Get Trades



## batch_get_market_candlesticks

> models::BatchGetMarketCandlesticksResponse batch_get_market_candlesticks(market_tickers, start_ts, end_ts, period_interval, include_latest_before_start)
Batch Get Market Candlesticks

Endpoint for retrieving candlestick data for multiple markets.  - Accepts up to 100 market tickers per request - Returns up to 10,000 candlesticks total across all markets - Returns candlesticks grouped by market_id - Optionally includes a synthetic initial candlestick for price continuity (see `include_latest_before_start` parameter) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_tickers** | **String** | Comma-separated list of market tickers (maximum 100) | [required] |
**start_ts** | **i64** | Start timestamp in Unix seconds | [required] |
**end_ts** | **i64** | End timestamp in Unix seconds | [required] |
**period_interval** | **i32** | Candlestick period interval in minutes | [required] |
**include_latest_before_start** | Option<**bool**> | If true, prepends the latest candlestick available before the start_ts. This synthetic candlestick is created by: 1. Finding the most recent real candlestick before start_ts 2. Projecting it forward to the first period boundary (calculated as the next period interval after start_ts) 3. Setting all OHLC prices to null, and `previous_price` to the close price from the real candlestick  |  |[default to false]

### Return type

[**models::BatchGetMarketCandlesticksResponse**](BatchGetMarketCandlesticksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market

> models::GetMarketResponse get_market(ticker)
Get Market

 Endpoint for getting data about a specific market by its ticker. A market represents a specific binary outcome within an event that users can trade on (e.g., \"Will candidate X win?\"). Markets have yes/no positions, current prices, volume, and settlement rules.

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


## get_market_candlesticks

> models::GetMarketCandlesticksResponse get_market_candlesticks(series_ticker, ticker, start_ts, end_ts, period_interval, include_latest_before_start)
Get Market Candlesticks

Time period length of each candlestick in minutes. Valid values: 1 (1 minute), 60 (1 hour), 1440 (1 day). Candlesticks for markets that settled before the historical cutoff are only available via `GET /historical/markets/{ticker}/candlesticks`. See [Historical Data](https://docs.kalshi.com/getting_started/historical_data) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_ticker** | **String** | Series ticker - the series that contains the target market | [required] |
**ticker** | **String** | Market ticker - unique identifier for the specific market | [required] |
**start_ts** | **i64** | Start timestamp (Unix timestamp). Candlesticks will include those ending on or after this time. | [required] |
**end_ts** | **i64** | End timestamp (Unix timestamp). Candlesticks will include those ending on or before this time. | [required] |
**period_interval** | **i64** | Time period length of each candlestick in minutes. Valid values are 1 (1 minute), 60 (1 hour), or 1440 (1 day). | [required] |
**include_latest_before_start** | Option<**bool**> | If true, prepends the latest candlestick available before the start_ts. This synthetic candlestick is created by: 1. Finding the most recent real candlestick before start_ts 2. Projecting it forward to the first period boundary (calculated as the next period interval after start_ts) 3. Setting all OHLC prices to null, and `previous_price` to the close price from the real candlestick  |  |[default to false]

### Return type

[**models::GetMarketCandlesticksResponse**](GetMarketCandlesticksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_orderbook

> models::GetMarketOrderbookResponse get_market_orderbook(ticker, depth)
Get Market Orderbook

 Endpoint for getting the current order book for a specific market.  The order book shows all active bid orders for both yes and no sides of a binary market. It returns yes bids and no bids only (no asks are returned). This is because in binary markets, a bid for yes at price X is equivalent to an ask for no at price (100-X). For example, a yes bid at 7¢ is the same as a no ask at 93¢, with identical contract sizes.  Each side shows price levels with their corresponding quantities and order counts, organized from best to worst prices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Market ticker | [required] |
**depth** | Option<**i64**> | Depth of the orderbook to retrieve (0 or negative means all levels, 1-100 for specific depth) |  |[default to 0]

### Return type

[**models::GetMarketOrderbookResponse**](GetMarketOrderbookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_orderbooks

> models::GetMarketOrderbooksResponse get_market_orderbooks(tickers)
Get Multiple Market Orderbooks

Endpoint for getting the current order books for multiple markets in a single request. The order book shows all active bid orders for both yes and no sides of a binary market. It returns yes bids and no bids only (no asks are returned). This is because in binary markets, a bid for yes at price X is equivalent to an ask for no at price (100-X). For example, a yes bid at 7¢ is the same as a no ask at 93¢, with identical contract sizes. Each side shows price levels with their corresponding quantities and order counts, organized from best to worst prices. Returns one orderbook per requested market ticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickers** | [**Vec<String>**](String.md) | List of market tickers to fetch orderbooks for | [required] |

### Return type

[**models::GetMarketOrderbooksResponse**](GetMarketOrderbooksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_markets

> models::GetMarketsResponse get_markets(limit, cursor, event_ticker, series_ticker, min_created_ts, max_created_ts, min_updated_ts, max_updated_ts, max_close_ts, min_close_ts, min_settled_ts, max_settled_ts, status, tickers, mve_filter)
Get Markets

Filter by market status. Possible values: `unopened`, `open`, `closed`, `settled`. Leave empty to return markets with any status.  - Only one `status` filter may be supplied at a time.  - Timestamp filters will be mutually exclusive from other timestamp filters and certain status filters.   | Compatible Timestamp Filters | Additional Status Filters| Extra Notes |  |------------------------------|--------------------------|-------------|  | min_created_ts, max_created_ts | `unopened`, `open`, *empty* | |  | min_close_ts, max_close_ts | `closed`, *empty* | |  | min_settled_ts, max_settled_ts | `settled`, *empty* | |  | min_updated_ts, max_updated_ts | *empty* | Incompatible with all filters besides `mve_filter=exclude`. May be combined with `series_ticker`, which requires `mve_filter=exclude` |   Markets that settled before the historical cutoff are only available via `GET /historical/markets`. See [Historical Data](https://docs.kalshi.com/getting_started/historical_data) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 1000. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**series_ticker** | Option<**String**> | Filter by series ticker |  |
**min_created_ts** | Option<**i64**> | Filter items that created after this Unix timestamp |  |
**max_created_ts** | Option<**i64**> | Filter items that created before this Unix timestamp |  |
**min_updated_ts** | Option<**i64**> | Return markets with metadata updated later than this Unix timestamp (in seconds). Tracks non-trading changes only. May be combined with max_updated_ts and mve_filter=exclude. May also be combined with series_ticker, which requires mve_filter=exclude. Incompatible with other filters. |  |
**max_updated_ts** | Option<**i64**> | Return markets with metadata updated at or before this Unix timestamp (in seconds). Tracks non-trading changes only. May be combined with min_updated_ts and mve_filter=exclude. May also be combined with series_ticker, which requires mve_filter=exclude. Incompatible with other filters. |  |
**max_close_ts** | Option<**i64**> | Filter items that close before this Unix timestamp |  |
**min_close_ts** | Option<**i64**> | Filter items that close after this Unix timestamp |  |
**min_settled_ts** | Option<**i64**> | Filter items that settled after this Unix timestamp |  |
**max_settled_ts** | Option<**i64**> | Filter items that settled before this Unix timestamp |  |
**status** | Option<**String**> | Filter by market status. Leave empty to return markets with any status. |  |
**tickers** | Option<**String**> | Filter by specific market tickers. Comma-separated list of market tickers to retrieve. |  |
**mve_filter** | Option<**String**> | Filter by multivariate events (combos). 'only' returns only multivariate events, 'exclude' excludes multivariate events. |  |

### Return type

[**models::GetMarketsResponse**](GetMarketsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series

> models::GetSeriesResponse get_series(series_ticker, include_volume)
Get Series

 Endpoint for getting data about a specific series by its ticker.  A series represents a template for recurring events that follow the same format and rules (e.g., \"Monthly Jobs Report\", \"Weekly Initial Jobless Claims\", \"Daily Weather in NYC\"). Series define the structure, settlement sources, and metadata that will be applied to each recurring event instance within that series.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_ticker** | **String** | The ticker of the series to retrieve | [required] |
**include_volume** | Option<**bool**> | If true, includes the total volume traded across all events in this series. |  |[default to false]

### Return type

[**models::GetSeriesResponse**](GetSeriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_list

> models::GetSeriesListResponse get_series_list(category, tags, include_product_metadata, include_volume, min_updated_ts)
Get Series List

 Endpoint for getting data about multiple series with specified filters.  A series represents a template for recurring events that follow the same format and rules (e.g., \"Monthly Jobs Report\", \"Weekly Initial Jobless Claims\", \"Daily Weather in NYC\"). This endpoint allows you to browse and discover available series templates by category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> | Return series whose `categories` list contains this value. A series can have more than one discovery category, so the `category` field of a returned series (its primary category) may differ from the filter value. Matching is exact and case-sensitive. |  |
**tags** | Option<**String**> |  |  |
**include_product_metadata** | Option<**bool**> |  |  |[default to false]
**include_volume** | Option<**bool**> | If true, includes the total volume traded across all events in each series. |  |[default to false]
**min_updated_ts** | Option<**i64**> | Filter series with metadata updated after this Unix timestamp (in seconds). Use this to efficiently poll for changes. |  |

### Return type

[**models::GetSeriesListResponse**](GetSeriesListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trades

> models::GetTradesResponse get_trades(limit, cursor, ticker, min_ts, max_ts, is_block_trade)
Get Trades

Endpoint for getting all trades for all markets. A trade represents a completed transaction between two users on a specific market. Each trade includes the market ticker, price, quantity, and timestamp information. Block trades are included in the response by default and identified by the `is_block_trade` field; use the `is_block_trade` query parameter to filter by block / non-block. This endpoint returns a paginated response. Use the 'limit' parameter to control page size (1-1000, defaults to 100). The response includes a 'cursor' field - pass this value in the 'cursor' parameter of your next request to get the next page. An empty cursor indicates no more pages are available. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 1000. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**ticker** | Option<**String**> | Filter by market ticker |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**is_block_trade** | Option<**bool**> | Filter trades by whether they are block trades. Omit to return all trades. Set to `true` to return only block trades. Set to `false` to return only non-block trades.  |  |

### Return type

[**models::GetTradesResponse**](GetTradesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

