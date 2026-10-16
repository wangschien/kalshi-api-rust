//! Hand-written WebSocket client. The AsyncAPI document is not used to generate code.
//!
//! Ping and pong stay in the I/O task. A full application channel cannot delay a pong.
//! The first text frame that does not fit ends the stream with [`WsEvent::Lost`]
//! and closes the socket without waiting for another frame.
//! Reconnect, resubscribe, and recovery belong to the caller.
//! `last_ping_rx_ns` is the time a Ping frame was received.

mod payloads;
mod transport;
mod types;

pub use payloads::*;
pub use transport::{WsConfig, WsError, WsEvent, WsHost, WsSession};
pub use types::*;
