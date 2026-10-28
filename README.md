# kalshi-api-rust

Unofficial async Rust client for the Kalshi Trading API, generated from Kalshi's official OpenAPI document. It tracks official updates and usually publishes a new version within an hour of one. That is a target, not a guarantee.

Unofficial; not affiliated with Kalshi.

- API version: 3.35.0
- Crate version: 3.34.0
- Generator version: openapi-generator 7.17.0

## Requirements

Rust 1.93.1 and tokio.

## Installation

```bash
cargo add kalshi-api
```

## Getting Started

```rust,no_run
use kalshi_api::{Client, ClientConfig, Environment, Subaccount};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key_pem = std::fs::read_to_string("path/to/private_key.pem")?;
    let client = Client::new(ClientConfig {
        base_url: Environment::Prod.base_url().to_owned(),
        key_id: "your-api-key-id".to_owned(),
        private_key_pem,
        // The caller chooses these values. `None` sets no local bound; a
        // `Some` value should cite the source it comes from.
        connect_timeout: None,
        request_timeout: None,
        max_body_bytes: None,
        max_capture_bytes: 0,
    })?;
    let balance = client.get_balance(Subaccount::Primary, None).await?;
    println!("Balance: ${}", balance.balance_dollars);
    Ok(())
}
```

The generated client uses `Configuration::new(connect_timeout, request_timeout)?`, with both values supplied as `Option<Duration>`; there is no default configuration. `None` sets no local deadline, and `ClientConfig::max_body_bytes: None` sets no response body limit. The SDK has no default number for any of them, and this README suggests none: the caller chooses each value and should cite the source for any `Some` value it sets. A request deadline that is set stays on each request when the middleware client is replaced. The WebSocket `send_timeout` is also `Option<Duration>`: `None` sets no local send deadline, and a `Some` deadline ends the session with `WsEvent::SendTimedOut` if a write stalls past it.

## Documentation

Full documentation is available at [docs.kalshi.com](https://docs.kalshi.com). Crate docs are on [docs.rs](https://docs.rs/kalshi-api).

## License

MIT. See [LICENSE](LICENSE).
