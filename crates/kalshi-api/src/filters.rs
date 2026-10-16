/// Optional filters for `GetOrders`.
///
/// [`Default`] leaves every field unset. Subaccount is a separate required argument, not a field here.
#[derive(Debug, Clone, Default)]
pub struct OrdersFilter {
    pub ticker: Option<String>,
    pub event_ticker: Option<String>,
    pub min_ts: Option<i64>,
    pub max_ts: Option<i64>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub exchange_index: Option<i64>,
}

/// Optional filters for `GetFills`.
///
/// [`Default`] leaves every field unset. Subaccount is a separate required argument, not a field here.
#[derive(Debug, Clone, Default)]
pub struct FillsFilter {
    pub ticker: Option<String>,
    pub order_id: Option<String>,
    pub min_ts: Option<i64>,
    pub max_ts: Option<i64>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub exchange_index: Option<i64>,
}

/// Optional filters for `GetPositions`.
///
/// [`Default`] leaves every field unset. Subaccount is a separate required argument, not a field here.
/// `limit` matches the generated query, which is `i32`.
#[derive(Debug, Clone, Default)]
pub struct PositionsFilter {
    pub settlement_status: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub count_filter: Option<String>,
    pub ticker: Option<String>,
    pub event_ticker: Option<String>,
    pub exchange_index: Option<i64>,
}

/// Optional filters for `GetOrderQueuePositions`.
///
/// [`Default`] leaves every field unset. Subaccount is a separate required argument, not a field here.
#[derive(Debug, Clone, Default)]
pub struct OrderQueuePositionsFilter {
    pub market_tickers: Option<String>,
    pub event_ticker: Option<String>,
}

/// Optional filters for `GetMarkets`. This call has no subaccount parameter.
#[derive(Debug, Clone, Default)]
pub struct MarketsFilter {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub event_ticker: Option<String>,
    pub series_ticker: Option<String>,
    pub min_created_ts: Option<i64>,
    pub max_created_ts: Option<i64>,
    pub min_updated_ts: Option<i64>,
    pub max_updated_ts: Option<i64>,
    pub max_close_ts: Option<i64>,
    pub min_close_ts: Option<i64>,
    pub min_settled_ts: Option<i64>,
    pub max_settled_ts: Option<i64>,
    pub status: Option<String>,
    pub tickers: Option<String>,
    pub mve_filter: Option<String>,
}
