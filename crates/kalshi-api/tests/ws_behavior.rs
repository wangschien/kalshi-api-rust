#![allow(clippy::disallowed_methods)]

use std::sync::Arc;
use std::time::Duration;

use base64::Engine;
use futures_util::{SinkExt, StreamExt};
use kalshi_api::{
    parse_incoming, Channel, ClientCommand, Clock, Incoming, MarketSide, SubscribeParams,
    SubscriptionAction, UnsubscribeParams, UpdateSubscriptionParams, UserOrderStatus, WsConfig,
    WsEvent, WsHost, WsSession,
};
use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use rsa::pss::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::RsaPrivateKey;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request};
use tokio_tungstenite::tungstenite::{Bytes, Message};
use tokio_tungstenite::{accept_async, accept_hdr_async};

struct FixedClock;

impl Clock for FixedClock {
    fn unix_ms(&self) -> u64 {
        1_700_000_000_000
    }

    fn unix_ns(&self) -> u64 {
        1_700_000_000_000_000_123
    }
}

fn key_pair() -> (String, VerifyingKey<Sha256>) {
    let mut rng = rand::rngs::OsRng;
    let private = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public = private.to_public_key();
    let pem = private.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
    (pem, VerifyingKey::<Sha256>::new(public))
}

fn header(req: &Request, name: &str) -> String {
    req.headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_owned()
}

#[tokio::test]
async fn client_control_ping_preserves_payload_and_bounds() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        let ping = tokio::time::timeout(Duration::from_secs(5), socket.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(ping, Message::Ping(Bytes::from_static(b"synthetic-ping")));
        socket
            .send(Message::Pong(Bytes::from_static(b"synthetic-ping")))
            .await
            .unwrap();
    });
    let session = WsSession::connect_with(WsConfig {
        send_timeout: None,
        url: format!("ws://{address}/trade-api/ws/v2"),
        key_id: "synthetic-key".to_owned(),
        private_key_pem: include_str!("fixtures/parity/ed25519_test_private.pem").to_owned(),
        clock: Arc::new(FixedClock),
    })
    .await
    .unwrap();
    assert!(matches!(
        session.ping(vec![0; 126]).await,
        Err(kalshi_api::ws::WsError::ControlPayloadTooLarge { length: 126 })
    ));
    session.ping(b"synthetic-ping".to_vec()).await.unwrap();
    peer.await.unwrap();
}

fn fixture(name: &str) -> serde_json::Value {
    let path = format!("{}/tests/fixtures/s3/{name}", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&text).unwrap()
}

#[test]
fn production_socket_is_the_default_host() {
    assert_eq!(
        WsHost::Prod.url(),
        "wss://external-api-ws.kalshi.com/trade-api/ws/v2"
    );
    assert_eq!(
        WsHost::Elections.url(),
        "wss://api.elections.kalshi.com/trade-api/ws/v2"
    );
    assert_eq!(
        WsHost::Demo.url(),
        "wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2"
    );
    assert_eq!(WsHost::sign_path(), "/trade-api/ws/v2");
}

