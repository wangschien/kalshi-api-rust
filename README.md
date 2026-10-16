# kalshi-api-rust

Unofficial async Rust client for the Kalshi Trading API, generated from Kalshi's official OpenAPI document. It tracks official updates and usually publishes a new version within an hour of one. That is a target, not a guarantee.

Unofficial; not affiliated with Kalshi.

- API version: 3.34.0
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
use std::time::Duration;

use kalshi_api::{Client, ClientConfig, Environment, Subaccount};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key_pem = std::fs::read_to_string("path/to/private_key.pem")?;
    let client = Client::new(ClientConfig {
        base_url: Environment::Prod.base_url().to_owned(),
        key_id: "your-api-key-id".to_owned(),
        private_key_pem,
        connect_timeout: Duration::from_secs(5),
        request_timeout: Duration::from_secs(30),
        max_body_bytes: 1_048_576,
        max_capture_bytes: 1_048_576,
    })?;
    let balance = client.get_balance(Subaccount::Primary, None).await?;
    println!("Balance: ${}", balance.balance_dollars);
    Ok(())
}
```

The generated client uses `Configuration::new(connect_timeout, request_timeout)?`, with both values supplied as `Duration`; there is no default configuration. Each request retains its deadline when the middleware client is replaced. WebSocket sessions require a caller-supplied `send_timeout` and end with `WsEvent::SendTimedOut` if a write stalls.

## Documentation

Full documentation is available at [docs.kalshi.com](https://docs.kalshi.com). Crate docs are on [docs.rs](https://docs.rs/kalshi-api).

## License

MIT. See [LICENSE](LICENSE).
