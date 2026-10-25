#![allow(clippy::disallowed_methods)]
#![allow(deprecated)]
//! `None` deadlines and limits are unbounded; `Some` ones are applied.
//! Paused-clock tests let the server wait 30 virtual days, so any finite
//! local deadline would fire first.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use kalshi_api::{Client, ClientConfig, Clock, SignMiddleware, TransportKind, VenueError};
use kalshi_api_openapi::apis::{configuration::Configuration, exchange_api};
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, Error, Middleware, Next, Result};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const FOREVER: Duration = Duration::from_secs(30 * 24 * 3600);

fn pem() -> String {
    include_str!("fixtures/parity/rsa_test_private.pem").to_owned()
}

fn cfg(
    base_url: String,
    connect: Option<Duration>,
    request: Option<Duration>,
    body: Option<usize>,
) -> ClientConfig {
    ClientConfig {
        base_url,
        key_id: "key-synthetic".to_owned(),
        private_key_pem: pem(),
        connect_timeout: connect,
        request_timeout: request,
        max_body_bytes: body,
        max_capture_bytes: 16,
    }
}

fn order_body(pad: usize) -> Vec<u8> {
    let ticker = "S".repeat(pad);
    format!(
        r#"{{"order":{{"order_id":"ord-synthetic-1","user_id":"user-synthetic","client_order_id":"cid-synthetic","ticker":"{ticker}","outcome_side":"yes","book_side":"bid","type":"limit","status":"resting","yes_price_dollars":"0.5600","no_price_dollars":"0.4400","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.0000","maker_fill_cost_dollars":"0.0000","taker_fees_dollars":"0.0000","maker_fees_dollars":"0.0000"}}}}"#
    )
    .into_bytes()
}

/// Reads one request head, waits `delay` (virtual under paused time), then answers.
async fn serve_after(delay: Duration, body: Vec<u8>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let Ok((mut socket, _)) = listener.accept().await else {
            return;
        };
        let mut request = Vec::new();
        let mut byte = [0];
        while !request.ends_with(b"\r\n\r\n") {
            if socket.read_exact(&mut byte).await.is_err() {
                return;
            }
            request.push(byte[0]);
        }
        tokio::time::sleep(delay).await;
        let head = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
            body.len()
        );
        let _ = socket.write_all(head.as_bytes()).await;
        let _ = socket.write_all(&body).await;
    });
    format!("http://{addr}")
}

/// A loopback listener whose accept queue is full and never drained. The
/// kernel drops further SYNs, so a new connect stays pending until a local
/// deadline ends it. No route or firewall setup is involved.
///
/// Only Linux is known to drop the SYN here. Other kernels may refuse or
/// accept the connect instead, which would fail these tests for a reason
/// unrelated to the client, so this helper and the connect tests that use it
/// are compiled on Linux only.
#[cfg(target_os = "linux")]
struct Unanswered {
    url: String,
    _listener: TcpListener,
    _held: Vec<std::net::TcpStream>,
}

#[cfg(target_os = "linux")]
fn unanswered() -> Unanswered {
    let socket = tokio::net::TcpSocket::new_v4().unwrap();
    socket.bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let listener = socket.listen(0).unwrap();
    let addr = listener.local_addr().unwrap();
    let mut held = Vec::new();
    loop {
        assert!(held.len() < 64, "accept queue never filled");
        match std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
            Ok(stream) => held.push(stream),
            Err(err) if err.kind() == std::io::ErrorKind::TimedOut => break,
            Err(err) => panic!("{err}"),
        }
    }
    Unanswered {
        url: format!("http://{addr}"),
        _listener: listener,
        _held: held,
    }
}

// ---------- Client layer: slow reply ----------

#[tokio::test(start_paused = true)]
async fn client_none_has_no_request_deadline_of_any_length() {
    let base = serve_after(FOREVER, order_body(1)).await;
    let client = Client::new(cfg(base, None, None, None)).unwrap();
    let order = client.get_order("abc").await.unwrap();
    assert_eq!(order.order.order_id, "ord-synthetic-1");
}

