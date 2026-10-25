#![allow(clippy::disallowed_methods)]

use std::sync::Arc;
use std::time::Duration;

use kalshi_api::{Channel, ClientCommand, Clock, SubscribeParams, WsConfig, WsEvent, WsSession};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

struct FixedClock;

impl Clock for FixedClock {
    fn unix_ms(&self) -> u64 {
        1_700_000_000_000
    }

    fn unix_ns(&self) -> u64 {
        1_700_000_000_000_000_123
    }
}

/// Test-chosen deadline; it is not an SDK default.
const DEADLINE: Duration = Duration::from_millis(100);
/// The test's own guard, so a broken build fails instead of hanging.
const TEST_GUARD: Duration = Duration::from_secs(30);
/// Enough unread bytes to fill the loopback socket buffers on both ends.
const COMMANDS: u64 = 128;
const TICKERS_PER_COMMAND: usize = 4096;

fn large_subscribe(id: u64) -> ClientCommand {
    ClientCommand::Subscribe {
        id,
        params: SubscribeParams {
            channels: vec![Channel::OrderbookDelta],
            market_ticker: None,
            market_tickers: Some(
                (0..TICKERS_PER_COMMAND)
                    .map(|n| format!("SYNTHETIC-{id}-{n:0>48}"))
                    .collect(),
            ),
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

#[tokio::test]
async fn connect_with_passes_a_some_send_timeout_to_the_session() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        // Complete the handshake, then never read, so client writes stall.
        let socket = accept_async(stream).await.unwrap();
        let _ = release_rx.await;
        drop(socket);
    });
    let mut session = WsSession::connect_with(WsConfig {
        url: format!("ws://{addr}/trade-api/ws/v2"),
        key_id: "synthetic-key".to_owned(),
        private_key_pem: include_str!("fixtures/parity/ed25519_test_private.pem").to_owned(),
        clock: Arc::new(FixedClock),
        send_timeout: Some(DEADLINE),
    })
    .await
    .unwrap();
    for id in 1..=COMMANDS {
        session.send(large_subscribe(id)).await.unwrap();
    }
    let event = tokio::time::timeout(TEST_GUARD, session.recv())
        .await
        .expect("no SendTimedOut: the Some send_timeout did not reach the session");
    assert_eq!(event, Some(WsEvent::SendTimedOut { timeout: DEADLINE }));
    assert_eq!(session.recv().await, None);
    let _ = release_tx.send(());
    server.await.unwrap();
}
