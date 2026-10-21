use serde::{Deserialize, Serialize};

use super::types::{required_nullable, LifecycleAdditionalMetadata, MarketSide};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultivariateLifecycleEvent {
    Created,
    Deactivated,
    Activated,
    CloseDateUpdated,
    Determined,
    Settled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultivariateLifecycleBody {
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
    pub event_type: MultivariateLifecycleEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_level_structure: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderlyingListBody {
    pub underlying_tickers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythValueBody {
    pub underlying_ticker: String,
    pub value_usd: String,
    pub source_ts_ms: i64,
    pub received_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TickerBody {
    pub market_id: String,
    pub market_ticker: String,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub yes_ask_dollars: String,
    pub volume_fp: String,
    pub open_interest_fp: String,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    pub yes_bid_size_fp: String,
    pub yes_ask_size_fp: String,
    pub last_trade_size_fp: String,
    pub ts: i64,
    pub ts_ms: i64,
    pub time: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketMetadataBody {
    pub event_type: MetadataEvent,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_strike: Option<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap_strike: Option<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_strike: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_sub_title: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetadataEvent {
    #[serde(rename = "metadata_updated")]
    MetadataUpdated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollateralReturnType {
    #[serde(rename = "MECNET")]
    MutuallyExclusive,
    #[serde(rename = "DIRECNET")]
    Directional,
    #[serde(rename = "")]
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventLifecycleBody {
    pub event_ticker: String,
    pub exchange_index: i64,
    pub title: String,
    pub subtitle: String,
    pub collateral_return_type: CollateralReturnType,
    pub series_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_date: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_period: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    Quadratic,
    QuadraticWithMakerFees,
    QuadraticWithComboMakerFees,
    Flat,
    MarginMarketMakerProgramFees,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventFeeBody {
    pub event_ticker: String,
    #[serde(deserialize_with = "required_nullable")]
    pub fee_type_override: Option<FeeType>,
    #[serde(deserialize_with = "required_nullable")]
    pub fee_multiplier_override: Option<serde_json::Number>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderGroupEvent {
    Created,
    Triggered,
    Reset,
    Deleted,
    LimitUpdated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderGroupBody {
    pub event_type: OrderGroupEvent,
    pub order_group_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: MarketSide,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_settlement_value_dollars: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqCreatedBody {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    pub created_ts: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_selected_legs: Option<Vec<SelectedLeg>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqDeletedBody {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    pub deleted_ts: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuoteCreatedBody {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<String>,
    pub created_ts: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuoteAcceptedBody {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_side: Option<MarketSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_accepted_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuoteExecutedBody {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub order_id: String,
    pub client_order_id: String,
    pub market_ticker: String,
    pub executed_ts: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_batch_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqBatchChildRfq {
    pub id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    pub leg_indices: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqBatchCreatedBody {
    pub id: String,
    pub creator_id: String,
    pub created_ts: String,
    pub legs: Vec<SelectedLeg>,
    pub rfqs: Vec<RfqBatchChildRfq>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqBatchDeletedBody {
    pub id: String,
    pub creator_id: String,
    pub rfq_ids: Vec<String>,
    pub deleted_ts: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchQuoteCreatedChild {
    pub quote_id: String,
    pub rfq_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchQuoteCreatedBody {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub quote_creator_id: String,
    pub created_ts: String,
    pub quotes: Vec<BatchQuoteCreatedChild>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqBatchBestQuote {
    pub rfq_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_bid_dollars: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_bid_dollars: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqBatchBestQuotesBody {
    pub rfq_batch_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_batch_id: Option<String>,
    pub version: i64,
    pub updated_ts: String,
    pub yes_complete: bool,
    pub no_complete: bool,
    pub rfqs: Vec<RfqBatchBestQuote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchQuoteAcceptedChild {
    pub quote_id: String,
    pub rfq_id: String,
    pub market_ticker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_accepted_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_offered_fp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchQuoteAcceptedBody {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub accepted_side: MarketSide,
    pub accepted_ts: String,
    pub quotes: Vec<BatchQuoteAcceptedChild>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfqBatchExecutedBody {
    pub rfq_batch_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_batch_id: Option<String>,
    pub rfq_creator_id: String,
    pub executed_rfq_ids: Vec<String>,
    pub failed_rfq_ids: Vec<String>,
    pub executed_ts: String,
    pub close_reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchQuoteExecutedBody {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub rfq_creator_id: String,
    pub executed_rfq_ids: Vec<String>,
    pub failed_rfq_ids: Vec<String>,
    pub executed_ts: String,
    pub close_reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}
