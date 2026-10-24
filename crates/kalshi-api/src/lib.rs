#![doc = include_str!("../../../README.md")]
//! Hand-written client for the Kalshi Trade API.
//!
//! Generated models stay in `kalshi-api-openapi`. This crate signs requests,
//! captures response bytes before that decode, and builds requests without
//! `Default`.
#![forbid(unsafe_code)]

mod amend;
mod batch;
mod capture;
mod client;
mod decrease;
mod error;
mod filters;
mod money;
mod order;
mod sign;
mod subaccount;
pub mod ws;

pub use amend::{AmendCount, AmendOrderV2};
pub use batch::BatchCancelItem;
pub use client::{exchange_index_query, Client, ClientConfig, Environment};
pub use decrease::{Decrease, DecreaseOrderV2};
pub use error::{BodyErrorKind, TransportKind, VenueError};
pub use filters::{
    FillsFilter, MarketsFilter, OrderQueuePositionsFilter, OrdersFilter, PositionsFilter,
};
pub use money::{parse_decimal, FixedPointError, QtyCenti};
pub use order::{BuildError, CreateOrderV2, ExchangeIndexChoice, OrderFlag};
pub use sign::{Clock, SignMiddleware, SystemClock};
pub use subaccount::{Subaccount, SubaccountChoice, SubaccountError, SubaccountFilter};
pub use ws::{
    parse_incoming, BookSide, Channel, ClientCommand, Incoming, LastUpdateReason, MarketSide,
    OrderAction, SubscribeParams, SubscriptionAction, UnsubscribeParams, UpdateSubscriptionParams,
    UserOrderStatus, WsConfig, WsEvent, WsHost, WsSession,
};
