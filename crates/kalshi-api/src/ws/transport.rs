use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::{HeaderName, HeaderValue};
use tokio_tungstenite::tungstenite::Message;

use crate::sign::{Clock, SignMiddleware};
use crate::ws::{parse_incoming, ClientCommand, Incoming};

const SIGN_PATH: &str = "/trade-api/ws/v2";

/// Production is the default trading socket. Elections remains available.
/// Demo matches the REST demo host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsHost {
    Prod,
    Elections,
    Demo,
}

impl WsHost {
    pub fn url(self) -> &'static str {
        match self {
            Self::Prod => "wss://external-api-ws.kalshi.com/trade-api/ws/v2",
            Self::Elections => "wss://api.elections.kalshi.com/trade-api/ws/v2",
            Self::Demo => "wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2",
        }
    }

    pub fn sign_path() -> &'static str {
        SIGN_PATH
    }
}

/// Every field is required. The private key is not printed.
pub struct WsConfig {
    pub url: String,
    pub key_id: String,
    pub private_key_pem: String,
    pub clock: Arc<dyn Clock>,
    /// Local deadline for each command or control-frame write. `None` sets no
    /// deadline: each write is awaited until it completes or fails. The SDK
    /// has no default; a caller that sets `Some` should cite its source.
    pub send_timeout: Option<Duration>,
}

#[derive(Debug)]
pub enum WsError {
    Sign(String),
    Connect(String),
    Closed,
    ControlPayloadTooLarge { length: usize },
}

impl std::fmt::Display for WsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sign(detail) => write!(f, "websocket signing failed: {detail}"),
            Self::Connect(detail) => write!(f, "websocket connect failed: {detail}"),
            Self::Closed => write!(f, "websocket closed"),
            Self::ControlPayloadTooLarge { length } => {
                write!(f, "control payload has {length} bytes; maximum is 125")
            }
        }
    }
}

impl std::error::Error for WsError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WsEvent {
    Frame(Box<Incoming>),
    /// The socket is discarded because a write did not finish before the caller's
    /// `Some` deadline. Never produced when `send_timeout` is `None`.
    /// Delivered once after queued frames, then `recv` returns `None`.
    SendTimedOut {
        timeout: Duration,
    },
    /// The text did not match a typed message. `raw` is that text, unchanged.
    /// Identity is filled when the JSON has it. The socket stays open.
    Rejected {
        detail: String,
        raw: String,
        channel: Option<String>,
        sid: Option<u64>,
        seq: Option<u64>,
        order_id: Option<String>,
        client_order_id: Option<String>,
    },
}

/// Both directions use unbounded channels: no text frame is dropped or ends the
/// stream because the caller has not read it yet, and `send` never waits for
/// queue space. Memory grows with what the caller leaves unread or unsent.
pub struct WsSession {
    outbound: mpsc::UnboundedSender<Outbound>,
    inbound: mpsc::UnboundedReceiver<WsEvent>,
    lost_rx: oneshot::Receiver<WsEvent>,
    stashed_lost: Option<WsEvent>,
    lost_taken: bool,
    finished: bool,
    last_ping_rx_ns: Arc<AtomicU64>,
    task: tokio::task::JoinHandle<()>,
}

enum Outbound {
    Command(Box<ClientCommand>),
    Ping(Vec<u8>),
}

impl Drop for WsSession {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl WsSession {
    pub async fn connect(
        host: WsHost,
        key_id: impl Into<String>,
        private_key_pem: impl Into<String>,
        clock: Arc<dyn Clock>,
        send_timeout: Option<Duration>,
    ) -> Result<Self, WsError> {
        Self::connect_with(WsConfig {
            url: host.url().to_owned(),
            key_id: key_id.into(),
            private_key_pem: private_key_pem.into(),
            clock,
            send_timeout,
        })
        .await
    }

    pub async fn connect_with(config: WsConfig) -> Result<Self, WsError> {
        let signer = SignMiddleware::new(
            &config.key_id,
            &config.private_key_pem,
            Arc::clone(&config.clock),
        )
        .map_err(|err| WsError::Sign(err.to_string()))?;
        let path = sign_path(&config.url)?;
        let (key_id, timestamp, signature) = signer.headers("GET", &path);
        let mut request = config
            .url
            .into_client_request()
            .map_err(|err| WsError::Connect(err.to_string()))?;
        insert_header(request.headers_mut(), "kalshi-access-key", &key_id)?;
        insert_header(request.headers_mut(), "kalshi-access-timestamp", &timestamp)?;
        insert_header(request.headers_mut(), "kalshi-access-signature", &signature)?;
        let (socket, _) = tokio_tungstenite::connect_async(request)
            .await
            .map_err(|err| WsError::Connect(err.to_string()))?;
        let (write, read) = socket.split();
        let (outbound, command_rx) = mpsc::unbounded_channel();
        let (event_tx, inbound) = mpsc::unbounded_channel();
        let (lost_tx, lost_rx) = oneshot::channel();
        let last_ping_rx_ns = Arc::new(AtomicU64::new(0));
        let task = tokio::spawn(run_io(
            read,
            write,
            command_rx,
            event_tx,
            lost_tx,
            IoShared {
                last_ping_rx_ns: Arc::clone(&last_ping_rx_ns),
                clock: config.clock,
                send_timeout: config.send_timeout,
            },
        ));
        Ok(Self {
            outbound,
            inbound,
            lost_rx,
            stashed_lost: None,
            lost_taken: false,
            finished: false,
            last_ping_rx_ns,
            task,
        })
    }