fn orderbook_subscribe(id: u64, ticker: &str) -> ClientCommand {
    ClientCommand::Subscribe {
        id,
        params: SubscribeParams {
            channels: vec![Channel::OrderbookDelta],
            market_ticker: Some(ticker.to_owned()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            use_yes_price: None,
            user_filter: None,
            shard_factor: None,
            shard_key: None,
            index_ids: None,
            underlying_tickers: None,
        },
    }
}

#[test]
fn synthetic_frames_round_trip() {
    let subscribe = orderbook_subscribe(1, "SYNTH-1");
    assert_eq!(
        serde_json::to_value(&subscribe).unwrap(),
        fixture("subscribe.json")
    );
    let unsubscribe = ClientCommand::Unsubscribe {
        id: 3,
        params: UnsubscribeParams { sids: vec![1] },
    };
    assert_eq!(
        serde_json::to_value(&unsubscribe).unwrap(),
        fixture("unsubscribe.json")
    );
    let update = ClientCommand::UpdateSubscription {
        id: 2,
        params: UpdateSubscriptionParams {
            index_ids: None,
            underlying_tickers: None,
            action: SubscriptionAction::GetSnapshot,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["SYNTH-1".to_owned()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
        },
    };
    assert_eq!(
        serde_json::to_value(&update).unwrap(),
        fixture("update_snapshot.json")
    );
    let index_list = ClientCommand::UpdateSubscription {
        id: 2,
        params: UpdateSubscriptionParams {
            index_ids: None,
            underlying_tickers: None,
            action: SubscriptionAction::IndexList,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
        },
    };
    let frame = serde_json::to_value(&index_list).unwrap();
    assert_eq!(
        frame,
        serde_json::json!({
            "id": 2,
            "cmd": "update_subscription",
            "params": {"sid": 1, "action": "indexlist"}
        })
    );
    assert_eq!(
        serde_json::from_str::<SubscriptionAction>("\"indexlist\"").unwrap(),
        SubscriptionAction::IndexList
    );

    for name in [
        "subscribed.json",
        "unsubscribed.json",
        "ok.json",
        "error.json",
        "orderbook_snapshot.json",
        "orderbook_delta.json",
        "trade.json",
        "fill.json",
        "market_lifecycle_v2.json",
        "market_lifecycle_full.json",
        "market_position.json",
        "user_order.json",
        "user_order_unknown_status.json",
        "user_order_null_created.json",
        "error_context.json",
        "orderbook_delta_ts.json",
    ] {
        let value = fixture(name);
        let text = serde_json::to_string(&value).unwrap();
        let parsed = parse_incoming(&text).unwrap_or_else(|err| panic!("{name}: {err}"));
        assert_eq!(serde_json::to_value(&parsed).unwrap(), value, "{name}");
    }
    match parse_incoming(&serde_json::to_string(&fixture("orderbook_delta.json")).unwrap()).unwrap()
    {
        Incoming::OrderbookDelta { msg, .. } => {
            assert_eq!(msg.side, MarketSide::Yes);
            assert_eq!(msg.client_order_id.as_deref(), Some("cid-synthetic"));
            assert_eq!(msg.subaccount, Some(1));
        }
        other => panic!("unexpected {other:?}"),
    }
    match parse_incoming(
        &serde_json::to_string(&fixture("user_order_unknown_status.json")).unwrap(),
    )
    .unwrap()
    {
        Incoming::UserOrder { msg, .. } => assert_eq!(msg.status, UserOrderStatus::Unknown),
        other => panic!("unexpected {other:?}"),
    }
    match parse_incoming(&serde_json::to_string(&fixture("market_position.json")).unwrap()).unwrap()
    {
        Incoming::MarketPosition { .. } => {}
        other => panic!("market position wire type was not accepted: {other:?}"),
    }
}

#[tokio::test]
async fn handshake_signs_the_path_and_pong_is_not_behind_the_channel() {
    let (pem, verifying) = key_pair();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let seen = Arc::new(std::sync::Mutex::new(String::new()));
    let seen_task = Arc::clone(&seen);
    let (text_tx, mut text_rx) = mpsc::channel(1);
    let (pong_tx, mut pong_rx) = mpsc::channel(1);
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_hdr_async(stream, move |req: &Request, response| {
            let target = req
                .uri()
                .path_and_query()
                .map(|item| item.as_str().to_owned())
                .unwrap_or_default();
            *seen_task.lock().unwrap() = format!(
                "{target}\n{}\n{}\n{}",
                header(req, "kalshi-access-key"),
                header(req, "kalshi-access-timestamp"),
                header(req, "kalshi-access-signature")
            );
            Ok::<_, ErrorResponse>(response)
        })
        .await
        .unwrap();
        let text = match socket.next().await {
            Some(Ok(Message::Text(text))) => text.to_string(),
            other => panic!("expected subscribe text, got {other:?}"),
        };
        text_tx.send(text).await.unwrap();
        let body = r#"{"type":"subscribed","id":1,"msg":{"channel":"orderbook_delta","sid":1}}"#;
        socket.send(Message::Text(body.into())).await.unwrap();
        socket
            .send(Message::Ping(Bytes::from_static(b"heartbeat")))
            .await
            .unwrap();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
        let mut pong = None;
        while pong.is_none() {
            match tokio::time::timeout_at(deadline, socket.next()).await {
                Ok(Some(Ok(Message::Pong(payload)))) => pong = Some(payload.to_vec()),
                Ok(Some(Ok(_))) => {}
                other => {
                    panic!("pong did not arrive while a frame was unread: {other:?}")
                }
            }
        }
        socket.send(Message::Text(body.into())).await.unwrap();
        pong_tx.send(pong.expect("pong")).await.unwrap();
        let _ = socket.next().await;
    });

    let clock = Arc::new(FixedClock);
    let mut session = WsSession::connect_with(WsConfig {
        send_timeout: None,
        url: format!("ws://{addr}/trade-api/ws/v2?ticker=SYNTH-1"),
        key_id: "key-synthetic".to_owned(),
        private_key_pem: pem,
        clock,
    })
    .await
    .unwrap();
    let captured = seen.lock().unwrap().clone();
    let mut lines = captured.lines();
    let target = lines.next().unwrap();
    let key = lines.next().unwrap();
    let timestamp = lines.next().unwrap();
    let signature = lines.next().unwrap();
    assert_eq!(target, "/trade-api/ws/v2?ticker=SYNTH-1");
    assert_eq!(key, "key-synthetic");
    let message = format!("{timestamp}GET/trade-api/ws/v2");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(signature)
        .unwrap();
    let signature = Signature::try_from(bytes.as_slice()).unwrap();
    verifying.verify(message.as_bytes(), &signature).unwrap();
    assert!(
        verifying
            .verify(
                format!("{timestamp}GET/trade-api/ws/v2?ticker=SYNTH-1").as_bytes(),
                &signature,
            )
            .is_err(),
        "a signature over the path plus the query was accepted"
    );

    session
        .send(orderbook_subscribe(1, "SYNTH-1"))
        .await
        .unwrap();
    // The test bounds its own wait so a write that never happens fails instead of hanging.
    let sent = tokio::time::timeout(Duration::from_secs(5), text_rx.recv())
        .await
        .expect("subscribe text not seen within the test's own bound")
        .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&sent).unwrap(),
        fixture("subscribe.json")
    );

    let pong = tokio::time::timeout(Duration::from_secs(2), pong_rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(pong, b"heartbeat");
    assert_eq!(session.last_ping_rx_ns(), FixedClock.unix_ns());
    for _ in 0..2 {
        assert!(matches!(
            recv_event(&mut session).await,
            Some(WsEvent::Frame(frame)) if matches!(frame.as_ref(), Incoming::Subscribed { .. })
        ));
    }
}

