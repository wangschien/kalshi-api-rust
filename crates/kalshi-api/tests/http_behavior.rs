#![allow(clippy::disallowed_methods)]

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use base64::Engine;
use kalshi_api::{
    AmendCount, AmendOrderV2, BatchCancelItem, Client, ClientConfig, CreateOrderV2, Decrease,
    DecreaseOrderV2, ExchangeIndexChoice, FillsFilter, OrderFlag, OrdersFilter, PositionsFilter,
    QtyCenti, Subaccount, SubaccountChoice, SubaccountFilter, VenueError,
};
use kalshi_api_openapi::models::create_order_v2_request::TimeInForce;
use kalshi_api_openapi::models::{BookSide, SelfTradePreventionType};
use rand::rngs::OsRng;
use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use rsa::pss::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::RsaPrivateKey;
use rust_decimal::Decimal;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn config(
    base_url: String,
    pem: &str,
    body: usize,
    capture: usize,
    request_timeout: Duration,
) -> ClientConfig {
    ClientConfig {
        base_url,
        key_id: "key-synthetic".to_owned(),
        private_key_pem: pem.to_owned(),
        connect_timeout: Duration::from_millis(200),
        request_timeout,
        max_body_bytes: body,
        max_capture_bytes: capture,
    }
}

fn key_pair() -> (String, VerifyingKey<Sha256>) {
    let mut rng = OsRng;
    let private = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public = private.to_public_key();
    let pem = private.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
    (pem, VerifyingKey::<Sha256>::new(public))
}

fn order_body(book_side: &str, pad: usize) -> String {
    let ticker = "S".repeat(pad);
    format!(
        r#"{{"order":{{"order_id":"ord-synthetic-1","user_id":"user-synthetic","client_order_id":"cid-synthetic","ticker":"{ticker}","outcome_side":"yes","book_side":"{book_side}","type":"limit","status":"resting","yes_price_dollars":"0.5600","no_price_dollars":"0.4400","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.0000","maker_fill_cost_dollars":"0.0000","taker_fees_dollars":"0.0000","maker_fees_dollars":"0.0000"}}}}"#
    )
}

async fn serve(status: &str, content_type: &str, body: &[u8], mode: Mode) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let status = status.to_owned();
    let content_type = content_type.to_owned();
    let body = body.to_vec();
    tokio::spawn(async move {
        let Ok((mut socket, _)) = listener.accept().await else {
            return;
        };
        let mut buf = vec![0u8; 8192];
        let _ = socket.read(&mut buf).await;
        match mode {
            Mode::Full => {
                let head = format!(
                    "HTTP/1.1 {status}\r\ncontent-type: {content_type}\r\ndate: Mon, 01 Jan 2024 00:00:00 GMT\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    body.len()
                );
                let _ = socket.write_all(head.as_bytes()).await;
                let _ = socket.write_all(&body).await;
            }
            Mode::HeadersThenDrop => {
                let head = format!("HTTP/1.1 {status}\r\ncontent-type: {content_type}\r\ncontent-length: 20\r\nconnection: close\r\n\r\n");
                let _ = socket.write_all(head.as_bytes()).await;
            }
            Mode::HeadersThenStall => {
                let head = format!("HTTP/1.1 {status}\r\ncontent-type: {content_type}\r\ncontent-length: 20\r\nconnection: close\r\n\r\n");
                let _ = socket.write_all(head.as_bytes()).await;
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    });
    format!("http://{addr}")
}

#[derive(Clone, Copy)]
enum Mode {
    Full,
    HeadersThenDrop,
    HeadersThenStall,
}

#[tokio::test]
async fn signing_uses_the_official_scheme_and_strips_the_query() {
    let (pem, verifying) = key_pair();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let seen = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let seen2 = Arc::clone(&seen);
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 8192];
        let n = socket.read(&mut buf).await.unwrap();
        seen2.lock().await.extend_from_slice(&buf[..n]);
        let body = order_body("bid", 1);
        let head = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = socket.write_all(head.as_bytes()).await;
    });
    let client = Client::new(config(
        format!("http://{addr}"),
        &pem,
        100_000,
        100_000,
        Duration::from_secs(2),
    ))
    .unwrap();
    assert!(client.api_key_disabled());
    let order = client.get_order("abc").await.unwrap();
    assert_eq!(order.order.order_id, "ord-synthetic-1");
    let raw = String::from_utf8_lossy(&seen.lock().await).to_string();
    let timestamp = header_value(&raw, "KALSHI-ACCESS-TIMESTAMP");
    let signature = header_value(&raw, "KALSHI-ACCESS-SIGNATURE");
    let key = header_value(&raw, "KALSHI-ACCESS-KEY");
    assert_eq!(key, "key-synthetic");
    assert!(raw.starts_with("GET /portfolio/orders/abc "));
    let path = "/portfolio/orders/abc";
    let message = format!("{timestamp}GET{path}");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(signature)
        .unwrap();
    let signature = Signature::try_from(bytes.as_slice()).unwrap();
    verifying.verify(message.as_bytes(), &signature).unwrap();
    let _ = client;
}