    pub async fn send(&self, command: ClientCommand) -> Result<(), WsError> {
        self.outbound
            .send(Outbound::Command(Box::new(command)))
            .map_err(|_| WsError::Closed)
    }

    /// Sends a WebSocket control Ping. Incoming Ping frames are answered automatically.
    pub async fn ping(&self, payload: Vec<u8>) -> Result<(), WsError> {
        if payload.len() > 125 {
            return Err(WsError::ControlPayloadTooLarge {
                length: payload.len(),
            });
        }
        self.outbound
            .send(Outbound::Ping(payload))
            .map_err(|_| WsError::Closed)
    }

    pub async fn recv(&mut self) -> Option<WsEvent> {
        if self.finished {
            return None;
        }
        loop {
            match self.inbound.try_recv() {
                Ok(event) => return Some(event),
                Err(mpsc::error::TryRecvError::Disconnected) => return self.pull_lost().await,
                Err(mpsc::error::TryRecvError::Empty) => {
                    if let Some(event) = self.stashed_lost.take() {
                        self.finished = true;
                        return Some(event);
                    }
                }
            }
            if self.lost_taken {
                return self.inbound.recv().await;
            }
            let lost = {
                let rx = &mut self.lost_rx;
                tokio::select! {
                    biased;
                    event = self.inbound.recv() => {
                        return match event {
                            Some(event) => Some(event),
                            None => self.pull_lost().await,
                        };
                    }
                    result = rx => result,
                }
            };
            self.lost_taken = true;
            let Ok(lost) = lost else {
                continue;
            };
            match self.inbound.try_recv() {
                Ok(event) => {
                    self.stashed_lost = Some(lost);
                    return Some(event);
                }
                Err(mpsc::error::TryRecvError::Empty | mpsc::error::TryRecvError::Disconnected) => {
                    self.finished = true;
                    return Some(lost);
                }
            }
        }
    }

    async fn pull_lost(&mut self) -> Option<WsEvent> {
        if let Some(event) = self.stashed_lost.take() {
            self.finished = true;
            return Some(event);
        }
        if !self.lost_taken {
            self.lost_taken = true;
            if let Ok(event) = (&mut self.lost_rx).await {
                self.finished = true;
                return Some(event);
            }
        }
        self.finished = true;
        None
    }

    /// Nanoseconds since the Unix epoch when the latest Ping frame was received.
    pub fn last_ping_rx_ns(&self) -> u64 {
        self.last_ping_rx_ns.load(Ordering::Relaxed)
    }
}

fn sign_path(url: &str) -> Result<String, WsError> {
    let uri: tokio_tungstenite::tungstenite::http::Uri = url
        .parse()
        .map_err(|err| WsError::Connect(format!("url: {err}")))?;
    let path = uri.path();
    if path.is_empty() {
        Ok("/".to_owned())
    } else {
        Ok(path.to_owned())
    }
}

fn insert_header(
    headers: &mut tokio_tungstenite::tungstenite::http::HeaderMap,
    name: &'static str,
    value: &str,
) -> Result<(), WsError> {
    let name = HeaderName::from_static(name);
    let value = HeaderValue::from_str(value).map_err(|err| WsError::Sign(err.to_string()))?;
    headers.insert(name, value);
    Ok(())
}

struct IoShared {
    last_ping_rx_ns: Arc<AtomicU64>,
    clock: Arc<dyn Clock>,
    send_timeout: Option<Duration>,
}

async fn run_io<R, W>(
    mut read: R,
    mut write: W,
    mut commands: mpsc::UnboundedReceiver<Outbound>,
    incoming: mpsc::UnboundedSender<WsEvent>,
    lost: oneshot::Sender<WsEvent>,
    shared: IoShared,
) where
    R: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
    W: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let mut app = AppQueue {
        tx: Some(incoming),
        lost: Some(lost),
        stopping: false,
    };
    loop {
        tokio::select! {
            biased;
            frame = read.next() => {
                let Some(frame) = frame else { break };
                let Ok(frame) = frame else { break };
                match handle_frame(&mut write, frame, &mut app, &shared.last_ping_rx_ns, &shared.clock, shared.send_timeout).await {
                    Ok(true) => {},
                    Ok(false) => break,
                    Err(timeout) => {
                        send_timed_out(&mut app, timeout);
                        return;
                    },
                }
            }
            command = commands.recv(), if !app.stopping => {
                let Some(command) = command else { break };
                let message = match command {
                    Outbound::Command(command) => Message::Text(json_text(&command).into()),
                    Outbound::Ping(payload) => Message::Ping(payload.into()),
                };
                match send_frame(&mut write, message, shared.send_timeout).await {
                    Ok(true) => {},
                    Ok(false) => break,
                    Err(timeout) => {
                        send_timed_out(&mut app, timeout);
                        return;
                    },
                }
            }
        }
    }
    if let Err(timeout) = send_frame(&mut write, Message::Close(None), shared.send_timeout).await {
        send_timed_out(&mut app, timeout);
    }
}

