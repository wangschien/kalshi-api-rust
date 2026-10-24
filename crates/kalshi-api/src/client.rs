//! Hand-written calls for the trading client.
//!
//! `CancelAllOrders` is intentionally not wrapped. Its query parameter says:
//! "Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts."
//! The operation text is: "Cancels all resting event-market orders for the authenticated Direct member across every exchange shard. If `subaccount` is omitted, matching orders may come from any subaccount. If it is provided, only orders for that subaccount are eligible. Newly placed orders may also be cancelled during the minute after the request."
//! The generated `cancel_all_orders` function is the raw call. [`Client`] does not invoke it.
//!
//! The official Python SDK leaves `subaccount` optional. Typed methods here require
//! [`crate::SubaccountFilter`], [`crate::Subaccount`], or [`crate::SubaccountChoice`].
//! Raw methods keep the spec's `Option` and omit the field when that option is `None`.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use kalshi_api_openapi::apis::account_api::{self};
use kalshi_api_openapi::apis::exchange_api::{self};
use kalshi_api_openapi::apis::historical_api::{self};
use kalshi_api_openapi::apis::market_api::{
    self, GetMarketOrderbookParams, GetMarketParams, GetMarketsParams,
};
use kalshi_api_openapi::apis::orders_api::{
    self, AmendOrderV2Params, BatchCancelOrdersV2Params, BatchCreateOrdersV2Params,
    CancelOrderV2Params, CreateOrderV2Params, DecreaseOrderV2Params, GetOrderParams,
    GetOrderQueuePositionParams, GetOrderQueuePositionsParams, GetOrdersParams,
};
use kalshi_api_openapi::apis::portfolio_api::{
    self, GetBalanceParams, GetFillsParams, GetPositionsParams,
};
use kalshi_api_openapi::apis::{configuration::Configuration, Error};
use kalshi_api_openapi::models::{
    AmendOrderV2Response, BatchCancelOrdersV2Request, BatchCancelOrdersV2Response,
    BatchCreateOrdersV2Request, BatchCreateOrdersV2Response, CancelOrderV2Response,
    CreateOrderV2Response, DecreaseOrderV2Response, ExchangeStatus, GetAccountApiLimitsResponse,
    GetAccountEndpointCostsResponse, GetBalanceResponse, GetExchangeScheduleResponse,
    GetFillsResponse, GetHistoricalCutoffResponse, GetMarketOrderbookResponse, GetMarketResponse,
    GetMarketsResponse, GetOrderQueuePositionResponse, GetOrderQueuePositionsResponse,
    GetOrderResponse, GetOrdersResponse, GetPositionsResponse, GetUserDataTimestampResponse,
};
use reqwest::StatusCode;
use reqwest_middleware::ClientBuilder;
use serde_json::Value;

use crate::amend::AmendOrderV2;
use crate::batch::BatchCancelItem;
use crate::capture::{scope, CaptureMiddleware, CaptureSlot};
use crate::decrease::DecreaseOrderV2;
use crate::error::{BodyErrorKind, TransportKind, VenueError};
use crate::filters::{
    FillsFilter, MarketsFilter, OrderQueuePositionsFilter, OrdersFilter, PositionsFilter,
};
use crate::order::{CreateOrderV2, ExchangeIndexChoice};
use crate::sign::{Clock, SignMiddleware, SystemClock};
use crate::subaccount::{Subaccount, SubaccountError, SubaccountFilter};

/// Official servers from the pinned OpenAPI document. Callers pass one explicitly.
///
/// `Prod` is `https://external-api.kalshi.com/trade-api/v2`, the first production
/// server in the spec and the default host in the official Python async SDK.
/// `ProdShared` is `https://api.elections.kalshi.com/trade-api/v2`, the other
/// production server listed in the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Prod,
    ProdShared,
    Demo,
    DemoShared,
}

impl Environment {
    /// Base URL for this environment, with no trailing slash beyond the version prefix.
    pub fn base_url(self) -> &'static str {
        match self {
            Self::Prod => "https://external-api.kalshi.com/trade-api/v2",
            Self::ProdShared => "https://api.elections.kalshi.com/trade-api/v2",
            Self::Demo => "https://external-api.demo.kalshi.co/trade-api/v2",
            Self::DemoShared => "https://demo-api.kalshi.co/trade-api/v2",
        }
    }
}

/// Every field is required. This struct has no default.
///
/// `connect_timeout`, `request_timeout`, and `max_body_bytes` are chosen by
/// the caller. `None` sets no local deadline or no body limit; the SDK has no
/// default number for any of them.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub key_id: String,
    pub private_key_pem: String,
    pub connect_timeout: Option<std::time::Duration>,
    pub request_timeout: Option<std::time::Duration>,
    pub max_body_bytes: Option<usize>,
    pub max_capture_bytes: usize,
}

pub struct Client {
    configuration: Configuration,
    pass_through: Arc<AtomicU64>,
    schema_drift: Arc<AtomicU64>,
}

impl Client {
    /// Builds a client that signs with the system clock.
    pub fn new(config: ClientConfig) -> Result<Self, anyhow::Error> {
        Self::with_clock(config, Arc::new(SystemClock))
    }