fn header_value(raw: &str, name: &str) -> String {
    raw.lines()
        .find_map(|line| {
            let (key, value) = line.split_once(':')?;
            key.eq_ignore_ascii_case(name)
                .then(|| value.trim().to_owned())
        })
        .unwrap_or_else(|| panic!("missing {name} in {raw}"))
}

#[tokio::test]
async fn capture_classifies_responses() {
    let (pem, _) = key_pair();
    let fixture = include_str!("fixtures/get_order.json");
    let base = serve("200 OK", "application/json", fixture.as_bytes(), Mode::Full).await;
    let client = Client::new(config(base, &pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    assert!(client.get_order("abc").await.is_ok());

    let bad = order_body("not-a-side", 400);
    let base = serve("200 OK", "application/json", bad.as_bytes(), Mode::Full).await;
    let client = Client::new(config(base, &pem, 20_000, 80, Duration::from_secs(2))).unwrap();
    let err = client.get_order("abc").await.unwrap_err();
    match err {
        VenueError::SchemaDrift {
            truncated, total, ..
        } => {
            assert!(truncated);
            assert!(total > 80);
        }
        other => panic!("expected schema drift, got {other:?}"),
    }
    assert_eq!(client.schema_drift_count(), 1);
    let base = serve("200 OK", "application/json", bad.as_bytes(), Mode::Full).await;
    let client = Client::new(config(base, &pem, 20_000, 80, Duration::from_secs(2))).unwrap();
    let raw = client.get_order_raw("abc").await.unwrap_err();
    assert!(matches!(raw, VenueError::TruncatedRaw { .. }), "{raw:?}");

    let base = serve("200 OK", "application/json", b"", Mode::Full).await;
    let client = Client::new(config(base, &pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    assert!(matches!(
        client.get_order("abc").await.unwrap_err(),
        VenueError::NonJson { .. }
    ));

    let base = serve(
        "502 Bad Gateway",
        "text/html",
        b"<html>down</html>",
        Mode::Full,
    )
    .await;
    let client = Client::new(config(base, &pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    match client.get_order("abc").await.unwrap_err() {
        VenueError::Http { status, .. } => assert_eq!(status, 502),
        other => panic!("expected http, got {other:?}"),
    }

    let base = serve(
        "429 Too Many Requests",
        "application/json",
        b"",
        Mode::HeadersThenDrop,
    )
    .await;
    let client = Client::new(config(base, &pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    let err = client.get_order("abc").await.unwrap_err();
    assert!(err.is_rate_limited(), "{err:?}");
    assert!(matches!(
        err,
        VenueError::HttpBodyFailed { status: 429, .. }
    ));

    let base = serve(
        "429 Too Many Requests",
        "application/json",
        b"",
        Mode::HeadersThenStall,
    )
    .await;
    let client = Client::new(config(
        base,
        &pem,
        100_000,
        100_000,
        Duration::from_millis(300),
    ))
    .unwrap();
    let err = client.get_order("abc").await.unwrap_err();
    assert!(err.is_rate_limited(), "{err:?}");
    assert!(
        matches!(err, VenueError::HttpBodyFailed { status: 429, .. }),
        "{err:?}"
    );

    let big = vec![b'{'; 300];
    let base = serve("200 OK", "application/json", &big, Mode::Full).await;
    let client = Client::new(config(base, &pem, 32, 16, Duration::from_secs(2))).unwrap();
    assert!(matches!(
        client.get_order("abc").await.unwrap_err(),
        VenueError::BodyTooLarge { .. }
    ));

    let client = Client::new(config(
        "http://127.0.0.1:1".to_owned(),
        &pem,
        1000,
        100,
        Duration::from_millis(200),
    ))
    .unwrap();
    assert!(matches!(
        client.get_order("abc").await.unwrap_err(),
        VenueError::Transport { .. }
    ));

    let base = serve("200 OK", "application/json", b"{not-json", Mode::Full).await;
    let client = Client::new(config(base, &pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    assert!(
        matches!(
            client.get_order("abc").await.unwrap_err(),
            VenueError::NonJson { .. }
        ),
        "a body that only starts like JSON is not schema drift"
    );
}

#[tokio::test]
async fn recovery_reads_send_only_the_filters_that_were_set() {
    let (pem, verifying) = key_pair();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let seen = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let seen_task = Arc::clone(&seen);
    tokio::spawn(async move {
        for _ in 0..4 {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 4096];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            let text = String::from_utf8_lossy(&buf[..n]).to_string();
            seen_task.lock().await.push(text);
            let body = b"{}";
            let head = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            );
            let _ = socket.write_all(head.as_bytes()).await;
            let _ = socket.write_all(body).await;
        }
    });
    let client = Client::new(config(
        format!("http://{addr}"),
        &pem,
        10_000,
        10_000,
        Duration::from_secs(2),
    ))
    .unwrap();
    let _ = client
        .get_orders(
            &OrdersFilter {
                ticker: Some("SYNTH-1".to_owned()),
                ..OrdersFilter::default()
            },
            SubaccountFilter::All,
        )
        .await;
    let _ = client
        .get_fills(
            &FillsFilter {
                order_id: Some("ord-synthetic-1".to_owned()),
                ..FillsFilter::default()
            },
            SubaccountFilter::All,
        )
        .await;
    let _ = client
        .get_positions(&PositionsFilter::default(), Subaccount::Primary)
        .await;
    let raw = client.get_market_raw("SYNTH-1").await.unwrap();
    assert_eq!(raw.1, serde_json::json!({}));
    let lines: Vec<String> = seen.lock().await.clone();
    assert!(
        lines[0].contains("GET /portfolio/orders?"),
        "{:?}",
        lines[0]
    );
    assert!(lines[0].contains("ticker=SYNTH-1"), "{:?}", lines[0]);
    assert!(!lines[0].contains("exchange_index"), "{:?}", lines[0]);
    let timestamp = header_value(&lines[0], "KALSHI-ACCESS-TIMESTAMP");
    let signature = header_value(&lines[0], "KALSHI-ACCESS-SIGNATURE");
    let path_only = format!("{timestamp}GET/portfolio/orders");
    let signed = Signature::try_from(
        base64::engine::general_purpose::STANDARD
            .decode(signature)
            .unwrap()
            .as_slice(),
    )
    .unwrap();
    verifying.verify(path_only.as_bytes(), &signed).unwrap();
    let with_query = format!("{timestamp}GET/portfolio/orders?ticker=SYNTH-1");
    assert!(
        verifying.verify(with_query.as_bytes(), &signed).is_err(),
        "a signature over the path plus the query was accepted"
    );
    assert!(
        lines[1].contains("order_id=ord-synthetic-1"),
        "{:?}",
        lines[1]
    );
    assert!(!lines[1].contains("ticker="), "{:?}", lines[1]);
    assert!(
        lines[2].contains("GET /portfolio/positions?"),
        "{:?}",
        lines[2]
    );
    assert_eq!(query_value(&lines[2], "subaccount").as_deref(), Some("0"));
    assert!(!lines[2].contains("ticker="), "{:?}", lines[2]);
    let position_ts = header_value(&lines[2], "KALSHI-ACCESS-TIMESTAMP");
    let position_sig = Signature::try_from(
        base64::engine::general_purpose::STANDARD
            .decode(header_value(&lines[2], "KALSHI-ACCESS-SIGNATURE"))
            .unwrap()
            .as_slice(),
    )
    .unwrap();
    verifying
        .verify(
            format!("{position_ts}GET/portfolio/positions").as_bytes(),
            &position_sig,
        )
        .unwrap();
    assert!(
        verifying
            .verify(
                format!("{position_ts}GET/portfolio/positions?subaccount=0").as_bytes(),
                &position_sig,
            )
            .is_err(),
        "a signature over the positions query was accepted"
    );
    assert!(
        lines[3].starts_with("GET /markets/SYNTH-1 "),
        "{:?}",
        lines[3]
    );
    assert_eq!(query_value(&lines[0], "subaccount"), None);
}

fn query_value(raw: &str, name: &str) -> Option<String> {
    let target = raw.lines().next()?.split_whitespace().nth(1)?;
    let query = target.split_once('?')?.1;
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then(|| value.to_owned())
    })
}

fn http_complete(buf: &[u8]) -> bool {
    let Some(split) = buf.windows(4).position(|window| window == b"\r\n\r\n") else {
        return false;
    };
    let head = String::from_utf8_lossy(&buf[..split]);
    let length = head.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        key.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse::<usize>().ok())?
    });
    match length {
        Some(len) => buf.len() >= split + 4 + len,
        None => true,
    }
}

fn body_json(raw: &str) -> serde_json::Value {
    let (_, body) = raw.split_once("\r\n\r\n").unwrap();
    serde_json::from_str(body.trim()).unwrap()
}

fn assert_path_signed(raw: &str, verifying: &VerifyingKey<Sha256>, method: &str, path: &str) {
    let timestamp = header_value(raw, "KALSHI-ACCESS-TIMESTAMP");
    let signature = header_value(raw, "KALSHI-ACCESS-SIGNATURE");
    assert_eq!(header_value(raw, "KALSHI-ACCESS-KEY"), "key-synthetic");
    let line = raw.lines().next().unwrap();
    assert!(line.starts_with(&format!("{method} {path}")), "{line}");
    let message = format!("{timestamp}{method}{path}");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(signature)
        .unwrap();
    let signature = Signature::try_from(bytes.as_slice()).unwrap();
    verifying.verify(message.as_bytes(), &signature).unwrap();
    if let Some((_, query)) = line.split_whitespace().nth(1).unwrap().split_once('?') {
        let with_query = format!("{timestamp}{method}{path}?{query}");
        assert!(
            verifying.verify(with_query.as_bytes(), &signature).is_err(),
            "signature covered the query"
        );
    }
}

async fn record_requests(count: usize) -> (String, Arc<tokio::sync::Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let seen = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let seen_task = Arc::clone(&seen);
    tokio::spawn(async move {
        for _ in 0..count {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let mut buf = Vec::new();
            let mut tmp = [0u8; 8192];
            loop {
                match tokio::time::timeout(Duration::from_millis(300), socket.read(&mut tmp)).await
                {
                    Ok(Ok(0)) | Err(_) => break,
                    Ok(Ok(read)) => {
                        buf.extend_from_slice(&tmp[..read]);
                        if http_complete(&buf) {
                            break;
                        }
                    }
                    Ok(Err(_)) => break,
                }
            }
            seen_task
                .lock()
                .await
                .push(String::from_utf8_lossy(&buf).to_string());
            let body = b"{}";
            let head = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            );
            let _ = socket.write_all(head.as_bytes()).await;
            let _ = socket.write_all(body).await;
        }
    });
    (format!("http://{addr}"), seen)
}

fn sample_order(subaccount: SubaccountChoice) -> CreateOrderV2 {
    CreateOrderV2::build(
        "SYNTH-1",
        BookSide::Bid,
        QtyCenti::parse("1.00").unwrap(),
        Decimal::from_str("0.56").unwrap(),
        TimeInForce::GoodTillCanceled,
        SelfTradePreventionType::Maker,
        "cid-synthetic",
        ExchangeIndexChoice::Omit,
        subaccount,
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
    )
    .unwrap()
}

fn flagged_order(
    post_only: OrderFlag,
    reduce_only: OrderFlag,
    cancel_on_pause: OrderFlag,
) -> CreateOrderV2 {
    CreateOrderV2::build(
        "SYNTH-1",
        BookSide::Bid,
        QtyCenti::parse("1.00").unwrap(),
        Decimal::from_str("0.56").unwrap(),
        TimeInForce::GoodTillCanceled,
        SelfTradePreventionType::Maker,
        "cid-synthetic",
        ExchangeIndexChoice::Omit,
        SubaccountChoice::Primary,
        post_only,
        reduce_only,
        cancel_on_pause,
    )
    .unwrap()
}

fn sample_amend() -> AmendOrderV2 {
    let count = AmendCount::from_filled_and_remaining(
        QtyCenti::parse("1.25").unwrap(),
        QtyCenti::parse("0.50").unwrap(),
    )
    .unwrap();
    AmendOrderV2::build(
        "SYNTH-1",
        BookSide::Bid,
        Decimal::from_str("0.57").unwrap(),
        count,
        "cid-synthetic",
        Some("cid-synthetic-2".to_owned()),
        ExchangeIndexChoice::Omit,
    )
    .unwrap()
}

#[tokio::test]
async fn write_paths_sign_the_method_and_send_subaccount() {
    let (pem, verifying) = key_pair();
    let (base, seen) = record_requests(8).await;
    let client = Client::new(config(base, &pem, 10_000, 10_000, Duration::from_secs(2))).unwrap();
    let _ = client
        .amend_order_v2("ord-synthetic-1", &sample_amend(), Subaccount::Numbered(2))
        .await;
    let _ = client
        .decrease_order_v2(
            "ord-synthetic-1",
            &DecreaseOrderV2::build(
                Decrease::By(QtyCenti::parse("1.00").unwrap()),
                ExchangeIndexChoice::Omit,
                Some("SYNTH-1".to_owned()),
            )
            .unwrap(),
            Subaccount::Primary,
        )
        .await;
    let _ = client
        .cancel_order_v2(
            "ord-synthetic-1",
            Subaccount::Primary,
            ExchangeIndexChoice::Omit,
            Some("SYNTH-1".to_owned()),
        )
        .await;
    let _ = client
        .batch_create_orders_v2(&[
            sample_order(SubaccountChoice::Primary),
            sample_order(SubaccountChoice::KeyRestricted),
        ])
        .await;
    let _ = client
        .batch_cancel_orders_v2(&[
            BatchCancelItem::build(
                "ord-synthetic-1",
                SubaccountChoice::Primary,
                ExchangeIndexChoice::Omit,
                Some("SYNTH-1".to_owned()),
            )
            .unwrap(),
            BatchCancelItem::build(
                "ord-synthetic-2",
                SubaccountChoice::KeyRestricted,
                ExchangeIndexChoice::Shard(-1),
                Some("SYNTH-1".to_owned()),
            )
            .unwrap(),
        ])
        .await;
    let _ = client
        .get_orders(&OrdersFilter::default(), SubaccountFilter::All)
        .await;
    let _ = client
        .get_orders(&OrdersFilter::default(), SubaccountFilter::Only(4))
        .await;
    let _ = client.get_balance(Subaccount::Numbered(5), None).await;

    let lines = seen.lock().await.clone();
    assert_eq!(lines.len(), 8, "{lines:?}");
    assert_path_signed(
        &lines[0],
        &verifying,
        "POST",
        "/portfolio/events/orders/ord-synthetic-1/amend",
    );
    assert_eq!(query_value(&lines[0], "subaccount").as_deref(), Some("2"));
    let amend = body_json(&lines[0]);
    assert_eq!(amend["count"], "1.75");
    assert_eq!(amend["ticker"], "SYNTH-1");
    assert_eq!(amend["side"], "bid");
    assert!(amend.get("exchange_index").is_none());

    assert_path_signed(
        &lines[1],
        &verifying,
        "POST",
        "/portfolio/events/orders/ord-synthetic-1/decrease",
    );
    assert_eq!(query_value(&lines[1], "subaccount").as_deref(), Some("0"));
    let decrease = body_json(&lines[1]);
    assert_eq!(decrease["reduce_by"], "1.00");
    assert!(decrease.get("reduce_to").is_none());

    assert_path_signed(
        &lines[2],
        &verifying,
        "DELETE",
        "/portfolio/events/orders/ord-synthetic-1",
    );
    assert_eq!(query_value(&lines[2], "subaccount").as_deref(), Some("0"));
    assert_eq!(
        query_value(&lines[2], "market_ticker").as_deref(),
        Some("SYNTH-1")
    );
    assert_eq!(query_value(&lines[2], "exchange_index"), None);

    assert_path_signed(
        &lines[3],
        &verifying,
        "POST",
        "/portfolio/events/orders/batched",
    );
    let created = body_json(&lines[3]);
    assert_eq!(created["orders"][0]["subaccount"], 0);
    assert!(created["orders"][1].get("subaccount").is_none());

    assert_path_signed(
        &lines[4],
        &verifying,
        "DELETE",
        "/portfolio/events/orders/batched",
    );
    let cancelled = body_json(&lines[4]);
    assert_eq!(cancelled["orders"][0]["order_id"], "ord-synthetic-1");
    assert_eq!(cancelled["orders"][0]["subaccount"], 0);
    assert_eq!(cancelled["orders"][1]["order_id"], "ord-synthetic-2");
    assert!(cancelled["orders"][1].get("subaccount").is_none());
    assert_eq!(cancelled["orders"][1]["exchange_index"], -1);

    assert_path_signed(&lines[5], &verifying, "GET", "/portfolio/orders");
    assert_eq!(query_value(&lines[5], "subaccount"), None);
    assert_path_signed(&lines[6], &verifying, "GET", "/portfolio/orders");
    assert_eq!(query_value(&lines[6], "subaccount").as_deref(), Some("4"));
    assert_path_signed(&lines[7], &verifying, "GET", "/portfolio/balance");
    assert_eq!(query_value(&lines[7], "subaccount").as_deref(), Some("5"));
    assert_eq!(query_value(&lines[7], "exchange_index"), None);
}

#[tokio::test]
async fn rejected_subaccount_numbers_do_not_leave_the_client() {
    let (pem, _) = key_pair();
    let client = Client::new(config(
        "http://127.0.0.1:1".to_owned(),
        &pem,
        1000,
        1000,
        Duration::from_millis(50),
    ))
    .unwrap();
    assert!(matches!(
        client
            .get_orders(&OrdersFilter::default(), SubaccountFilter::Only(64))
            .await
            .unwrap_err(),
        VenueError::RequestBuild { .. }
    ));
    assert!(matches!(
        client
            .get_positions(&PositionsFilter::default(), Subaccount::Numbered(0))
            .await
            .unwrap_err(),
        VenueError::RequestBuild { .. }
    ));
    assert!(CreateOrderV2::build(
        "SYNTH-1",
        BookSide::Bid,
        QtyCenti::parse("1.00").unwrap(),
        Decimal::from_str("0.56").unwrap(),
        TimeInForce::GoodTillCanceled,
        SelfTradePreventionType::Maker,
        "cid-synthetic",
        ExchangeIndexChoice::Omit,
        SubaccountChoice::Numbered(64),
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
    )
    .is_err());
}

fn assert_flag(body: &serde_json::Value, name: &str, expected: Option<bool>) {
    match expected {
        Some(value) => assert_eq!(body[name], value, "{name} in {body}"),
        None => assert!(body.get(name).is_none(), "{name} was sent in {body}"),
    }
}

fn assert_order_flags(
    body: &serde_json::Value,
    post_only: Option<bool>,
    reduce_only: Option<bool>,
    cancel_on_pause: Option<bool>,
) {
    assert_flag(body, "post_only", post_only);
    assert_flag(body, "reduce_only", reduce_only);
    assert_flag(body, "cancel_order_on_pause", cancel_on_pause);
}

#[tokio::test]
async fn order_flags_reach_single_and_batch_bodies() {
    let (pem, verifying) = key_pair();
    let (base, seen) = record_requests(4).await;
    let client = Client::new(config(base, &pem, 10_000, 10_000, Duration::from_secs(2))).unwrap();
    let all_true = flagged_order(OrderFlag::True, OrderFlag::True, OrderFlag::True);
    let all_false = flagged_order(OrderFlag::False, OrderFlag::False, OrderFlag::False);
    let omitted = flagged_order(
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
    );
    let mixed = flagged_order(OrderFlag::True, OrderFlag::False, OrderFlag::VenueDefault);
    let _ = client.create_order_v2(&all_true).await;
    let _ = client.create_order_v2(&all_false).await;
    let _ = client.create_order_v2(&omitted).await;
    let _ = client
        .batch_create_orders_v2(&[all_true, all_false, omitted, mixed])
        .await;

    let lines = seen.lock().await.clone();
    assert_eq!(lines.len(), 4, "{lines:?}");
    for line in &lines[..3] {
        assert_path_signed(line, &verifying, "POST", "/portfolio/events/orders");
    }
    assert_path_signed(
        &lines[3],
        &verifying,
        "POST",
        "/portfolio/events/orders/batched",
    );
    assert_order_flags(&body_json(&lines[0]), Some(true), Some(true), Some(true));
    assert_order_flags(&body_json(&lines[1]), Some(false), Some(false), Some(false));
    assert_order_flags(&body_json(&lines[2]), None, None, None);
    let batched = body_json(&lines[3]);
    let orders = batched["orders"].as_array().unwrap();
    assert_eq!(orders.len(), 4);
    assert_order_flags(&orders[0], Some(true), Some(true), Some(true));
    assert_order_flags(&orders[1], Some(false), Some(false), Some(false));
    assert_order_flags(&orders[2], None, None, None);
    assert_order_flags(&orders[3], Some(true), Some(false), None);
}

#[tokio::test]
async fn signed_raw_get_signs_the_trade_api_path_for_both_bases() {
    let (pem, verifying) = key_pair();
    let (base, seen) = record_requests(3).await;
    let path = "/exchange/user_data_timestamp";
    let signed = "/trade-api/v2/exchange/user_data_timestamp";
    let origin = Client::new(config(
        base.clone(),
        &pem,
        10_000,
        10_000,
        Duration::from_secs(2),
    ))
    .unwrap();
    origin.get_signed_raw(path).await.unwrap();
    let prefixed = Client::new(config(
        format!("{base}/trade-api/v2"),
        &pem,
        10_000,
        10_000,
        Duration::from_secs(2),
    ))
    .unwrap();
    prefixed.get_signed_raw(path).await.unwrap();
    origin.get_signed_raw(signed).await.unwrap();
    let rejected = origin
        .get_signed_raw("/exchange/user_data_timestamp?subaccount=1")
        .await
        .unwrap_err();
    assert!(
        matches!(rejected, VenueError::RequestBuild { .. }),
        "{rejected:?}"
    );

    let lines = seen.lock().await.clone();
    assert_eq!(lines.len(), 3, "{lines:?}");
    for line in &lines {
        assert_path_signed(line, &verifying, "GET", signed);
        assert!(
            !line.contains("/trade-api/v2/trade-api/v2/"),
            "prefix was applied twice: {line}"
        );
    }
}

#[tokio::test]
async fn unknown_enum_fixtures_are_schema_drift() {
    let (pem, _) = key_pair();
    let root = format!("{}/tests/fixtures/s2b/drift", env!("CARGO_MANIFEST_DIR"));
    drift_case(
        &pem,
        &std::fs::read_to_string(format!("{root}/order_status.json")).unwrap(),
        "weird",
        "resting",
        |client| async move {
            client
                .get_orders(&OrdersFilter::default(), SubaccountFilter::All)
                .await
        },
    )
    .await;
    drift_case(
        &pem,
        &std::fs::read_to_string(format!("{root}/fill_side.json")).unwrap(),
        "mid",
        "yes",
        |client| async move {
            client
                .get_fills(&FillsFilter::default(), SubaccountFilter::All)
                .await
        },
    )
    .await;
    drift_case(
        &pem,
        &std::fs::read_to_string(format!("{root}/fill_action.json")).unwrap(),
        "hold",
        "buy",
        |client| async move {
            client
                .get_fills(&FillsFilter::default(), SubaccountFilter::All)
                .await
        },
    )
    .await;
}

async fn drift_case<T, F, Fut>(pem: &str, body: &str, unknown: &str, legal: &str, call: F)
where
    T: std::fmt::Debug,
    F: Fn(Client) -> Fut,
    Fut: std::future::Future<Output = Result<T, VenueError>>,
{
    assert!(
        body.contains(unknown),
        "fixture no longer contains the unknown enum {unknown}"
    );
    let base = serve("200 OK", "application/json", body.as_bytes(), Mode::Full).await;
    let client = Client::new(config(base, pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    let err = call(client).await.unwrap_err();
    assert!(
        matches!(err, VenueError::SchemaDrift { .. }),
        "unknown enum {unknown} was not schema drift: {err:?}"
    );
    let replaced = body.replacen(unknown, legal, 1);
    assert!(!replaced.contains(unknown));
    let base = serve(
        "200 OK",
        "application/json",
        replaced.as_bytes(),
        Mode::Full,
    )
    .await;
    let client = Client::new(config(base, pem, 100_000, 100_000, Duration::from_secs(2))).unwrap();
    call(client)
        .await
        .unwrap_or_else(|err| panic!("legal value {legal} was rejected: {err:?}"));
}
