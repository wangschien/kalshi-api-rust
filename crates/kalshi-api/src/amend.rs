use kalshi_api_openapi::models::{AmendOrderV2Request, BookSide};
use rust_decimal::Decimal;

use crate::money::{decimal_wire, format_centi, FixedPointError, QtyCenti};
use crate::order::ExchangeIndexChoice;

/// Amend `count` is filled plus the desired remaining size.
/// The only constructor is [`AmendCount::from_filled_and_remaining`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendCount {
    wire: String,
}

impl AmendCount {
    pub fn from_filled_and_remaining(
        filled: QtyCenti,
        remaining: QtyCenti,
    ) -> Result<Self, FixedPointError> {
        let sum = filled
            .0
            .checked_add(remaining.0)
            .ok_or_else(|| FixedPointError {
                input: format!("{}+{}", filled.wire(), remaining.wire()),
            })?;
        Ok(Self {
            wire: format_centi(sum),
        })
    }

    pub fn as_wire(&self) -> &str {
        &self.wire
    }
}

/// Typed amend body. `count` is an [`AmendCount`], so the wire value is filled plus remaining.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendOrderV2 {
    ticker: String,
    side: BookSide,
    price: Decimal,
    count: AmendCount,
    client_order_id: String,
    updated_client_order_id: Option<String>,
    expiration_time: Option<i64>,
    exchange_index: ExchangeIndexChoice,
}

impl AmendOrderV2 {
    /// Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker.
    ///
    /// Updated total/max fillable count for the order. Set this to the order's already filled count plus the desired resting remaining count after the amend.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        ticker: impl Into<String>,
        side: BookSide,
        price: Decimal,
        count: AmendCount,
        client_order_id: impl Into<String>,
        updated_client_order_id: Option<String>,
        exchange_index: ExchangeIndexChoice,
    ) -> Result<Self, FixedPointError> {
        let _price_wire = decimal_wire(price, 4)?;
        Ok(Self {
            ticker: ticker.into(),
            side,
            price,
            count,
            client_order_id: client_order_id.into(),
            updated_client_order_id,
            expiration_time: None,
            exchange_index,
        })
    }

    /// Zero removes expiry. Omitting this option preserves the existing expiry.
    pub fn with_expiration_time(mut self, expiration_time: i64) -> Self {
        self.expiration_time = Some(expiration_time);
        self
    }

    pub fn to_value(&self) -> Result<serde_json::Value, FixedPointError> {
        Ok(serde_json::to_value(self.to_generated()?).expect("amend request is json"))
    }

    pub(crate) fn to_generated(&self) -> Result<AmendOrderV2Request, FixedPointError> {
        let mut request = AmendOrderV2Request::new(
            self.ticker.clone(),
            self.side,
            decimal_wire(self.price, 4)?,
            self.count.as_wire().to_owned(),
        );
        request.client_order_id = Some(self.client_order_id.clone());
        request.updated_client_order_id = self.updated_client_order_id.clone();
        request.expiration_time = self.expiration_time;
        request.exchange_index = self.exchange_index.as_option();
        Ok(request)
    }
}