#[test]
fn orderbook_subscribe_keeps_use_yes_price() {
    let mut command = orderbook_subscribe(11, "SYNTH-1");
    let ClientCommand::Subscribe { params, .. } = &mut command else {
        panic!("subscribe");
    };
    params.use_yes_price = Some(true);
    assert_eq!(
        serde_json::to_value(&command).unwrap(),
        fixture("subscribe_yes_price.json")
    );
}

#[test]
fn lifecycle_close_update_keeps_the_new_close_time() {
    let value = serde_json::json!({
        "type": "market_lifecycle_v2",
        "sid": 1,
        "seq": 2,
        "msg": {
            "market_ticker": "SYNTH-1",
            "event_type": "close_date_updated",
            "close_ts": 1700000100
        }
    });
    let frame = parse_incoming(&value.to_string()).unwrap();
    let roundtrip = serde_json::to_value(&frame).unwrap();
    assert_eq!(roundtrip["msg"]["close_ts"], value["msg"]["close_ts"]);
}

#[test]
#[allow(deprecated)]
fn user_order_keeps_order_update_timestamp_and_null_created_keys() {
    let mut value = fixture("user_order.json");
    value["msg"]["last_updated_ts_ms"] = serde_json::json!(1_700_000_100_123_i64);
    let frame = parse_incoming(&value.to_string()).unwrap();
    let roundtrip = serde_json::to_value(&frame).unwrap();
    assert_eq!(
        roundtrip["msg"]["last_updated_ts_ms"],
        value["msg"]["last_updated_ts_ms"]
    );

    let nulls = fixture("user_order_null_created.json");
    match parse_incoming(&nulls.to_string()).unwrap() {
        Incoming::UserOrder { msg, .. } => {
            assert_eq!(msg.created_time, None);
            assert_eq!(msg.created_ts_ms, None);
            assert_eq!(msg.last_updated_ts_ms, Some(1_700_000_100_123));
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn required_nullable_user_order_keys_must_be_present() {
    for key in ["created_time", "created_ts_ms"] {
        let mut value = fixture("user_order.json");
        value["msg"].as_object_mut().unwrap().remove(key);
        assert!(
            parse_incoming(&value.to_string()).is_err(),
            "{key} was treated as optional"
        );
    }
}

#[test]
fn ws_module_exports_the_error_type() {
    let err = kalshi_api::ws::WsError::Closed;
    assert!(err.to_string().contains("websocket"));
    assert_eq!(
        kalshi_api::ws::WsHost::Prod.url(),
        "wss://external-api-ws.kalshi.com/trade-api/ws/v2"
    );
}

#[test]
fn contributing_states_reconnect_is_the_callers_job() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../CONTRIBUTING.md");
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("`kalshi_api::ws`"));
    assert!(text.contains("unbounded"));
    assert!(text.contains("Reconnect, resubscribe, and recovery are the caller's responsibility."));
}

async fn connect(addr: &str) -> WsSession {
    let (pem, _) = key_pair();
    WsSession::connect_with(WsConfig {
        send_timeout: None,
        url: format!("ws://{addr}/trade-api/ws/v2"),
        key_id: "key-synthetic".to_owned(),
        private_key_pem: pem,
        clock: Arc::new(FixedClock),
    })
    .await
    .unwrap()
}

async fn recv_event(session: &mut WsSession) -> Option<WsEvent> {
    tokio::time::timeout(Duration::from_secs(2), session.recv())
        .await
        .expect("recv timed out")
}

/// Far more than any small fixed queue depth.
const BURST: usize = 2048;

#[tokio::test]
async fn unread_burst_is_delivered_in_order_and_the_socket_stays_open() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let (alive_tx, alive_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        for seq in 1..=BURST {
            let mut frame = fixture("orderbook_delta.json");
            frame["seq"] = serde_json::json!(seq);
            socket
                .send(Message::Text(frame.to_string().into()))
                .await
                .unwrap();
        }
        socket
            .send(Message::Ping(Bytes::from_static(b"after-burst")))
            .await
            .unwrap();
        let alive = loop {
            match tokio::time::timeout(Duration::from_secs(5), socket.next()).await {
                Ok(Some(Ok(Message::Pong(payload)))) => break payload.as_ref() == b"after-burst",
                Ok(Some(Ok(Message::Close(_)) | Err(_)) | None) | Err(_) => break false,
                Ok(Some(Ok(_))) => {}
            }
        };
        let _ = alive_tx.send(alive);
        let _ = release_rx.await;
    });
    let mut session = connect(&addr).await;
    assert!(
        alive_rx.await.unwrap(),
        "the client closed the socket or stopped answering while frames were unread"
    );
    for seq in 1..=BURST as u64 {
        match recv_event(&mut session).await {
            Some(WsEvent::Frame(frame)) => match *frame {
                Incoming::OrderbookDelta { seq: got, .. } => assert_eq!(got, seq),
                other => panic!("unexpected frame {other:?}"),
            },
            other => panic!("frame {seq} of {BURST} was not delivered: {other:?}"),
        }
    }
    let _ = release_tx.send(());
    server.await.unwrap();
}