    /// Builds a client that signs with `clock`.
    pub fn with_clock(config: ClientConfig, clock: Arc<dyn Clock>) -> Result<Self, anyhow::Error> {
        let pass_through = Arc::new(AtomicU64::new(0));
        let capture = CaptureMiddleware::new(
            config.max_body_bytes,
            config.max_capture_bytes,
            Arc::clone(&pass_through),
        )
        .map_err(anyhow::Error::msg)?;
        let sign = SignMiddleware::new(config.key_id, &config.private_key_pem, clock)?;
        let mut http = reqwest::Client::builder().use_rustls_tls();
        if let Some(timeout) = config.connect_timeout {
            http = http.connect_timeout(timeout);
        }
        if let Some(timeout) = config.request_timeout {
            http = http.timeout(timeout);
        }
        let http = http.build()?;
        // Capture is added first, so it is outside signing. Signing is innermost.
        let client = ClientBuilder::new(http).with(capture).with(sign).build();
        let configuration = Configuration {
            connect_timeout: config.connect_timeout,
            request_timeout: config.request_timeout,
            base_path: config.base_url,
            user_agent: Some("kalshi-api/3.34.0".to_owned()),
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };
        Ok(Self {
            configuration,
            pass_through,
            schema_drift: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Requests that did not open a capture slot.
    pub fn pass_through_count(&self) -> u64 {
        self.pass_through.load(Ordering::Relaxed)
    }

    /// Responses classified as schema drift.
    pub fn schema_drift_count(&self) -> u64 {
        self.schema_drift.load(Ordering::Relaxed)
    }

    /// The generated configuration's API key slot stays empty. Signing is the middleware.
    pub fn api_key_disabled(&self) -> bool {
        self.configuration.api_key.is_none()
    }

    /// Get Order.
    pub async fn get_order(&self, order_id: &str) -> Result<GetOrderResponse, VenueError> {
        let order_id = order_id.to_owned();
        let configuration = &self.configuration;
        self.invoke("get_order", || {
            orders_api::get_order(
                configuration,
                GetOrderParams {
                    order_id: order_id.clone(),
                },
            )
        })
        .await
    }

    /// Get Order.
    pub async fn get_order_raw(&self, order_id: &str) -> Result<(StatusCode, Value), VenueError> {
        let order_id = order_id.to_owned();
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_order_raw",
            orders_api::get_order(configuration, GetOrderParams { order_id }),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts.
    ///
    /// Filter results by exchange shard. Omit to return results from all exchange shards.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `SubaccountFilter::All` omits the query. `SubaccountFilter::Only` sends that subaccount, including `0`.
    pub async fn get_orders(
        &self,
        filter: &OrdersFilter,
        subaccount: SubaccountFilter,
    ) -> Result<GetOrdersResponse, VenueError> {
        let params = orders_params(filter, filter_query(subaccount)?)?;
        let configuration = &self.configuration;
        self.invoke("get_orders", || {
            orders_api::get_orders(configuration, params)
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts.
    ///
    /// Filter results by exchange shard. Omit to return results from all exchange shards.
    ///
    /// `None` omits the query, and the spec then returns every subaccount.
    pub async fn get_orders_raw(
        &self,
        filter: &OrdersFilter,
        subaccount: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let params = orders_params(filter, subaccount)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_orders_raw",
            orders_api::get_orders(configuration, params),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts.
    ///
    /// Filter results by exchange shard. Omit to return results from all exchange shards.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `SubaccountFilter::All` omits the query. `SubaccountFilter::Only` sends that subaccount, including `0`.
    pub async fn get_fills(
        &self,
        filter: &FillsFilter,
        subaccount: SubaccountFilter,
    ) -> Result<GetFillsResponse, VenueError> {
        let params = fills_params(filter, filter_query(subaccount)?)?;
        let configuration = &self.configuration;
        self.invoke("get_fills", || {
            portfolio_api::get_fills(configuration, params)
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts.
    ///
    /// Filter results by exchange shard. Omit to return results from all exchange shards.
    ///
    /// `None` omits the query, and the spec then returns every subaccount.
    pub async fn get_fills_raw(
        &self,
        filter: &FillsFilter,
        subaccount: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let params = fills_params(filter, subaccount)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_fills_raw",
            portfolio_api::get_fills(configuration, params),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Filter results by exchange shard. Omit to return results from all exchange shards.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `Subaccount::Primary` sends `0`.
    pub async fn get_positions(
        &self,
        filter: &PositionsFilter,
        subaccount: Subaccount,
    ) -> Result<GetPositionsResponse, VenueError> {
        let params = positions_params(filter, Some(primary_query(subaccount)?))?;
        let configuration = &self.configuration;
        self.invoke("get_positions", || {
            portfolio_api::get_positions(configuration, params)
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Filter results by exchange shard. Omit to return results from all exchange shards.
    ///
    /// `None` omits the query, and the spec then uses the primary account.
    pub async fn get_positions_raw(
        &self,
        filter: &PositionsFilter,
        subaccount: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let params = positions_params(filter, subaccount)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_positions_raw",
            portfolio_api::get_positions(configuration, params),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Get Market.
    pub async fn get_market(&self, ticker: &str) -> Result<GetMarketResponse, VenueError> {
        let ticker = ticker.to_owned();
        let configuration = &self.configuration;
        self.invoke("get_market", || {
            market_api::get_market(
                configuration,
                GetMarketParams {
                    ticker: ticker.clone(),
                },
            )
        })
        .await
    }

    /// Get Market.
    pub async fn get_market_raw(&self, ticker: &str) -> Result<(StatusCode, Value), VenueError> {
        let ticker = ticker.to_owned();
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_market_raw",
            market_api::get_market(configuration, GetMarketParams { ticker }),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `Subaccount::Primary` sends `0`. The body `count` comes from [`AmendOrderV2`].
    pub async fn amend_order_v2(
        &self,
        order_id: &str,
        amend: &AmendOrderV2,
        subaccount: Subaccount,
    ) -> Result<AmendOrderV2Response, VenueError> {
        let order_id = order_id.to_owned();
        let body = amend.to_generated().map_err(show_err)?;
        let subaccount = Some(primary_query(subaccount)?);
        let configuration = &self.configuration;
        self.invoke("amend_order_v2", || {
            orders_api::amend_order_v2(
                configuration,
                AmendOrderV2Params {
                    order_id: order_id.clone(),
                    amend_order_v2_request: body.clone(),
                    subaccount,
                },
            )
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// `None` omits the query, and the spec then uses the primary account.
    pub async fn amend_order_v2_raw(
        &self,
        order_id: &str,
        amend: &AmendOrderV2,
        subaccount: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let body = amend.to_generated().map_err(show_err)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "amend_order_v2_raw",
            orders_api::amend_order_v2(
                configuration,
                AmendOrderV2Params {
                    order_id: order_id.to_owned(),
                    amend_order_v2_request: body,
                    subaccount,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by market ticker.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `Subaccount::Primary` sends `0`. [`crate::Decrease`] sets exactly one of `reduce_by` or `reduce_to`.
    pub async fn decrease_order_v2(
        &self,
        order_id: &str,
        decrease: &DecreaseOrderV2,
        subaccount: Subaccount,
    ) -> Result<DecreaseOrderV2Response, VenueError> {
        let order_id = order_id.to_owned();
        let body = decrease.to_generated();
        let subaccount = Some(primary_query(subaccount)?);
        let configuration = &self.configuration;
        self.invoke("decrease_order_v2", || {
            orders_api::decrease_order_v2(
                configuration,
                DecreaseOrderV2Params {
                    order_id: order_id.clone(),
                    decrease_order_v2_request: body.clone(),
                    subaccount,
                },
            )
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by market ticker.
    ///
    /// `None` omits the query, and the spec then uses the primary account.
    pub async fn decrease_order_v2_raw(
        &self,
        order_id: &str,
        decrease: &DecreaseOrderV2,
        subaccount: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let body = decrease.to_generated();
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "decrease_order_v2_raw",
            orders_api::decrease_order_v2(
                configuration,
                DecreaseOrderV2Params {
                    order_id: order_id.to_owned(),
                    decrease_order_v2_request: body,
                    subaccount,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker parameter is required for auto-routing.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `Subaccount::Primary` sends `0`.
    pub async fn cancel_order_v2(
        &self,
        order_id: &str,
        subaccount: Subaccount,
        exchange_index: ExchangeIndexChoice,
        market_ticker: Option<String>,
    ) -> Result<CancelOrderV2Response, VenueError> {
        let order_id = order_id.to_owned();
        let subaccount = Some(primary_query(subaccount)?);
        let exchange_index = exchange_index.as_option();
        let configuration = &self.configuration;
        self.invoke("cancel_order_v2", || {
            orders_api::cancel_order_v2(
                configuration,
                CancelOrderV2Params {
                    order_id: order_id.clone(),
                    subaccount,
                    exchange_index,
                    market_ticker: market_ticker.clone(),
                },
            )
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker parameter is required for auto-routing.
    ///
    /// `None` omits the query, and the spec then uses the primary account.
    pub async fn cancel_order_v2_raw(
        &self,
        order_id: &str,
        subaccount: Option<i64>,
        exchange_index: Option<i64>,
        market_ticker: Option<String>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "cancel_order_v2_raw",
            orders_api::cancel_order_v2(
                configuration,
                CancelOrderV2Params {
                    order_id: order_id.to_owned(),
                    subaccount,
                    exchange_index,
                    market_ticker,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// The subaccount number to use for this order. 0 is the primary subaccount. Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// Each item is a [`CreateOrderV2`]. `SubaccountChoice::KeyRestricted` omits that order's field.
    pub async fn batch_create_orders_v2(
        &self,
        orders: &[CreateOrderV2],
    ) -> Result<BatchCreateOrdersV2Response, VenueError> {
        let body = batch_create_body(orders)?;
        let configuration = &self.configuration;
        self.invoke("batch_create_orders_v2", || {
            orders_api::batch_create_orders_v2(
                configuration,
                BatchCreateOrdersV2Params {
                    batch_create_orders_v2_request: body.clone(),
                },
            )
        })
        .await
    }

    /// The subaccount number to use for this order. 0 is the primary subaccount. Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// `SubaccountChoice::KeyRestricted` omits that order's field, which is the spec's absent value.
    pub async fn batch_create_orders_v2_raw(
        &self,
        orders: &[CreateOrderV2],
    ) -> Result<(StatusCode, Value), VenueError> {
        let body = batch_create_body(orders)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "batch_create_orders_v2_raw",
            orders_api::batch_create_orders_v2(
                configuration,
                BatchCreateOrdersV2Params {
                    batch_create_orders_v2_request: body,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Optional subaccount number to use for this cancellation (0 for primary, 1-63 for subaccounts). Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker field is required for auto-routing.
    ///
    /// Each item is a [`BatchCancelItem`].
    pub async fn batch_cancel_orders_v2(
        &self,
        orders: &[BatchCancelItem],
    ) -> Result<BatchCancelOrdersV2Response, VenueError> {
        let body = batch_cancel_body(orders)?;
        let configuration = &self.configuration;
        self.invoke("batch_cancel_orders_v2", || {
            orders_api::batch_cancel_orders_v2(
                configuration,
                BatchCancelOrdersV2Params {
                    batch_cancel_orders_v2_request: body.clone(),
                },
            )
        })
        .await
    }

    /// Optional subaccount number to use for this cancellation (0 for primary, 1-63 for subaccounts). Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker field is required for auto-routing.
    ///
    /// `SubaccountChoice::KeyRestricted` omits that order's field, which is the spec's absent value.
    pub async fn batch_cancel_orders_v2_raw(
        &self,
        orders: &[BatchCancelItem],
    ) -> Result<(StatusCode, Value), VenueError> {
        let body = batch_cancel_body(orders)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "batch_cancel_orders_v2_raw",
            orders_api::batch_cancel_orders_v2(
                configuration,
                BatchCancelOrdersV2Params {
                    batch_cancel_orders_v2_request: body,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Get Order Queue Position.
    pub async fn get_order_queue_position(
        &self,
        order_id: &str,
    ) -> Result<GetOrderQueuePositionResponse, VenueError> {
        let order_id = order_id.to_owned();
        let configuration = &self.configuration;
        self.invoke("get_order_queue_position", || {
            orders_api::get_order_queue_position(
                configuration,
                GetOrderQueuePositionParams {
                    order_id: order_id.clone(),
                },
            )
        })
        .await
    }

    /// Get Order Queue Position.
    pub async fn get_order_queue_position_raw(
        &self,
        order_id: &str,
    ) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_order_queue_position_raw",
            orders_api::get_order_queue_position(
                configuration,
                GetOrderQueuePositionParams {
                    order_id: order_id.to_owned(),
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `Subaccount::Primary` sends `0`.
    pub async fn get_order_queue_positions(
        &self,
        filter: &OrderQueuePositionsFilter,
        subaccount: Subaccount,
    ) -> Result<GetOrderQueuePositionsResponse, VenueError> {
        let params = queue_params(filter, Some(primary_query(subaccount)?));
        let configuration = &self.configuration;
        self.invoke("get_order_queue_positions", || {
            orders_api::get_order_queue_positions(configuration, params)
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// `None` omits the query, and the spec then uses the primary account.
    pub async fn get_order_queue_positions_raw(
        &self,
        filter: &OrderQueuePositionsFilter,
        subaccount: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let params = queue_params(filter, subaccount);
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_order_queue_positions_raw",
            orders_api::get_order_queue_positions(configuration, params),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange index used to scope the balance and portfolio value. If omitted, both include all exchange indexes.
    ///
    /// The official Python SDK leaves subaccount optional. This argument is required.
    /// `Subaccount::Primary` sends `0`.
    pub async fn get_balance(
        &self,
        subaccount: Subaccount,
        exchange_index: Option<i64>,
    ) -> Result<GetBalanceResponse, VenueError> {
        let subaccount = Some(primary_query(subaccount)?);
        let configuration = &self.configuration;
        self.invoke("get_balance", || {
            portfolio_api::get_balance(
                configuration,
                GetBalanceParams {
                    subaccount,
                    exchange_index,
                },
            )
        })
        .await
    }

    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount.
    ///
    /// Exchange index used to scope the balance and portfolio value. If omitted, both include all exchange indexes.
    ///
    /// `None` omits the query, and the spec then uses the primary account.
    pub async fn get_balance_raw(
        &self,
        subaccount: Option<i64>,
        exchange_index: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_balance_raw",
            portfolio_api::get_balance(
                configuration,
                GetBalanceParams {
                    subaccount,
                    exchange_index,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Get Markets.
    pub async fn get_markets(
        &self,
        filter: &MarketsFilter,
    ) -> Result<GetMarketsResponse, VenueError> {
        let params = markets_params(filter);
        let configuration = &self.configuration;
        self.invoke("get_markets", || {
            market_api::get_markets(configuration, params)
        })
        .await
    }

    /// Get Markets.
    pub async fn get_markets_raw(
        &self,
        filter: &MarketsFilter,
    ) -> Result<(StatusCode, Value), VenueError> {
        let params = markets_params(filter);
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_markets_raw",
            market_api::get_markets(configuration, params),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Get Exchange Status.
    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, VenueError> {
        let configuration = &self.configuration;
        self.invoke("get_exchange_status", || {
            exchange_api::get_exchange_status(configuration)
        })
        .await
    }

    /// Get Exchange Status.
    pub async fn get_exchange_status_raw(&self) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_exchange_status_raw",
            exchange_api::get_exchange_status(configuration),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Get Exchange Schedule.
    pub async fn get_exchange_schedule(&self) -> Result<GetExchangeScheduleResponse, VenueError> {
        let configuration = &self.configuration;
        self.invoke("get_exchange_schedule", || {
            exchange_api::get_exchange_schedule(configuration)
        })
        .await
    }

    /// Get Exchange Schedule.
    pub async fn get_exchange_schedule_raw(&self) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_exchange_schedule_raw",
            exchange_api::get_exchange_schedule(configuration),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Approximate time through which user data endpoints were last validated.
    pub async fn get_user_data_timestamp(
        &self,
    ) -> Result<GetUserDataTimestampResponse, VenueError> {
        let configuration = &self.configuration;
        self.invoke("get_user_data_timestamp", || {
            exchange_api::get_user_data_timestamp(configuration)
        })
        .await
    }

    /// Approximate time through which user data endpoints were last validated.
    pub async fn get_user_data_timestamp_raw(&self) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_user_data_timestamp_raw",
            exchange_api::get_user_data_timestamp(configuration),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Cutoff between live and historical trades, orders, markets, and positions.
    pub async fn get_historical_cutoff(&self) -> Result<GetHistoricalCutoffResponse, VenueError> {
        let configuration = &self.configuration;
        self.invoke("get_historical_cutoff", || {
            historical_api::get_historical_cutoff(configuration)
        })
        .await
    }

    /// Cutoff between live and historical trades, orders, markets, and positions.
    pub async fn get_historical_cutoff_raw(&self) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_historical_cutoff_raw",
            historical_api::get_historical_cutoff(configuration),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Authenticated API tier and token-bucket limits.
    pub async fn get_account_api_limits(&self) -> Result<GetAccountApiLimitsResponse, VenueError> {
        let configuration = &self.configuration;
        self.invoke("get_account_api_limits", || {
            account_api::get_account_api_limits(configuration)
        })
        .await
    }

    /// Authenticated API tier and token-bucket limits.
    pub async fn get_account_api_limits_raw(&self) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_account_api_limits_raw",
            account_api::get_account_api_limits(configuration),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Endpoints whose token cost differs from the default.
    pub async fn get_account_endpoint_costs(
        &self,
    ) -> Result<GetAccountEndpointCostsResponse, VenueError> {
        let configuration = &self.configuration;
        self.invoke("get_account_endpoint_costs", || {
            account_api::get_account_endpoint_costs(configuration)
        })
        .await
    }

    /// Endpoints whose token cost differs from the default.
    pub async fn get_account_endpoint_costs_raw(&self) -> Result<(StatusCode, Value), VenueError> {
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_account_endpoint_costs_raw",
            account_api::get_account_endpoint_costs(configuration),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Current bid book for one market. `depth` is sent only when `Some`; `Some(100)` asks for 100 levels.
    pub async fn get_market_orderbook(
        &self,
        ticker: &str,
        depth: Option<i64>,
    ) -> Result<GetMarketOrderbookResponse, VenueError> {
        let ticker = ticker.to_owned();
        let configuration = &self.configuration;
        self.invoke("get_market_orderbook", || {
            market_api::get_market_orderbook(
                configuration,
                GetMarketOrderbookParams {
                    ticker: ticker.clone(),
                    depth,
                },
            )
        })
        .await
    }

    /// Current bid book for one market. `depth` is sent only when `Some`.
    pub async fn get_market_orderbook_raw(
        &self,
        ticker: &str,
        depth: Option<i64>,
    ) -> Result<(StatusCode, Value), VenueError> {
        let ticker = ticker.to_owned();
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "get_market_orderbook_raw",
            market_api::get_market_orderbook(
                configuration,
                GetMarketOrderbookParams { ticker, depth },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// Signed GET on this client's origin.
    ///
    /// `path` is `/exchange/user_data_timestamp` or the same path already prefixed with
    /// `/trade-api/v2`. An origin such as `https://external-api.kalshi.com` and a base that
    /// already ends in `/trade-api/v2` both request `/trade-api/v2/...`. The signature covers
    /// that path. A query or fragment in `path` is rejected before the request is sent.
    pub async fn get_signed_raw(&self, path: &str) -> Result<(StatusCode, Value), VenueError> {
        let url = signed_get_url(&self.configuration.base_path, path)?;
        let client = &self.configuration.client;
        let (sent, slot) = scope("get_signed_raw", client.get(&url).send()).await;
        if slot.status.is_some() || slot.too_large || slot.body_error.is_some() {
            return finish_captured(&slot);
        }
        match sent {
            Ok(_) => finish_captured(&slot),
            Err(reqwest_middleware::Error::Reqwest(err)) => {
                Err(transport_from_reqwest(&err, &slot))
            }
            Err(reqwest_middleware::Error::Middleware(_)) => Err(VenueError::Transport {
                kind: TransportKind::Other,
            }),
        }
    }

    /// The subaccount number to use for this order. 0 is the primary subaccount. Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// [`CreateOrderV2::build`] requires [`crate::SubaccountChoice`] before it returns.
    pub async fn create_order_v2(
        &self,
        order: &CreateOrderV2,
    ) -> Result<CreateOrderV2Response, VenueError> {
        let request = order.to_generated().map_err(show_err)?;
        let configuration = &self.configuration;
        self.invoke("create_order_v2", || {
            orders_api::create_order_v2(
                configuration,
                CreateOrderV2Params {
                    create_order_v2_request: request.clone(),
                },
            )
        })
        .await
    }

    /// The subaccount number to use for this order. 0 is the primary subaccount. Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// `SubaccountChoice::KeyRestricted` omits the field, which is the spec's absent value.
    pub async fn create_order_v2_raw(
        &self,
        order: &CreateOrderV2,
    ) -> Result<(StatusCode, Value), VenueError> {
        let request = order.to_generated().map_err(show_err)?;
        let configuration = &self.configuration;
        let (typed, slot) = scope(
            "create_order_v2_raw",
            orders_api::create_order_v2(
                configuration,
                CreateOrderV2Params {
                    create_order_v2_request: request,
                },
            ),
        )
        .await;
        finish_raw(typed, &slot)
    }

    /// A request that does not open the capture slot. The middleware counts it and still returns the response.
    #[cfg(test)]
    pub(crate) async fn execute_without_capture_slot(
        &self,
        url: &str,
    ) -> Result<StatusCode, VenueError> {
        let response = self
            .configuration
            .client
            .get(url)
            .send()
            .await
            .map_err(transport_only)?;
        Ok(response.status())
    }

    async fn invoke<T, E, Fut>(
        &self,
        operation: &str,
        call: impl FnOnce() -> Fut,
    ) -> Result<T, VenueError>
    where
        E: std::fmt::Debug,
        Fut: std::future::Future<Output = Result<T, Error<E>>>,
    {
        let (result, slot) = scope(operation, call()).await;
        match result {
            Ok(value) => Ok(value),
            Err(err) => {
                let mapped = map_generated(err, &slot);
                if matches!(mapped, VenueError::SchemaDrift { .. }) {
                    self.schema_drift.fetch_add(1, Ordering::Relaxed);
                }
                Err(mapped)
            }
        }
    }
}

/// Converts an exchange shard index to the `i32` query used by the list calls whose generated parameter is `i32`.
pub fn exchange_index_query(value: i64) -> Result<i32, VenueError> {
    i32::try_from(value).map_err(|_| VenueError::IndexOutOfRange { value })
}

fn show_err(err: impl std::fmt::Display) -> VenueError {
    VenueError::RequestBuild {
        detail: err.to_string(),
    }
}

fn signed_get_url(base_path: &str, path: &str) -> Result<String, VenueError> {
    if !path.starts_with('/')
        || path.contains('?')
        || path.contains('#')
        || path.contains('\\')
        || path.contains("://")
        || path.chars().any(char::is_whitespace)
    {
        return Err(VenueError::RequestBuild {
            detail: "signed GET requires an absolute path with no query".to_owned(),
        });
    }
    let origin = request_origin(base_path)?;
    Ok(format!("{origin}{}", trade_api_path(path)))
}

fn request_origin(base_path: &str) -> Result<String, VenueError> {
    let mut url = reqwest::Url::parse(base_path).map_err(|_| VenueError::RequestBuild {
        detail: "client base URL has no host".to_owned(),
    })?;
    if url.host_str().is_none() {
        return Err(VenueError::RequestBuild {
            detail: "client base URL has no host".to_owned(),
        });
    }
    url.set_path("/");
    url.set_query(None);
    url.set_fragment(None);
    let origin = url.as_str().trim_end_matches('/');
    if origin.is_empty() {
        return Err(VenueError::RequestBuild {
            detail: "client base URL has no host".to_owned(),
        });
    }
    Ok(origin.to_owned())
}

fn trade_api_path(path: &str) -> String {
    const PREFIX: &str = "/trade-api/v2";
    if path == PREFIX || path.starts_with("/trade-api/v2/") {
        path.to_owned()
    } else {
        format!("{PREFIX}{path}")
    }
}

fn filter_query(subaccount: SubaccountFilter) -> Result<Option<i64>, VenueError> {
    subaccount.query_value().map_err(subaccount_err)
}

fn primary_query(subaccount: Subaccount) -> Result<i64, VenueError> {
    subaccount.query_value().map_err(subaccount_err)
}

fn subaccount_err(err: SubaccountError) -> VenueError {
    show_err(err)
}

fn shard_filter(value: Option<i64>) -> Result<Option<i32>, VenueError> {
    value.map(exchange_index_query).transpose()
}

fn orders_params(
    filter: &OrdersFilter,
    subaccount: Option<i64>,
) -> Result<GetOrdersParams, VenueError> {
    Ok(GetOrdersParams {
        ticker: filter.ticker.clone(),
        event_ticker: filter.event_ticker.clone(),
        min_ts: filter.min_ts,
        max_ts: filter.max_ts,
        status: filter.status.clone(),
        limit: filter.limit,
        cursor: filter.cursor.clone(),
        subaccount,
        exchange_index: shard_filter(filter.exchange_index)?,
    })
}

fn fills_params(
    filter: &FillsFilter,
    subaccount: Option<i64>,
) -> Result<GetFillsParams, VenueError> {
    Ok(GetFillsParams {
        ticker: filter.ticker.clone(),
        order_id: filter.order_id.clone(),
        min_ts: filter.min_ts,
        max_ts: filter.max_ts,
        limit: filter.limit,
        cursor: filter.cursor.clone(),
        subaccount,
        exchange_index: shard_filter(filter.exchange_index)?,
    })
}

fn positions_params(
    filter: &PositionsFilter,
    subaccount: Option<i64>,
) -> Result<GetPositionsParams, VenueError> {
    Ok(GetPositionsParams {
        settlement_status: filter.settlement_status.clone(),
        cursor: filter.cursor.clone(),
        limit: filter.limit,
        count_filter: filter.count_filter.clone(),
        ticker: filter.ticker.clone(),
        event_ticker: filter.event_ticker.clone(),
        subaccount,
        exchange_index: shard_filter(filter.exchange_index)?,
    })
}

fn queue_params(
    filter: &OrderQueuePositionsFilter,
    subaccount: Option<i64>,
) -> GetOrderQueuePositionsParams {
    GetOrderQueuePositionsParams {
        market_tickers: filter.market_tickers.clone(),
        event_ticker: filter.event_ticker.clone(),
        subaccount,
    }
}

fn markets_params(filter: &MarketsFilter) -> GetMarketsParams {
    GetMarketsParams {
        limit: filter.limit,
        cursor: filter.cursor.clone(),
        event_ticker: filter.event_ticker.clone(),
        series_ticker: filter.series_ticker.clone(),
        min_created_ts: filter.min_created_ts,
        max_created_ts: filter.max_created_ts,
        min_updated_ts: filter.min_updated_ts,
        max_updated_ts: filter.max_updated_ts,
        max_close_ts: filter.max_close_ts,
        min_close_ts: filter.min_close_ts,
        min_settled_ts: filter.min_settled_ts,
        max_settled_ts: filter.max_settled_ts,
        status: filter.status.clone(),
        tickers: filter.tickers.clone(),
        mve_filter: filter.mve_filter.clone(),
    }
}

fn batch_create_body(orders: &[CreateOrderV2]) -> Result<BatchCreateOrdersV2Request, VenueError> {
    let mut generated = Vec::with_capacity(orders.len());
    for order in orders {
        generated.push(order.to_generated().map_err(show_err)?);
    }
    Ok(BatchCreateOrdersV2Request::new(generated))
}

fn batch_cancel_body(orders: &[BatchCancelItem]) -> Result<BatchCancelOrdersV2Request, VenueError> {
    let mut generated = Vec::with_capacity(orders.len());
    for order in orders {
        generated.push(order.to_generated().map_err(show_err)?);
    }
    Ok(BatchCancelOrdersV2Request::new(generated))
}

/// Result of a `_raw` call. When no status line was captured, the generated
/// call's own error decides the kind, as on the typed path: a request that was
/// sent but got no reply is `Timeout` (outcome unknown), not `Connect`.
fn finish_raw<T, E: std::fmt::Debug>(
    typed: Result<T, Error<E>>,
    slot: &CaptureSlot,
) -> Result<(StatusCode, Value), VenueError> {
    if slot.status.is_none() && slot.body_error.is_none() && !slot.too_large {
        return Err(match typed {
            Err(err) => map_generated(err, slot),
            Ok(_) => VenueError::Transport {
                kind: TransportKind::Other,
            },
        });
    }
    finish_captured(slot)
}

fn finish_captured(slot: &CaptureSlot) -> Result<(StatusCode, Value), VenueError> {
    if let Some(kind) = slot.body_error {
        return Err(VenueError::HttpBodyFailed {
            status: slot.status.unwrap_or(0),
            body_error: kind,
            prefix: slot.prefix.clone(),
        });
    }
    if slot.too_large {
        return Err(VenueError::BodyTooLarge {
            status: slot.status.unwrap_or(0),
            total_seen: slot.total,
            prefix: slot.prefix.clone(),
        });
    }
    let status = slot.status.ok_or(VenueError::Transport {
        kind: TransportKind::Connect,
    })?;
    if slot.truncated {
        return Err(VenueError::TruncatedRaw {
            status,
            prefix: slot.prefix.clone(),
            total: slot.total,
        });
    }
    match serde_json::from_slice::<Value>(&slot.full) {
        Ok(value) => Ok((
            StatusCode::from_u16(status).unwrap_or(StatusCode::OK),
            value,
        )),
        Err(_) => Err(VenueError::NonJson {
            status,
            body_prefix: slot.prefix.clone(),
            truncated: false,
            total: slot.total,
        }),
    }
}

fn map_generated<E: std::fmt::Debug>(err: Error<E>, slot: &CaptureSlot) -> VenueError {
    if slot.too_large {
        return VenueError::BodyTooLarge {
            status: slot.status.unwrap_or(0),
            total_seen: slot.total,
            prefix: slot.prefix.clone(),
        };
    }
    if let Some(kind) = slot.body_error {
        if let Some(status) = slot.status {
            return VenueError::HttpBodyFailed {
                status,
                body_error: kind,
                prefix: slot.prefix.clone(),
            };
        }
        return VenueError::Transport {
            kind: transport_kind_from_body(kind),
        };
    }
    match err {
        Error::ResponseError(response) => {
            let status = response.status.as_u16();
            let raw = response.content.into_bytes();
            let (code, message) = json_code_message(&raw);
            VenueError::Http {
                status,
                code,
                message,
                raw,
                truncated: slot.truncated,
                total: slot.total,
            }
        }
        Error::Serde(_) => classify_decode(slot),
        Error::Reqwest(err) => transport_from_reqwest(&err, slot),
        Error::ReqwestMiddleware(err) => match err {
            reqwest_middleware::Error::Reqwest(err) => transport_from_reqwest(&err, slot),
            reqwest_middleware::Error::Middleware(_) => {
                if let Some(status) = slot.status {
                    VenueError::HttpBodyFailed {
                        status,
                        body_error: BodyErrorKind::Other,
                        prefix: slot.prefix.clone(),
                    }
                } else {
                    VenueError::Transport {
                        kind: TransportKind::Other,
                    }
                }
            }
        },
        Error::Io(_) => VenueError::Transport {
            kind: TransportKind::Other,
        },
    }
}

fn classify_decode(slot: &CaptureSlot) -> VenueError {
    let status = slot.status.unwrap_or(0);
    // Only the full body counts. A prefix that merely starts with '{' is not JSON.
    if serde_json::from_slice::<Value>(&slot.full).is_ok() {
        VenueError::SchemaDrift {
            operation: slot.operation.clone(),
            status,
            prefix: slot.prefix.clone(),
            truncated: slot.truncated,
            total: slot.total,
        }
    } else {
        VenueError::NonJson {
            status,
            body_prefix: slot.prefix.clone(),
            truncated: slot.truncated,
            total: slot.total,
        }
    }
}

fn json_code_message(raw: &[u8]) -> (Option<String>, Option<String>) {
    let Ok(value) = serde_json::from_slice::<Value>(raw) else {
        return (None, None);
    };
    let code = value.get("code").and_then(Value::as_str).map(str::to_owned);
    let message = value
        .get("message")
        .and_then(Value::as_str)
        .map(str::to_owned);
    (code, message)
}

fn transport_from_reqwest(err: &reqwest::Error, slot: &CaptureSlot) -> VenueError {
    if let Some(status) = slot.status {
        return VenueError::HttpBodyFailed {
            status,
            body_error: if err.is_timeout() {
                BodyErrorKind::Timeout
            } else {
                BodyErrorKind::Reset
            },
            prefix: slot.prefix.clone(),
        };
    }
    VenueError::Transport {
        kind: if err.is_timeout() {
            TransportKind::Timeout
        } else if err.is_connect() {
            TransportKind::Connect
        } else {
            TransportKind::Other
        },
    }
}

fn transport_kind_from_body(kind: BodyErrorKind) -> TransportKind {
    match kind {
        BodyErrorKind::Timeout => TransportKind::Timeout,
        BodyErrorKind::Reset => TransportKind::Connect,
        BodyErrorKind::Other => TransportKind::Other,
    }
}

#[cfg(test)]
fn transport_only(err: reqwest_middleware::Error) -> VenueError {
    match err {
        reqwest_middleware::Error::Reqwest(err) => transport_from_reqwest(&err, &empty_slot()),
        reqwest_middleware::Error::Middleware(_) => VenueError::Transport {
            kind: TransportKind::Other,
        },
    }
}

#[cfg(test)]
fn empty_slot() -> CaptureSlot {
    CaptureSlot {
        operation: String::new(),
        status: None,
        content_type: None,
        date: None,
        prefix: Vec::new(),
        full: Vec::new(),
        truncated: false,
        total: 0,
        body_error: None,
        too_large: false,
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod pass_through_tests {
    use super::{Client, ClientConfig};
    use rsa::pkcs8::{EncodePrivateKey, LineEnding};
    use rsa::RsaPrivateKey;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn unscoped_request_increments_the_counter() {
        let mut rng = rand::rngs::OsRng;
        let private = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let pem = private.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let mut buf = [0u8; 1024];
            let _ = socket.read(&mut buf).await;
            let body = b"ok";
            let head = format!(
                "HTTP/1.1 204 No Content\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            );
            let _ = socket.write_all(head.as_bytes()).await;
            let _ = socket.write_all(body).await;
        });
        let client = Client::new(ClientConfig {
            base_url: format!("http://{addr}"),
            key_id: "key-synthetic".to_owned(),
            private_key_pem: pem,
            connect_timeout: Some(Duration::from_secs(2)),
            request_timeout: Some(Duration::from_secs(2)),
            max_body_bytes: Some(1000),
            max_capture_bytes: 1000,
        })
        .unwrap();
        let before = client.pass_through_count();
        client
            .execute_without_capture_slot(&format!("http://{addr}/health"))
            .await
            .unwrap();
        assert_eq!(client.pass_through_count(), before + 1);
    }
}

#[cfg(test)]
mod signed_get_url_tests {
    use super::{signed_get_url, VenueError};

    #[test]
    fn absolute_path_uses_the_origin_and_rejects_a_query() {
        let url = signed_get_url(
            "https://external-api.kalshi.com/trade-api/v2",
            "/trade-api/v2/exchange/user_data_timestamp",
        )
        .unwrap();
        assert_eq!(
            url,
            "https://external-api.kalshi.com/trade-api/v2/exchange/user_data_timestamp"
        );
        assert!(matches!(
            signed_get_url(
                "https://external-api.kalshi.com/trade-api/v2",
                "/trade-api/v2/portfolio/orders?subaccount=1",
            ),
            Err(VenueError::RequestBuild { .. })
        ));
    }

    #[test]
    fn origin_only_and_prefixed_bases_share_the_trade_api_path() {
        let short = "/exchange/user_data_timestamp";
        let full = "/trade-api/v2/exchange/user_data_timestamp";
        let expected = "https://external-api.kalshi.com/trade-api/v2/exchange/user_data_timestamp";
        for base in [
            "https://external-api.kalshi.com",
            "https://external-api.kalshi.com/",
            "https://external-api.kalshi.com/trade-api/v2",
            "https://external-api.kalshi.com/trade-api/v2/",
        ] {
            assert_eq!(signed_get_url(base, short).unwrap(), expected, "{base}");
            assert_eq!(signed_get_url(base, full).unwrap(), expected, "{base}");
        }
        assert_eq!(
            signed_get_url("http://127.0.0.1:9", short).unwrap(),
            "http://127.0.0.1:9/trade-api/v2/exchange/user_data_timestamp"
        );
        assert!(matches!(
            signed_get_url("not-a-url", short),
            Err(VenueError::RequestBuild { .. })
        ));
    }
}
