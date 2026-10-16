use kalshi_api_openapi::models::BatchCancelOrdersV2RequestOrdersInner;

use crate::order::ExchangeIndexChoice;
use crate::subaccount::{SubaccountChoice, SubaccountError};

/// One order in a batch cancel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchCancelItem {
    order_id: String,
    subaccount: SubaccountChoice,
    exchange_index: ExchangeIndexChoice,
    market_ticker: Option<String>,
}

impl BatchCancelItem {
    /// Optional subaccount number to use for this cancellation (0 for primary, 1-63 for subaccounts). Subaccount-restricted API keys must omit this field or pass their locked subaccount.
    ///
    /// Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker field is required for auto-routing.
    pub fn build(
        order_id: impl Into<String>,
        subaccount: SubaccountChoice,
        exchange_index: ExchangeIndexChoice,
        market_ticker: Option<String>,
    ) -> Result<Self, SubaccountError> {
        subaccount.body_value()?;
        Ok(Self {
            order_id: order_id.into(),
            subaccount,
            exchange_index,
            market_ticker,
        })
    }

    pub fn to_value(&self) -> Result<serde_json::Value, SubaccountError> {
        Ok(serde_json::to_value(self.to_generated()?).expect("batch cancel item is json"))
    }

    pub(crate) fn to_generated(
        &self,
    ) -> Result<BatchCancelOrdersV2RequestOrdersInner, SubaccountError> {
        let mut item = BatchCancelOrdersV2RequestOrdersInner::new(self.order_id.clone());
        item.subaccount = self.subaccount.body_value()?;
        item.exchange_index = self.exchange_index.as_option();
        item.market_ticker = self.market_ticker.clone();
        Ok(item)
    }
}
