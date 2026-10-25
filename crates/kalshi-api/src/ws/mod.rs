//! WebSocket client. Message types in `generated.rs` come from `spec/asyncapi.yaml`
//! through `tools/kalshi-sdk/ws_codegen.py`; the transport is hand-written.
//!
//! Ping and pong stay in the I/O task, so unread frames cannot delay a pong.
//! Incoming frames and outgoing commands use unbounded channels: a frame the
//! caller has not read yet is kept, never dropped, and `send` never waits for
//! queue space.
//! Reconnect, resubscribe, and recovery belong to the caller.
//! `last_ping_rx_ns` is the time a Ping frame was received.

mod generated;
mod transport;
mod types;

pub use generated::*;
pub use transport::{WsConfig, WsError, WsEvent, WsHost, WsSession};
pub use types::*;