#[tokio::test(start_paused = true)]
async fn client_some_request_deadline_still_times_out() {
    let base = serve_after(FOREVER, order_body(1)).await;
    let client = Client::new(cfg(base, None, Some(Duration::from_secs(30)), None)).unwrap();
    let err = client.get_order("abc").await.unwrap_err();
    assert!(
        matches!(
            err,
            VenueError::Transport {
                kind: TransportKind::Timeout
            }
        ),
        "{err:?}"
    );
}

// ---------- Client layer: connect ----------

#[cfg(target_os = "linux")]
#[tokio::test(start_paused = true)]
async fn client_none_has_no_connect_deadline_of_any_length() {
    let target = unanswered();
    let client = Client::new(cfg(target.url.clone(), None, None, None)).unwrap();
    let outcome = tokio::time::timeout(Duration::from_secs(3600), client.get_order("abc")).await;
    assert!(
        outcome.is_err(),
        "a local deadline ended the connect: {outcome:?}"
    );
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn client_some_connect_deadline_still_fires() {
    let target = unanswered();
    let client = Client::new(cfg(
        target.url.clone(),
        Some(Duration::from_millis(200)),
        None,
        None,
    ))
    .unwrap();
    let outcome = tokio::time::timeout(Duration::from_secs(10), client.get_order("abc")).await;
    let result = outcome.expect("Some(connect_timeout) did not end the connect within 10s");
    assert!(result.is_err(), "{result:?}");
}

// ---------- Client layer: large body ----------

#[tokio::test]
async fn client_none_reads_a_24_mib_body() {
    let body = order_body(24 << 20);
    let base = serve_after(Duration::ZERO, body).await;
    let client = Client::new(cfg(base, None, None, None)).unwrap();
    let order = client.get_order("abc").await.unwrap();
    assert_eq!(order.order.ticker.len(), 24 << 20);
}

#[tokio::test]
async fn client_some_body_limit_still_rejects() {
    let body = order_body(24 << 20);
    let base = serve_after(Duration::ZERO, body).await;
    let client = Client::new(cfg(base, None, None, Some(1 << 20))).unwrap();
    let err = client.get_order("abc").await.unwrap_err();
    assert!(matches!(err, VenueError::BodyTooLarge { .. }), "{err:?}");
}

#[test]
fn capture_not_checked_against_unset_limit() {
    let mut c = cfg("http://127.0.0.1:1".to_owned(), None, None, None);
    c.max_capture_bytes = usize::MAX;
    assert!(Client::new(c).is_ok());
    let mut c = cfg("http://127.0.0.1:1".to_owned(), None, None, Some(8));
    c.max_capture_bytes = 9;
    assert!(Client::new(c).is_err());
}

// ---------- Generated layer: slow reply ----------

#[tokio::test(start_paused = true)]
async fn generated_none_has_no_deadline_of_any_length() {
    let base = serve_after(FOREVER, b"{}".to_vec()).await;
    let mut config = Configuration::new(None, None).unwrap();
    config.base_path = base;
    let response = exchange_api::get_exchange_status_without_preload_content(&config)
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test(start_paused = true)]
async fn generated_none_with_replaced_client_has_no_deadline() {
    let base = serve_after(FOREVER, b"{}".to_vec()).await;
    let mut config = Configuration::new(None, None).unwrap();
    config.base_path = base;
    config.client = ClientBuilder::new(reqwest::Client::new()).build();
    let response = exchange_api::get_exchange_status_without_preload_content(&config)
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test(start_paused = true)]
async fn generated_some_still_times_out_builder_and_per_request() {
    // constructor client
    let base = serve_after(FOREVER, b"{}".to_vec()).await;
    let mut config = Configuration::new(None, Some(Duration::from_secs(30))).unwrap();
    config.base_path = base;
    let err = exchange_api::get_exchange_status_without_preload_content(&config)
        .await
        .unwrap_err();
    assert!(
        format!("{err:?}").to_lowercase().contains("timed out")
            || format!("{err:?}").contains("TimedOut"),
        "{err:?}"
    );
    // replaced client: only the per-request deadline remains
    let base = serve_after(FOREVER, b"{}".to_vec()).await;
    let mut config = Configuration::new(None, Some(Duration::from_secs(30))).unwrap();
    config.base_path = base;
    config.client = ClientBuilder::new(reqwest::Client::new()).build();
    let err = exchange_api::get_exchange_status_without_preload_content(&config)
        .await
        .unwrap_err();
    assert!(
        format!("{err:?}").to_lowercase().contains("timed out")
            || format!("{err:?}").contains("TimedOut"),
        "{err:?}"
    );
}

// ---------- Generated layer: every operation, structural ----------

struct FixedClock;
impl Clock for FixedClock {
    fn unix_ms(&self) -> u64 {
        1_700_000_000_000
    }
}

struct Capture(Arc<Mutex<Option<Request>>>);

#[async_trait]
impl Middleware for Capture {
    async fn handle(
        &self,
        request: Request,
        _: &mut http::Extensions,
        _: Next<'_>,
    ) -> Result<Response> {
        *self.0.lock().unwrap() = Some(request);
        Err(Error::middleware(std::io::Error::other(
            "offline request captured",
        )))
    }
}

async fn per_request_timeouts(request_timeout: Option<Duration>) -> (usize, usize) {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/parity/requests.json")).unwrap();
    let captured = Arc::new(Mutex::new(None));
    let sign = SignMiddleware::new(
        "synthetic-key",
        include_str!("fixtures/parity/rsa_test_private.pem"),
        Arc::new(FixedClock),
    )
    .unwrap();
    let configuration = Configuration {
        base_path: "https://example.invalid/trade-api/v2".to_owned(),
        user_agent: Some("parity-fixture".to_owned()),
        client: ClientBuilder::new(reqwest::Client::new())
            .with(sign)
            .with(Capture(Arc::clone(&captured)))
            .build(),
        ..Configuration::new(None, request_timeout).unwrap()
    };
    let mut matched = 0;
    let mut total = 0;
    for fixture in &fixtures {
        let operation = fixture["operation"].as_str().unwrap();
        include!("fixtures/parity/request_dispatch.rs");
        let request = captured.lock().unwrap().take().unwrap();
        total += 1;
        if request.timeout() == request_timeout.as_ref() {
            matched += 1;
        }
    }
    (matched, total)
}

#[tokio::test]
async fn generated_every_operation_none_sets_no_request_timeout() {
    let (matched, total) = per_request_timeouts(None).await;
    println!("AUDIT_NONE matched={matched} total={total}");
    assert_eq!(total, 708);
    assert_eq!(matched, total);
}

#[tokio::test]
async fn generated_every_operation_some_sets_request_timeout() {
    let (matched, total) = per_request_timeouts(Some(Duration::from_secs(30))).await;
    println!("AUDIT_SOME matched={matched} total={total}");
    assert_eq!(total, 708);
    assert_eq!(matched, total);
}

// ---------- Generated layer: connect ----------

#[cfg(target_os = "linux")]
#[tokio::test]
async fn generated_some_connect_deadline_still_fires() {
    let mut config = Configuration::new(Some(Duration::from_millis(200)), None).unwrap();
    let target = unanswered();
    config.base_path = target.url.clone();
    let outcome = tokio::time::timeout(
        Duration::from_secs(10),
        exchange_api::get_exchange_status_without_preload_content(&config),
    )
    .await;
    let result = outcome.expect("Some(connect_timeout) did not end the connect within 10s");
    assert!(result.is_err());
}

#[cfg(target_os = "linux")]
#[tokio::test(start_paused = true)]
async fn generated_none_has_no_connect_deadline_of_any_length() {
    let mut config = Configuration::new(None, None).unwrap();
    let target = unanswered();
    config.base_path = target.url.clone();
    let outcome = tokio::time::timeout(
        Duration::from_secs(3600),
        exchange_api::get_exchange_status_without_preload_content(&config),
    )
    .await;
    assert!(outcome.is_err(), "a local deadline ended the connect");
}