/// `Ok(false)` means the write failed. `Err` carries the deadline that expired.
async fn send_frame<W>(
    write: &mut W,
    message: Message,
    send_timeout: Option<Duration>,
) -> Result<bool, Duration>
where
    W: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    match send_timeout {
        None => Ok(write.send(message).await.is_ok()),
        Some(timeout) => tokio::time::timeout(timeout, write.send(message))
            .await
            .map(|result| result.is_ok())
            .map_err(|_| timeout),
    }
}

fn json_text(command: &impl Serialize) -> String {
    serde_json::to_string(command).unwrap_or_else(|_| "{}".to_owned())
}

struct AppQueue {
    tx: Option<mpsc::UnboundedSender<WsEvent>>,
    lost: Option<oneshot::Sender<WsEvent>>,
    stopping: bool,
}

fn send_timed_out(app: &mut AppQueue, timeout: Duration) {
    if let Some(tx) = app.lost.take() {
        let _ = tx.send(WsEvent::SendTimedOut { timeout });
    }
    app.tx.take();
    app.stopping = true;
}

async fn handle_frame<W>(
    write: &mut W,
    frame: Message,
    app: &mut AppQueue,
    last_ping_rx_ns: &AtomicU64,
    clock: &Arc<dyn Clock>,
    send_timeout: Option<Duration>,
) -> Result<bool, Duration>
where
    W: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    Ok(match frame {
        Message::Ping(payload) => {
            last_ping_rx_ns.store(clock.unix_ns(), Ordering::Relaxed);
            if !send_frame(write, Message::Pong(payload), send_timeout).await? {
                return Ok(false);
            }
            !app.stopping
        }
        Message::Pong(_) => true,
        Message::Text(text) => deliver_text(app, text.as_str()),
        Message::Binary(_) => true,
        Message::Close(_) => false,
        Message::Frame(_) => true,
    })
}

/// `false` means the session receiver is gone, so the socket closes.
fn deliver_text(app: &mut AppQueue, raw: &str) -> bool {
    if app.stopping {
        return false;
    }
    let Some(tx) = app.tx.as_ref() else {
        return false;
    };
    let event = match parse_incoming(raw) {
        Ok(frame) => WsEvent::Frame(Box::new(frame)),
        Err(err) => rejected(raw, err),
    };
    tx.send(event).is_ok()
}

fn rejected(raw: &str, err: serde_json::Error) -> WsEvent {
    let identity = frame_identity(raw);
    WsEvent::Rejected {
        detail: err.to_string(),
        raw: raw.to_owned(),
        channel: identity.channel,
        sid: identity.sid,
        seq: identity.seq,
        order_id: identity.order_id,
        client_order_id: identity.client_order_id,
    }
}

struct FrameIdentity {
    channel: Option<String>,
    sid: Option<u64>,
    seq: Option<u64>,
    order_id: Option<String>,
    client_order_id: Option<String>,
}

fn frame_identity(text: &str) -> FrameIdentity {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
        return FrameIdentity {
            channel: None,
            sid: None,
            seq: None,
            order_id: None,
            client_order_id: None,
        };
    };
    let msg = value.get("msg");
    FrameIdentity {
        channel: string_at(&value, "type")
            .or_else(|| msg.and_then(|item| string_at(item, "channel"))),
        sid: u64_at(&value, "sid").or_else(|| msg.and_then(|item| u64_at(item, "sid"))),
        seq: u64_at(&value, "seq"),
        order_id: msg
            .and_then(|item| string_at(item, "order_id"))
            .or_else(|| string_at(&value, "order_id")),
        client_order_id: msg
            .and_then(|item| string_at(item, "client_order_id"))
            .or_else(|| string_at(&value, "client_order_id")),
    }
}

fn string_at(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|item| item.as_str())
        .map(str::to_owned)
}

fn u64_at(value: &serde_json::Value, key: &str) -> Option<u64> {
    value.get(key).and_then(serde_json::Value::as_u64)
}

#[cfg(test)]
#[path = "transport_queues.rs"]
mod queues;

#[cfg(test)]
#[path = "transport_deadlines.rs"]
mod deadlines;
