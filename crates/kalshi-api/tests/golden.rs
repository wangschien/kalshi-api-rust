#![allow(clippy::disallowed_methods)]

use kalshi_api_openapi::models::{
    AmendOrderV2Response, BatchCancelOrdersV2Response, BatchCreateOrdersV2Response,
    CancelOrderV2Response, CreateOrderV2Response, DecreaseOrderV2Response, ExchangeStatus,
    GetBalanceResponse, GetExchangeScheduleResponse, GetFillsResponse, GetMarketResponse,
    GetMarketsResponse, GetOrderQueuePositionResponse, GetOrderQueuePositionsResponse,
    GetOrderResponse, GetOrdersResponse, GetPositionsResponse,
};
use kalshi_api_openapi::models::{BookSide, OrderStatus};

fn load<T: serde::de::DeserializeOwned>(name: &str) -> T {
    let path = format!("{}/tests/fixtures/s2b/{name}", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("{name}: {err}"))
}

fn error_code(error: &Option<Option<kalshi_api_openapi::models::ErrorResponse>>) -> Option<&str> {
    error
        .as_ref()
        .and_then(|inner| inner.as_ref())
        .and_then(|body| body.code.as_deref())
}

#[test]
fn synthetic_samples_match_generated_responses() {
    let _ = load::<GetOrderResponse>("../get_order.json");
    let _ = load::<CreateOrderV2Response>("create_order_v2.json");
    let _ = load::<AmendOrderV2Response>("amend_order_v2.json");
    let _ = load::<DecreaseOrderV2Response>("decrease_order_v2.json");
    let _ = load::<CancelOrderV2Response>("cancel_order_v2.json");
    let created = load::<BatchCreateOrdersV2Response>("batch_create_orders_v2.json");
    let cancelled = load::<BatchCancelOrdersV2Response>("batch_cancel_orders_v2.json");
    let orders = load::<GetOrdersResponse>("get_orders.json");
    let fills = load::<GetFillsResponse>("get_fills.json");
    let positions = load::<GetPositionsResponse>("get_positions.json");
    let _ = load::<GetBalanceResponse>("get_balance.json");
    let _ = load::<GetMarketResponse>("get_market.json");
    let markets = load::<GetMarketsResponse>("get_markets.json");
    let _ = load::<GetOrderQueuePositionResponse>("get_order_queue_position.json");
    let queue = load::<GetOrderQueuePositionsResponse>("get_order_queue_positions.json");
    let _ = load::<ExchangeStatus>("get_exchange_status.json");
    let schedule = load::<GetExchangeScheduleResponse>("get_exchange_schedule.json");

    assert_eq!(orders.orders.len(), 1, "order list item must be present");
    assert_eq!(orders.orders[0].status, OrderStatus::Resting);
    assert_eq!(orders.orders[0].book_side, BookSide::Bid);
    assert_eq!(fills.fills.len(), 1, "fill must be present");
    assert!(fills.fills[0].side.is_some());
    assert!(fills.fills[0].action.is_some());
    assert_eq!(positions.market_positions.len(), 1);
    assert_eq!(positions.event_positions.len(), 1);
    assert_eq!(queue.queue_positions.len(), 1);
    assert!(!queue.queue_positions[0].queue_position_fp.is_empty());
    assert_eq!(markets.markets.len(), 1);
    assert_eq!(schedule.schedule.standard_hours.len(), 1);
    assert_eq!(schedule.schedule.maintenance_windows.len(), 1);
    assert!(
        created
            .orders
            .iter()
            .any(|item| error_code(&item.error) == Some("synthetic")),
        "batch create must include a per-item error"
    );
    assert!(created.orders.iter().any(|item| {
        item.order_id.is_some() && item.fill_count.is_some() && error_code(&item.error).is_none()
    }));
    assert!(
        cancelled
            .orders
            .iter()
            .any(|item| error_code(&item.error) == Some("synthetic")),
        "batch cancel must include a per-item error"
    );
    assert!(cancelled.orders.iter().any(|item| {
        !item.reduced_by.is_empty() && item.ts_ms.is_some() && error_code(&item.error).is_none()
    }));
}
