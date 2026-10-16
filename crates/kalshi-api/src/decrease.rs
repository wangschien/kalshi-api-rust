use kalshi_api_openapi::models::DecreaseOrderV2Request;

use crate::money::{FixedPointError, QtyCenti};
use crate::order::ExchangeIndexChoice;

/// Exactly one of `reduce_by` or `reduce_to`.
///
/// String representation of the number of contracts to reduce by. Exactly one of `reduce_by` or `reduce_to` must be provided.
///
/// String representation of the number of contracts to reduce to. Exactly one of `reduce_by` or `reduce_to` must be provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decrease {
    By(QtyCenti),
    To(QtyCenti),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecreaseOrderV2 {
    decrease: Decrease,
    exchange_index: ExchangeIndexChoice,
    market_ticker: Option<String>,
}

impl DecreaseOrderV2 {
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by market ticker.
    pub fn build(
        decrease: Decrease,
        exchange_index: ExchangeIndexChoice,
        market_ticker: Option<String>,
    ) -> Result<Self, FixedPointError> {
        let qty = match decrease {
            Decrease::By(qty) | Decrease::To(qty) => qty,
        };
        if qty.0 < 0 {
            return Err(FixedPointError { input: qty.wire() });
        }
        Ok(Self {
            decrease,
            exchange_index,
            market_ticker,
        })
    }

    pub fn to_value(&self) -> Result<serde_json::Value, FixedPointError> {
        Ok(serde_json::to_value(self.to_generated()).expect("decrease request is json"))
    }

    pub(crate) fn to_generated(&self) -> DecreaseOrderV2Request {
        let mut request = DecreaseOrderV2Request::new();
        match self.decrease {
            Decrease::By(qty) => request.reduce_by = Some(qty.wire()),
            Decrease::To(qty) => request.reduce_to = Some(qty.wire()),
        }
        request.exchange_index = self.exchange_index.as_option();
        request.market_ticker = self.market_ticker.clone();
        request
    }
}
