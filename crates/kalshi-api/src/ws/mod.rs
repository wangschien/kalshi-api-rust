//! WebSocket client. Message types in `generated.rs` come from `spec/asyncapi.yaml`
//! through `tools/kalshi-sdk/ws_codegen.py`; the transport is hand-written.
//!
//! Ping and pong stay in the I/O task. A full application channel cannot delay a pong.
//! The first text frame that does not fit ends the stream with [`WsEvent::Lost`]
//! and closes the socket without waiting for another frame.
//! Reconnect, resubscribe, and recovery belong to the caller.
//! `last_ping_rx_ns` is the time a Ping frame was received.

mod generated;
mod transport;
mod types;

pub use generated::*;
pub use transport::{WsConfig, WsError, WsEvent, WsHost, WsSession};
pub use types::*;
