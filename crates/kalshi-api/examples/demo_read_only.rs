use std::sync::Arc;
use std::time::Duration;

use anyhow::{ensure, Context};
use kalshi_api::ws::{
    Channel, ClientCommand, Incoming, SubscribeParams, WsEvent, WsHost, WsSession,
};
use kalshi_api::{Client, ClientConfig, Environment, MarketsFilter, SystemClock};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
#[allow(
    clippy::disallowed_methods,
    reason = "CLI startup owns the runtime and reads the demo key before network work."
)]
async fn main() -> anyhow::Result<()> {
    let key_id = std::env::var("KALSHI_DEMO_ID").context("KALSHI_DEMO_ID is required")?;
    let key_path =
        std::env::var("KALSHI_DEMO_FILE_PATH").context("KALSHI_DEMO_FILE_PATH is required")?;
    let pem = std::fs::read_to_string(key_path).context("cannot read demo key")?;
    let client = Client::new(ClientConfig {
        base_url: Environment::Demo.base_url().to_owned(),
        key_id: key_id.clone(),
        private_key_pem: pem.clone(),
        connect_timeout: Duration::from_secs(10),
        request_timeout: Duration::from_secs(20),
        max_body_bytes: 4 * 1024 * 1024,
        max_capture_bytes: 64 * 1024,
    })?;
    let status = client.get_exchange_status().await?;
    let markets = client
        .get_markets(&MarketsFilter {
            limit: Some(1),
            status: Some("open".to_owned()),
            ..MarketsFilter::default()
        })
        .await?;
    let ticker = &markets
        .markets
        .first()
        .context("demo returned no open market")?
        .ticker;
    let market = client.get_market(ticker).await?;
    ensure!(
        &market.market.ticker == ticker,
        "market ticker did not match"
    );
    let _ = client.get_market_orderbook(ticker, Some(1)).await?;
    let mut socket = tokio::time::timeout(
        Duration::from_secs(20),
        WsSession::connect(
            WsHost::Demo,
            key_id,
            pem,
            Arc::new(SystemClock),
            Duration::from_secs(10),
        ),
    )
    .await??;
    socket
        .send(ClientCommand::Subscribe {
            id: 1,
            params: SubscribeParams {
                channels: vec![Channel::OrderbookDelta],
                market_ticker: Some(ticker.clone()),
                market_tickers: None,
                market_id: None,
                market_ids: None,
                send_initial_snapshot: None,
                skip_ticker_ack: None,
                use_yes_price: Some(true),
                user_filter: None,
                shard_factor: None,
                shard_key: None,
                index_ids: None,
                underlying_tickers: None,
            },
        })
        .await?;
    let started = tokio::time::Instant::now();
    let deadline = started + Duration::from_secs(30);
    let mut subscribed = false;
    let mut snapshots = 0;
    let mut deltas = 0;
    let mut errors = 0;
    let mut rejected = 0;
    let mut lost = 0;
    let mut sequence_gaps = 0;
    let mut last_sequence = None;
    loop {
        let event = match tokio::time::timeout_at(deadline, socket.recv()).await {
            Err(_) => break,
            Ok(Some(event)) => event,
            Ok(None) => {
                lost += 1;
                break;
            }
        };
        match event {
            WsEvent::Frame(frame) => {
                let sequence = match *frame {
                    Incoming::Subscribed { .. } => {
                        subscribed = true;
                        None
                    }
                    Incoming::OrderbookSnapshot { seq, .. } => {
                        snapshots += 1;
                        Some(seq)
                    }
                    Incoming::OrderbookDelta { seq, .. } => {
                        deltas += 1;
                        Some(seq)
                    }
                    Incoming::Error { .. } => {
                        errors += 1;
                        None
                    }
                    _ => None,
                };
                if let Some(sequence) = sequence {
                    if last_sequence.is_some_and(|previous| previous + 1 != sequence) {
                        sequence_gaps += 1;
                    }
                    last_sequence = Some(sequence);
                }
            }
            WsEvent::Rejected { .. } => rejected += 1,
            WsEvent::Lost { .. } => lost += 1,
            WsEvent::SendTimedOut { .. } => {
                errors += 1;
                break;
            }
        }
    }
    let receipt = json!({
        "environment": "demo", "rest_gets": 4, "venue_order_writes": 0,
        "exchange_active": status.exchange_active, "trading_active": status.trading_active,
        "market_ticker": ticker, "subscribed": subscribed, "snapshots": snapshots,
        "deltas": deltas, "rejected": rejected, "errors": errors, "lost": lost,
        "sequence_gaps": sequence_gaps, "duration_ms": started.elapsed().as_millis(),
    });
    println!("{receipt}");
    ensure!(
        subscribed
            && snapshots > 0
            && errors == 0
            && rejected == 0
            && lost == 0
            && sequence_gaps == 0,
        "demo subscription verification failed"
    );
    Ok(())
}