#[tokio::test]
async fn rejected_frame_keeps_raw_identity_and_continues() {
    let marker = "ord-audit-raw-must-survive";
    let mut invalid = fixture("user_order.json");
    invalid["msg"]["status"] = serde_json::json!("new-unknown-status");
    invalid["msg"]["order_id"] = serde_json::json!(marker);
    let raw = invalid.to_string();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let fill = fixture("fill.json").to_string();
    let sent = raw.clone();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        socket.send(Message::Text(sent.into())).await.unwrap();
        socket.send(Message::Text(fill.into())).await.unwrap();
        let _ = release_rx.await;
    });
    let mut session = connect(&addr).await;
    match recv_event(&mut session).await {
        Some(WsEvent::Rejected {
            raw: got,
            channel,
            sid,
            order_id,
            client_order_id,
            ..
        }) => {
            assert_eq!(got, raw);
            assert_eq!(channel.as_deref(), Some("user_order"));
            assert_eq!(sid, Some(1));
            assert_eq!(order_id.as_deref(), Some(marker));
            assert_eq!(client_order_id.as_deref(), Some("cid-synthetic"));
        }
        other => panic!("expected Rejected, got {other:?}"),
    }
    assert!(matches!(
        recv_event(&mut session).await,
        Some(WsEvent::Frame(frame)) if matches!(frame.as_ref(), Incoming::Fill { .. })
    ));
    let _ = release_tx.send(());
    server.await.unwrap();
}
