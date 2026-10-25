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

const INCOMING_CAPACITY: usize = 256;
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
    /// Depth of the application channel. A `try_send` that returns full ends
    /// the stream. [`WsEvent::Lost`] is signaled separately, so that full
    /// channel cannot block it.
    pub incoming_capacity: usize,
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
    /// A full application channel terminates the stream with [`WsEvent::Lost`]
    /// instead of delivering this event.
    Rejected {
        detail: String,
        raw: String,
        channel: Option<String>,
        sid: Option<u64>,
        seq: Option<u64>,
        order_id: Option<String>,
        client_order_id: Option<String>,
    },
    /// The application channel could not take a text frame. `recv` yields this
    /// once, then `None`. `dropped` is the count at that moment. The socket closes.
    /// Reconnect, resubscribe, and recovery belong to the caller.
    Lost {
        channel: Option<String>,
        sid: Option<u64>,
        dropped: u64,
    },
}

pub struct WsSession {
    outbound: mpsc::Sender<Outbound>,
    inbound: mpsc::Receiver<WsEvent>,
    lost_rx: oneshot::Receiver<WsEvent>,
    stashed_lost: Option<WsEvent>,
    lost_taken: bool,
    finished: bool,
    last_ping_rx_ns: Arc<AtomicU64>,
    dropped_frames: Arc<AtomicU64>,
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
            incoming_capacity: INCOMING_CAPACITY,
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
        let (outbound, command_rx) = mpsc::channel(32);
        let (event_tx, inbound) = mpsc::channel(config.incoming_capacity.max(1));
        let (lost_tx, lost_rx) = oneshot::channel();
        let last_ping_rx_ns = Arc::new(AtomicU64::new(0));
        let dropped_frames = Arc::new(AtomicU64::new(0));
        let task = tokio::spawn(run_io(
            read,
            write,
            command_rx,
            event_tx,
            lost_tx,
            IoShared {
                last_ping_rx_ns: Arc::clone(&last_ping_rx_ns),
                dropped_frames: Arc::clone(&dropped_frames),
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
            dropped_frames,
            task,
        })
    }

    pub async fn send(&self, command: ClientCommand) -> Result<(), WsError> {
        self.outbound
            .send(Outbound::Command(Box::new(command)))
            .await
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
            .await
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

    /// Text frames not delivered on [`WsSession::recv`].
    pub fn dropped_frames(&self) -> u64 {
        self.dropped_frames.load(Ordering::Relaxed)
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
    dropped_frames: Arc<AtomicU64>,
    clock: Arc<dyn Clock>,
    send_timeout: Option<Duration>,
}

async fn run_io<R, W>(
    mut read: R,
    mut write: W,
    mut commands: mpsc::Receiver<Outbound>,
    incoming: mpsc::Sender<WsEvent>,
    lost: oneshot::Sender<WsEvent>,
    shared: IoShared,
) where
    R: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
    W: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let mut app = AppQueue {
        tx: Some(incoming),
        lost: Some(lost),
        dropped_frames: shared.dropped_frames,
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
    tx: Option<mpsc::Sender<WsEvent>>,
    lost: Option<oneshot::Sender<WsEvent>>,
    dropped_frames: Arc<AtomicU64>,
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

fn deliver_text(app: &mut AppQueue, raw: &str) -> bool {
    if app.stopping {
        app.dropped_frames.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    let Some(tx) = app.tx.clone() else {
        app.dropped_frames.fetch_add(1, Ordering::Relaxed);
        return false;
    };
    let event = match parse_incoming(raw) {
        Ok(frame) => WsEvent::Frame(Box::new(frame)),
        Err(err) => rejected(raw, err),
    };
    match tx.try_send(event) {
        Ok(()) => true,
        Err(mpsc::error::TrySendError::Full(_)) => {
            terminate(app, raw);
            false
        }
        Err(mpsc::error::TrySendError::Closed(_)) => false,
    }
}

fn terminate(app: &mut AppQueue, raw: &str) {
    let dropped = app.dropped_frames.fetch_add(1, Ordering::Relaxed) + 1;
    let identity = frame_identity(raw);
    if let Some(tx) = app.lost.take() {
        let _ = tx.send(WsEvent::Lost {
            channel: identity.channel,
            sid: identity.sid,
            dropped,
        });
    }
    // Drop the data sender so recv ends after queued frames and Lost.
    // Returning false from the read loop closes the socket in this turn.
    app.tx.take();
    app.stopping = true;
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
mod queue_race {
    #![allow(clippy::disallowed_methods)]
    use super::*;
    use futures_util::FutureExt;
    use std::future::Future;
    use std::sync::Mutex;
    use std::task::{Context, Poll, Wake, Waker};

    struct ImmediateConsumer {
        session: Mutex<WsSession>,
        events: Mutex<Vec<WsEvent>>,
    }

    impl Wake for ImmediateConsumer {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            // try_send wakes the receiver before it returns. An executor may
            // run that receiver before the sender's next statement.
            let Ok(mut session) = self.session.lock() else {
                return;
            };
            let event = session.recv().now_or_never().flatten();
            drop(session);
            if let Some(event) = event {
                if let Ok(mut events) = self.events.lock() {
                    events.push(event);
                }
            }
        }
    }

    #[tokio::test]
    async fn consumed_frame_does_not_make_the_next_frame_lost() {
        let (tx, inbound) = mpsc::channel(1);
        let (outbound, _commands) = mpsc::channel(1);
        let (lost_tx, lost_rx) = oneshot::channel();
        let dropped_frames = Arc::new(AtomicU64::new(0));
        let consumer = Arc::new(ImmediateConsumer {
            session: Mutex::new(WsSession {
                outbound,
                inbound,
                lost_rx,
                stashed_lost: None,
                lost_taken: false,
                finished: false,
                last_ping_rx_ns: Arc::new(AtomicU64::new(0)),
                dropped_frames: Arc::clone(&dropped_frames),
                task: tokio::spawn(std::future::pending()),
            }),
            events: Mutex::new(Vec::new()),
        });
        let waker = Waker::from(Arc::clone(&consumer));
        {
            let mut session = consumer.session.lock().unwrap();
            let mut pending = Box::pin(session.recv());
            assert!(matches!(
                pending.as_mut().poll(&mut Context::from_waker(&waker)),
                Poll::Pending
            ));
        }
        let mut app = AppQueue {
            tx: Some(tx),
            lost: Some(lost_tx),
            dropped_frames: Arc::clone(&dropped_frames),
            stopping: false,
        };
        let raw = r#"{"type":"subscribed","id":1,"msg":{"channel":"fill","sid":1}}"#;
        assert!(deliver_text(&mut app, raw));
        assert_eq!(consumer.events.lock().unwrap().len(), 1);
        assert!(consumer.session.lock().unwrap().inbound.is_empty());
        assert!(deliver_text(&mut app, raw));
        assert_eq!(dropped_frames.load(Ordering::Relaxed), 0);
        let next = consumer
            .session
            .lock()
            .unwrap()
            .recv()
            .now_or_never()
            .flatten();
        assert!(
            matches!(next, Some(WsEvent::Frame(_))),
            "a frame consumed before the next send was treated as a full queue: {next:?}"
        );
    }
}

#[cfg(test)]
#[path = "transport_deadlines.rs"]
mod deadlines;
