use std::fmt;

use kalshi_api_openapi::models::create_order_v2_request::TimeInForce;
use kalshi_api_openapi::models::{BookSide, CreateOrderV2Request, SelfTradePreventionType};
use rust_decimal::Decimal;

use crate::money::{decimal_wire, FixedPointError, QtyCenti};
use crate::subaccount::{SubaccountChoice, SubaccountError};

/// Caller must choose a shard or an omitted index. There is no default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeIndexChoice {
    Shard(i64),
    Omit,
}

/// Required choice for `post_only`, `reduce_only`, and `cancel_order_on_pause`.
///
/// The official Python SDK leaves each field optional. [`CreateOrderV2::build`]
/// requires one of these for each flag. `True` sends `true` and `False` sends
/// `false`. `VenueDefault` omits the field, which is the venue's absent value,
/// and has to be selected on purpose.
///
/// Leaving a flag out does not compile:
///
/// ```compile_fail,E0061
/// use kalshi_api::{CreateOrderV2, ExchangeIndexChoice, QtyCenti, SubaccountChoice};
/// use kalshi_api_openapi::models::create_order_v2_request::TimeInForce;
/// use kalshi_api_openapi::models::{BookSide, SelfTradePreventionType};
/// use rust_decimal::Decimal;
/// use std::str::FromStr;
/// let _ = CreateOrderV2::build(
///     "SYNTH-1",
///     BookSide::Bid,
///     QtyCenti::parse("1.00").unwrap(),
///     Decimal::from_str("0.5600").unwrap(),
///     TimeInForce::GoodTillCanceled,
///     SelfTradePreventionType::Maker,
///     "cid-synthetic",
///     ExchangeIndexChoice::Omit,
///     SubaccountChoice::Primary,
/// );
/// ```
///
/// There is no [`Default`] that fills a flag in:
///
/// ```compile_fail,E0277
/// fn need_default<T: Default>() {}
/// need_default::<kalshi_api::OrderFlag>();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderFlag {
    True,
    False,
    VenueDefault,
}

impl OrderFlag {
    /// `True` and `False` are sent. `VenueDefault` is the absent field.
    pub(crate) fn wire(self) -> Option<bool> {
        match self {
            Self::True => Some(true),
            Self::False => Some(false),
            Self::VenueDefault => None,
        }
    }
}

impl ExchangeIndexChoice {
    pub(crate) fn as_option(self) -> Option<i64> {
        match self {
            Self::Shard(index) => Some(index),
            Self::Omit => None,
        }
    }
}

#[derive(Debug)]
pub enum BuildError {
    FixedPoint(FixedPointError),
    Subaccount(SubaccountError),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FixedPoint(err) => write!(f, "{err}"),
            Self::Subaccount(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<FixedPointError> for BuildError {
    fn from(err: FixedPointError) -> Self {
        Self::FixedPoint(err)
    }
}

impl From<SubaccountError> for BuildError {
    fn from(err: SubaccountError) -> Self {
        Self::Subaccount(err)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOrderV2 {
    ticker: String,
    side: BookSide,
    count: QtyCenti,
    price: Decimal,
    time_in_force: TimeInForce,
    self_trade_prevention_type: SelfTradePreventionType,
    client_order_id: String,
    exchange_index: ExchangeIndexChoice,
    subaccount: SubaccountChoice,
    post_only: OrderFlag,
    reduce_only: OrderFlag,
    cancel_order_on_pause: OrderFlag,
}

impl CreateOrderV2 {
    /// The subaccount number to use for this order. 0 is the primary subaccount. Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// `subaccount` is required. `SubaccountChoice::KeyRestricted` omits the JSON field.
    ///
    /// `post_only`, `reduce_only`, and `cancel_order_on_pause` are required.
    /// `OrderFlag::VenueDefault` omits that field. `OrderFlag::False` is sent.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        ticker: impl Into<String>,
        side: BookSide,
        count: QtyCenti,
        price: Decimal,
        time_in_force: TimeInForce,
        self_trade_prevention_type: SelfTradePreventionType,
        client_order_id: impl Into<String>,
        exchange_index: ExchangeIndexChoice,
        subaccount: SubaccountChoice,
        post_only: OrderFlag,
        reduce_only: OrderFlag,
        cancel_order_on_pause: OrderFlag,
    ) -> Result<Self, BuildError> {
        if count.0 <= 0 {
            return Err(FixedPointError {
                input: count.wire(),
            }
            .into());
        }
        let _price_wire = decimal_wire(price, 4)?;
        subaccount.body_value()?;
        Ok(Self {
            ticker: ticker.into(),
            side,
            count,
            price,
            time_in_force,
            self_trade_prevention_type,
            client_order_id: client_order_id.into(),
            exchange_index,
            subaccount,
            post_only,
            reduce_only,
            cancel_order_on_pause,
        })
    }

    pub fn to_value(&self) -> Result<serde_json::Value, BuildError> {
        Ok(serde_json::to_value(self.to_generated()?).expect("create order request is json"))
    }

    pub(crate) fn to_generated(&self) -> Result<CreateOrderV2Request, BuildError> {
        let mut request = CreateOrderV2Request::new(
            self.ticker.clone(),
            self.side,
            self.count.wire(),
            decimal_wire(self.price, 4)?,
            self.time_in_force,
            self.self_trade_prevention_type,
        );
        request.client_order_id = Some(self.client_order_id.clone());
        request.exchange_index = self.exchange_index.as_option();
        request.subaccount = self.subaccount.body_value()?;
        request.post_only = self.post_only.wire();
        request.reduce_only = self.reduce_only.wire();
        request.cancel_order_on_pause = self.cancel_order_on_pause.wire();
        Ok(request)
    }
}
