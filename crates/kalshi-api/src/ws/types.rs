use super::payloads::*;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The key is required. JSON null is `None`. A missing key is an error.
pub(super) fn required_nullable<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer)
}

/// Outcome side. Orderbook deltas use this for `side`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketSide {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
}

/// Book side. `bid` matches outcome side `yes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookSide {
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "ask")]
    Ask,
}

/// Legacy fill action. The empty string is a known wire value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderAction {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell_partial")]
    SellPartial,
    #[serde(rename = "sell_to_close")]
    SellToClose,
    #[serde(rename = "")]
    Unspecified,
}

/// Closed `user_order` status. The wire value `unknown` is one of the known statuses.
/// Any other value fails the frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserOrderStatus {
    #[serde(rename = "resting")]
    Resting,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Known update reasons stay typed. A reason outside the list is [`LastUpdateReason::Unknown`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LastUpdateReason {
    Decrease,
    Amend,
    MarginCancel,
    SelfTradeCancel,
    ExpiryCancel,
    CloseCancel,
    HaltCancel,
    Trade,
    PostOnlyCrossCancel,
    ReduceOnlyCancel,
    SettlementBoundsCancel,
    Unknown(String),
}

impl Serialize for LastUpdateReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let wire = match self {
            Self::Decrease => "Decrease",
            Self::Amend => "Amend",
            Self::MarginCancel => "MarginCancel",
            Self::SelfTradeCancel => "SelfTradeCancel",
            Self::ExpiryCancel => "ExpiryCancel",
            Self::CloseCancel => "CloseCancel",
            Self::HaltCancel => "HaltCancel",
            Self::Trade => "Trade",
            Self::PostOnlyCrossCancel => "PostOnlyCrossCancel",
            Self::ReduceOnlyCancel => "ReduceOnlyCancel",
            Self::SettlementBoundsCancel => "SettlementBoundsCancel",
            Self::Unknown(value) => value,
        };
        serializer.serialize_str(wire)
    }
}

impl<'de> Deserialize<'de> for LastUpdateReason {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Decrease" => Self::Decrease,
            "Amend" => Self::Amend,
            "MarginCancel" => Self::MarginCancel,
            "SelfTradeCancel" => Self::SelfTradeCancel,
            "ExpiryCancel" => Self::ExpiryCancel,
            "CloseCancel" => Self::CloseCancel,
            "HaltCancel" => Self::HaltCancel,
            "Trade" => Self::Trade,
            "PostOnlyCrossCancel" => Self::PostOnlyCrossCancel,
            "ReduceOnlyCancel" => Self::ReduceOnlyCancel,
            "SettlementBoundsCancel" => Self::SettlementBoundsCancel,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle,
    #[serde(rename = "communications")]
    Communications,
    #[serde(rename = "batch_communications")]
    BatchCommunications,
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates,
    #[serde(rename = "pyth_value")]
    PythValue,
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "market_positions")]
    MarketPositions,
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2,
    #[serde(rename = "user_orders")]
    UserOrders,
    #[serde(rename = "cfbenchmarks_value")]
    CfBenchmarksValue,
    #[serde(rename = "cfbenchmarks_value_5hz")]
    CfBenchmarksValue5Hz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionAction {
    #[serde(rename = "subscribe_indices")]
    SubscribeIndices,
    #[serde(rename = "unsubscribe_indices")]
    UnsubscribeIndices,
    #[serde(rename = "subscribe_underlyings")]
    SubscribeUnderlyings,
    #[serde(rename = "unsubscribe_underlyings")]
    UnsubscribeUnderlyings,
    #[serde(rename = "underlying_list")]
    UnderlyingList,
    #[serde(rename = "add_markets")]
    AddMarkets,
    #[serde(rename = "delete_markets")]
    DeleteMarkets,
    #[serde(rename = "get_snapshot")]
    GetSnapshot,
    /// Ask a `cfbenchmarks_value` or `cfbenchmarks_value_5hz` subscription for its index ids.
    ///
    /// The command is `update_subscription` with `action` `indexlist`. The subscription id
    /// selects the channel. The server replies with that channel's index list and does not
    /// change the subscription.
    #[serde(rename = "indexlist")]
    IndexList,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscribeParams {
    pub channels: Vec<Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
    /// Orderbook only. `true` reports both sides in yes-leg prices.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_yes_price: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_factor: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_tickers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsubscribeParams {
    pub sids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSubscriptionParams {
    pub action: SubscriptionAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_tickers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sids: Option<Vec<u64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ClientCommand {
    ListSubscriptions {
        id: u64,
    },
    Subscribe {
        id: u64,
        params: SubscribeParams,
    },
    Unsubscribe {
        id: u64,
        params: UnsubscribeParams,
    },
    UpdateSubscription {
        id: u64,
        params: UpdateSubscriptionParams,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscribedBody {
    pub channel: String,
    pub sid: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: i64,
    pub msg: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OkBody {
    Indices(CfBenchmarksIndexListBody),
    Underlyings(UnderlyingListBody),
    Subscriptions(Vec<SubscribedBody>),
    Markets(OkMarketsBody),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OkMarketsBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderbookSnapshotBody {
    pub market_ticker: String,
    pub market_id: String,
    pub clock: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "yes_dollars"
    )]
    pub yes_dollars_fp: Option<Vec<Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "no_dollars")]
    pub no_dollars_fp: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderbookDeltaBody {
    pub market_ticker: String,
    pub market_id: String,
    pub clock: i64,
    pub price_dollars: String,
    pub delta_fp: String,
    pub side: MarketSide,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeBody {
    pub trade_id: String,
    pub market_ticker: String,
    pub yes_price_dollars: String,
    pub no_price_dollars: String,
    pub count_fp: String,
    pub taker_side: MarketSide,
    pub taker_outcome_side: MarketSide,
    pub taker_book_side: BookSide,
    pub is_block_trade: bool,
    pub ts: i64,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FillBody {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    pub exchange_index: i64,
    pub is_taker: bool,
    pub side: MarketSide,
    pub yes_price_dollars: String,
    pub count_fp: String,
    pub fee_cost: String,
    pub action: OrderAction,
    pub outcome_side: MarketSide,
    pub book_side: BookSide,
    pub ts: i64,
    pub ts_ms: i64,
    pub post_position_fp: String,
    pub purchased_side: MarketSide,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleEvent {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deactivated")]
    Deactivated,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "close_date_updated")]
    CloseDateUpdated,
    #[serde(rename = "determined")]
    Determined,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "price_level_structure_updated")]
    PriceLevelStructureUpdated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleAdditionalMetadata {
    pub name: String,
    pub title: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub can_close_early: bool,
    pub event_ticker: String,
    pub expected_expiration_ts: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_strike: Option<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap_strike: Option<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_strike: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketLifecycleBody {
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_ts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_ts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub determination_ts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settled_ts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deactivated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<LifecycleAdditionalMetadata>,
    pub event_type: LifecycleEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_level_structure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_ranges: Option<Vec<PriceRange>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LifecycleMessage {
    Metadata(MarketMetadataBody),
    Lifecycle(Box<MarketLifecycleBody>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketPositionBody {
    pub user_id: String,
    pub market_ticker: String,
    pub position_fp: String,
    pub position_cost_dollars: String,
    pub realized_pnl_dollars: String,
    pub fees_paid_dollars: String,
    pub position_fee_cost_dollars: String,
    pub volume_fp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfTradePrevention {
    #[serde(rename = "taker_at_cross")]
    TakerAtCross,
    #[serde(rename = "maker")]
    Maker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOrderBody {
    pub order_id: String,
    pub user_id: String,
    pub ticker: String,
    pub exchange_index: i64,
    pub status: UserOrderStatus,
    pub side: MarketSide,
    pub is_yes: bool,
    pub outcome_side: MarketSide,
    pub book_side: BookSide,
    pub yes_price_dollars: String,
    pub fill_count_fp: String,
    pub remaining_count_fp: String,
    pub initial_count_fp: String,
    pub taker_fill_cost_dollars: String,
    pub maker_fill_cost_dollars: String,
    pub taker_fees_dollars: String,
    pub maker_fees_dollars: String,
    pub client_order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<SelfTradePrevention>,
    #[serde(deserialize_with = "required_nullable")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_ts_ms: Option<i64>,
    #[serde(deserialize_with = "required_nullable")]
    pub created_ts_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_reason: Option<LastUpdateReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Incoming {
    #[serde(rename = "subscribed")]
    Subscribed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        msg: SubscribedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        sid: u64,
        seq: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "ok")]
    Ok {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sid: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        seq: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        msg: Option<OkBody>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "error")]
    Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sid: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        seq: Option<u64>,
        msg: ErrorBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        sid: u64,
        seq: u64,
        msg: OrderbookSnapshotBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: u64,
        seq: u64,
        msg: OrderbookDeltaBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: u64,
        seq: u64,
        msg: TradeBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: u64,
        seq: u64,
        msg: LifecycleMessage,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: u64,
        msg: FillBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: u64,
        msg: UserOrderBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    /// Wire type is `market_position`, not the channel name `market_positions`.
    #[serde(rename = "market_position")]
    MarketPosition {
        sid: u64,
        msg: MarketPositionBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "cfbenchmarks_value")]
    CfBenchmarksValue {
        sid: u64,
        seq: u64,
        msg: CfBenchmarksValueBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "cfbenchmarks_value_indexlist")]
    CfBenchmarksIndexList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        sid: u64,
        seq: u64,
        msg: CfBenchmarksIndexListBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "cfbenchmarks_value_5hz")]
    CfBenchmarksValue5Hz {
        sid: u64,
        seq: u64,
        msg: CfBenchmarksValue5HzBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "cfbenchmarks_value_5hz_indexlist")]
    CfBenchmarksIndexList5Hz {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        sid: u64,
        seq: u64,
        msg: CfBenchmarksIndexListBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "pyth_value")]
    PythValue {
        sid: u64,
        seq: u64,
        msg: PythValueBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "pyth_value_underlying_list")]
    PythUnderlyingList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
        sid: u64,
        seq: u64,
        msg: UnderlyingListBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: u64,
        msg: TickerBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: u64,
        seq: u64,
        msg: MultivariateLifecycleBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "event_lifecycle")]
    EventLifecycle {
        sid: u64,
        seq: u64,
        msg: EventLifecycleBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "event_fee_update")]
    EventFeeUpdate {
        sid: u64,
        seq: u64,
        msg: EventFeeBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: u64,
        seq: u64,
        msg: OrderGroupBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: u64,
        seq: u64,
        msg: RfqCreatedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: u64,
        seq: u64,
        msg: RfqDeletedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: u64,
        seq: u64,
        msg: QuoteCreatedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: u64,
        seq: u64,
        msg: QuoteAcceptedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: u64,
        seq: u64,
        msg: QuoteExecutedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "rfq_batch_created")]
    RfqBatchCreated {
        sid: u64,
        seq: u64,
        msg: RfqBatchCreatedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "rfq_batch_deleted")]
    RfqBatchDeleted {
        sid: u64,
        seq: u64,
        msg: RfqBatchDeletedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "batch_quote_created")]
    BatchQuoteCreated {
        sid: u64,
        seq: u64,
        msg: BatchQuoteCreatedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "rfq_batch_best_quotes")]
    RfqBatchBestQuotes {
        sid: u64,
        seq: u64,
        msg: RfqBatchBestQuotesBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "batch_quote_accepted")]
    BatchQuoteAccepted {
        sid: u64,
        seq: u64,
        msg: BatchQuoteAcceptedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "rfq_batch_executed")]
    RfqBatchExecuted {
        sid: u64,
        seq: u64,
        msg: RfqBatchExecutedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "batch_quote_executed")]
    BatchQuoteExecuted {
        sid: u64,
        seq: u64,
        msg: BatchQuoteExecutedBody,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sending_ts_ms: Option<i64>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CfBenchmarksAvgData {
    pub value: String,
    pub window_size: i64,
    pub window_start_ts_ms: i64,
    pub window_end_ts_exclusive: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CfBenchmarksValueBody {
    pub index_id: String,
    pub received_at: i64,
    pub data: String,
    pub avg_60s_data: CfBenchmarksAvgData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_60s_windowed_average_15min: Option<CfBenchmarksAvgData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CfBenchmarksIndexListBody {
    pub index_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CfBenchmarksValue5HzBody {
    pub index_id: String,
    pub value_usd: String,
    pub source_ts_ms: i64,
    pub received_at: i64,
    pub data: String,
}

pub fn parse_incoming(text: &str) -> Result<Incoming, serde_json::Error> {
    serde_json::from_str(text)
}

#[cfg(test)]
mod tests {
    use super::{parse_incoming, Incoming, LastUpdateReason, UserOrderStatus};

    #[test]
    fn wire_unknown_status_is_a_known_variant() {
        let frame = r#"{"type":"user_order","sid":1,"msg":{"order_id":"ord-synthetic-1","user_id":"user-synthetic","ticker":"SYNTH-1","exchange_index":0,"status":"unknown","side":"yes","is_yes":true,"outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.5600","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.000000","maker_fill_cost_dollars":"0.000000","taker_fees_dollars":"0.000000","maker_fees_dollars":"0.000000","client_order_id":"cid-synthetic","created_time":null,"created_ts_ms":null,"last_update_reason":"Trade"}}"#;
        match parse_incoming(frame).unwrap() {
            Incoming::UserOrder { msg, .. } => {
                assert_eq!(msg.status, UserOrderStatus::Unknown);
                assert_eq!(msg.last_update_reason, Some(LastUpdateReason::Trade));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn new_user_order_status_fails_the_frame() {
        let frame = r#"{"type":"user_order","sid":1,"msg":{"order_id":"ord-synthetic-1","user_id":"user-synthetic","ticker":"SYNTH-1","exchange_index":0,"status":"weird","side":"yes","is_yes":true,"outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.5600","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.000000","maker_fill_cost_dollars":"0.000000","taker_fees_dollars":"0.000000","maker_fees_dollars":"0.000000","client_order_id":"cid-synthetic","created_time":null,"created_ts_ms":null}}"#;
        assert!(parse_incoming(frame).is_err());
    }

    #[test]
    fn new_last_update_reason_is_kept() {
        let frame = r#"{"type":"user_order","sid":1,"msg":{"order_id":"ord-synthetic-1","user_id":"user-synthetic","ticker":"SYNTH-1","exchange_index":0,"status":"resting","side":"yes","is_yes":true,"outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.5600","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.000000","maker_fill_cost_dollars":"0.000000","taker_fees_dollars":"0.000000","maker_fees_dollars":"0.000000","client_order_id":"cid-synthetic","created_time":null,"created_ts_ms":null,"last_update_reason":"NotInTheList"}}"#;
        match parse_incoming(frame).unwrap() {
            Incoming::UserOrder { msg, .. } => {
                assert_eq!(
                    msg.last_update_reason,
                    Some(LastUpdateReason::Unknown("NotInTheList".to_owned()))
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn orderbook_clock_is_required() {
        let snapshot = r#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"SYNTH-1","market_id":"m","clock":4182057}}"#;
        match parse_incoming(snapshot).unwrap() {
            Incoming::OrderbookSnapshot { msg, .. } => assert_eq!(msg.clock, 4182057),
            other => panic!("unexpected {other:?}"),
        }
        let delta = r#"{"type":"orderbook_delta","sid":1,"seq":2,"msg":{"market_ticker":"SYNTH-1","market_id":"m","clock":4182061,"price_dollars":"0.5600","delta_fp":"1.00","side":"yes"}}"#;
        match parse_incoming(delta).unwrap() {
            Incoming::OrderbookDelta { msg, .. } => assert_eq!(msg.clock, 4182061),
            other => panic!("unexpected {other:?}"),
        }
        for frame in [
            r#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"SYNTH-1","market_id":"m"}}"#,
            r#"{"type":"orderbook_delta","sid":1,"seq":2,"msg":{"market_ticker":"SYNTH-1","market_id":"m","price_dollars":"0.5600","delta_fp":"1.00","side":"yes"}}"#,
        ] {
            assert!(parse_incoming(frame).is_err(), "{frame}");
        }
    }

    #[test]
    fn old_orderbook_dollars_alias_is_kept() {
        let frame = r#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"SYNTH-1","market_id":"m","clock":0,"yes_dollars":[["0.5600","1.00"]],"no_dollars":[["0.4400","2.00"]]}}"#;
        match parse_incoming(frame).unwrap() {
            Incoming::OrderbookSnapshot { msg, .. } => {
                assert_eq!(
                    msg.yes_dollars_fp,
                    Some(vec![vec!["0.5600".to_owned(), "1.00".to_owned()]])
                );
                assert_eq!(
                    msg.no_dollars_fp,
                    Some(vec![vec!["0.4400".to_owned(), "2.00".to_owned()]])
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn cfbenchmarks_value_and_5hz_frames_parse() {
        let value = r#"{"type":"cfbenchmarks_value","sid":1,"seq":42,"msg":{"index_id":"BRTI","received_at":1710000000123,"data":"{\"type\":\"value\"}","avg_60s_data":{"value":"68000.12000000","window_size":3,"window_start_ts_ms":1709999940123,"window_end_ts_exclusive":1710000000123}}}"#;
        match parse_incoming(value).unwrap() {
            Incoming::CfBenchmarksValue { msg, .. } => assert_eq!(msg.index_id, "BRTI"),
            other => panic!("unexpected {other:?}"),
        }
        let list = r#"{"type":"cfbenchmarks_value_indexlist","id":2,"sid":1,"seq":1,"msg":{"index_ids":["BRTI","ETHUSD_RTI"]}}"#;
        match parse_incoming(list).unwrap() {
            Incoming::CfBenchmarksIndexList { id, msg, .. } => {
                assert_eq!(id, Some(2));
                assert_eq!(msg.index_ids, ["BRTI", "ETHUSD_RTI"]);
            }
            other => panic!("unexpected {other:?}"),
        }
        let fast = r#"{"type":"cfbenchmarks_value_5hz","sid":1,"seq":42,"msg":{"index_id":"BRTI","value_usd":"68000.12000000","source_ts_ms":1710000000323,"received_at":1710000000341,"data":"{\"type\":\"value\"}"}}"#;
        match parse_incoming(fast).unwrap() {
            Incoming::CfBenchmarksValue5Hz { msg, .. } => {
                assert_eq!(msg.value_usd, "68000.12000000")
            }
            other => panic!("unexpected {other:?}"),
        }
        let fast_list = r#"{"type":"cfbenchmarks_value_5hz_indexlist","sid":1,"seq":1,"msg":{"index_ids":["BRTI"]}}"#;
        match parse_incoming(fast_list).unwrap() {
            Incoming::CfBenchmarksIndexList5Hz { id, msg, .. } => {
                assert!(id.is_none());
                assert_eq!(msg.index_ids, ["BRTI"]);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
