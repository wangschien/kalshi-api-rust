# Contributing

Maintainer notes for kalshi-api-rust. The user guide is [README.md](README.md).

## From the official OpenAPI document

`kalshi-api-openapi` is generated from Kalshi's official OpenAPI document, info version 3.35.0. API documentation is at <https://docs.kalshi.com>. The locked document is `spec/openapi.yaml`. Its sha256, the generator coordinate, and the generator jar sha256 are in `tools/kalshi-sdk/pins.toml`. openapi-generator 7.17.0 writes the crate. Generated sources are not edited by hand.

`tools/kalshi-sdk/drift.sh` regenerates into a temporary directory, runs `cargo fmt`, inserts four crate-level clippy allows next to the allows the generator already emits, and fails if that tree differs from `crates/kalshi-api-openapi`. Those allows are `needless_return`, `derivable_impls`, `empty_docs`, and `into_iter_on_ref`. The same command checks `tools/kalshi-sdk/subaccount_semantics.json` against each subaccount field's description in the OpenAPI document.

`spec/asyncapi.yaml` describes the handwritten WebSocket client. It is not used to generate Rust code.

## Comparison with the official Python SDK

The source of truth is `kalshi_python_async` 3.35.0. Its wheel URL and SHA-256 are pinned in `tools/kalshi-sdk/pins.toml`. The 119 operations expose 357 public Python methods when response-loading variants are included. `conformance.py` checks every model field, required key, enum, method, and parameter against this wheel. The model exception file is empty. Every operation also has `_with_http_info`, returning `ApiResponse<T>` with status, headers and original body bytes, and `_without_preload_content`, returning an unread `reqwest::Response` even for HTTP error statuses.

The current official OpenAPI labels itself 3.35.0 and matches the pinned Python async SDK on those operations, except `get_weather_index` uses Rust `from` for Python `var_from`. RFQ batch communications surfaces remain Rust-only in `expected_api_async.json` until the Python SDK exposes them.

Generation adds the Python SDK's media headers, omits absent optional request bodies, and preserves its model defaults from `python_model_defaults.json`. JSON number fields use `serde_json::Number` with arbitrary precision. Fixed-point quantities and dollar amounts remain strings. This prevents integer values in Python's integer-or-float fields from being rounded through `f64`.

`make_fixtures.py` executes the pinned SDK's own public methods, model validation, and request serialization. The fixtures cover 714 public-method requests, 937 model cases including every inline enum value and 11 complete OpenAPI examples, 598 required-field rejections, 165 WebSocket cases, and 10 cross-language signing vectors. Network connections and HTTP session creation are disabled during fixture generation. It uses only synthetic keys. The committed fixtures drive offline Rust request, model, signing, and WebSocket tests; Python is not a Rust build or runtime dependency. Query comparisons decode equivalent percent and form encodings. Date query inputs use the SDK's configured date format.

To run the static source comparison and Rust checks, unpack the pinned wheel, set `KALSHI_PYTHON_SDK` to its package directory, and run `tools/kalshi-sdk/verify_offline.sh`. `CARGO` can select a Cargo wrapper, and `CARGO_TARGET_DIR` selects the build directory. To regenerate fixtures, convert the two YAML specifications to JSON, generate `python_surface.json` with `parity.py`, and pass those paths to `make_fixtures.py` using an isolated Python environment containing the pinned SDK's runtime dependencies.

The SDK does not retry. The OpenAPI document specifies no retry or backoff policy, so a server error, a rate-limit response, or a transport error is returned to the caller on the first attempt. Both `ClientConfig` and generated `Configuration::new(connect_timeout, request_timeout)` take caller-supplied `Option<Duration>` values; `None` sets no local deadline. `ClientConfig::max_body_bytes` is `Option<usize>`; `None` sets no response body limit. The SDK has no default number for any of them: the Kalshi documents give none. Generated configuration has no `Default`. Its constructor applies the bounds that are set; every generated request also sets `request_timeout` when it is `Some`, including when a caller replaces the middleware client. The request deadline covers response headers and body consumption, including the unread response variant.

The `demo_read_only` example reads `KALSHI_DEMO_ID` and `KALSHI_DEMO_FILE_PATH`. It also requires `KALSHI_DEMO_LISTEN_MS`. `KALSHI_DEMO_WS_SEND_TIMEOUT_MS` is optional; when it is unset, the WebSocket session sets no local send deadline. The caller chooses those durations and should cite their source. It sets no REST deadline or body limit. It performs four GETs and an orderbook subscription for the chosen duration on fixed demo hosts, and reports a JSON receipt. It has no order-writing path.

## Known spec inconsistencies

`GetOrdersParams`, `GetFillsParams`, and `GetPositionsParams` type `exchange_index` as `i32`. The `ExchangeIndex` schema is an integer with `format: int64` and is `i64` on the model types. `kalshi-api` converts an `i64` to that query field and rejects values outside `i32`.

`Environment::Prod` is `https://external-api.kalshi.com/trade-api/v2`, the spec's first production server and the official Python async SDK default. `Environment::ProdShared` is `https://api.elections.kalshi.com/trade-api/v2`.

The AsyncAPI document marks `clock` required on `orderbook_snapshot` and `orderbook_delta` messages. The exchange does not send it: production and demo order book frames carry no `clock` key. `OrderbookSnapshotBody::clock` and `OrderbookDeltaBody::clock` are therefore `Option<i64>`, and `None` means the frame had no `clock`. Both fields are listed in `NOT_SENT` in `tools/kalshi-sdk/ws_fields.py`; `ws_codegen.py` and `ws_fields.json` read that table, and generation fails if the document stops requiring either field. Measured on a recorded production session of one market, KXBTC15M-26OCT250945-45 (2026-10-25 13:30 to 13:45 UTC). The file is gzip JSON lines named `<recorder>-KXBTC15M-26OCT250945-45-<window start>-<window end>-<id>.jsonl.gz`, one received message per line with the wire frame under `raw`. It has 425,734 lines (`zcat FILE | wc -l`) and 402,649 order book frames, one snapshot and 402,648 deltas (`zcat FILE | grep -c '"type":"orderbook_'`); 0 of them have `clock` (`zcat FILE | grep '"type":"orderbook_' | grep -c '"clock"'`). Order book frames recorded on demo on 2026-10-24 have no `clock` either. `crates/kalshi-api/tests/fixtures/recorded` holds the first snapshot and the first delta of that session, wire frame only.

## Operations

Each row has a typed method and, except for cancel-all, a `*_raw` method that returns the JSON value only when the body is complete. Fixtures combine synthetic inputs with official OpenAPI and AsyncAPI examples.

The official Python SDK leaves `subaccount` optional. This typed client does not. That difference is deliberate. The same SDK leaves `post_only`, `reduce_only`, and `cancel_order_on_pause` optional. `CreateOrderV2::build` requires `OrderFlag` for each one. `OrderFlag::True` and `OrderFlag::False` are written on the request, including `false`. `OrderFlag::VenueDefault` omits that field. `SubaccountFilter::All` omits the query, which means every subaccount. `SubaccountFilter::Only` sends one subaccount, and `0` means only the primary account. `Subaccount::Primary` sends `0` on calls whose omitted query means the primary account. `Subaccount::Numbered` sends `1` through `63`. `SubaccountChoice::Primary` sends `0` in the JSON body, `Numbered` sends `1` through `63`, and `KeyRestricted` omits the field for a key locked to one subaccount. Raw methods still take `Option<i64>` where the spec's parameter is optional, and `None` omits the field.

`CancelAllOrders` is intentionally not wrapped. Its operation text is: "Cancels all resting event-market orders for the authenticated Direct member across every exchange shard. If `subaccount` is omitted, matching orders may come from any subaccount. If it is provided, only orders for that subaccount are eligible. Newly placed orders may also be cancelled during the minute after the request." The query parameter says: "Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts." The generated `cancel_all_orders` function is the raw call.

| Operation | Typed method | Raw method | Omitted subaccount | Exchange index | Sample |
| --- | --- | --- | --- | --- | --- |
| Create order (V2) | `create_order_v2` | `create_order_v2_raw` | `SubaccountChoice` on `CreateOrderV2::build`: `0`, `1`–`63`, or omit for a restricted key | `ExchangeIndexChoice` on the order. Omit auto-routes when the ticker is present | `crates/kalshi-api/tests/fixtures/s2b/create_order_v2.json` |
| Amend order (V2) | `amend_order_v2` | `amend_order_v2_raw` | `Subaccount::Primary` sends `0` | `ExchangeIndexChoice` on the amend body | `crates/kalshi-api/tests/fixtures/s2b/amend_order_v2.json` |
| Decrease order (V2) | `decrease_order_v2` | `decrease_order_v2_raw` | `Subaccount::Primary` sends `0` | `ExchangeIndexChoice` on the decrease body | `crates/kalshi-api/tests/fixtures/s2b/decrease_order_v2.json` |
| Cancel order (V2) | `cancel_order_v2` | `cancel_order_v2_raw` | `Subaccount::Primary` sends `0` | `ExchangeIndexChoice` on the query | `crates/kalshi-api/tests/fixtures/s2b/cancel_order_v2.json` |
| Batch create orders (V2) | `batch_create_orders_v2` | `batch_create_orders_v2_raw` | `SubaccountChoice` on each `CreateOrderV2` | `ExchangeIndexChoice` on each order | `crates/kalshi-api/tests/fixtures/s2b/batch_create_orders_v2.json` |
| Batch cancel orders (V2) | `batch_cancel_orders_v2` | `batch_cancel_orders_v2_raw` | `SubaccountChoice` on each `BatchCancelItem` | `ExchangeIndexChoice` on each item | `crates/kalshi-api/tests/fixtures/s2b/batch_cancel_orders_v2.json` |
| Cancel all orders | not wrapped | generated `cancel_all_orders` | every subaccount | none | none |
| Get order | `get_order` | `get_order_raw` | none | none | `crates/kalshi-api/tests/fixtures/get_order.json` |
| Get orders | `get_orders` | `get_orders_raw` | `SubaccountFilter::All` omits the query (every subaccount) | optional filter; omit returns every shard | `crates/kalshi-api/tests/fixtures/s2b/get_orders.json` |
| Get fills | `get_fills` | `get_fills_raw` | `SubaccountFilter::All` omits the query (every subaccount) | optional filter; omit returns every shard | `crates/kalshi-api/tests/fixtures/s2b/get_fills.json` |
| Get positions | `get_positions` | `get_positions_raw` | `Subaccount::Primary` sends `0` | optional filter; omit returns every shard | `crates/kalshi-api/tests/fixtures/s2b/get_positions.json` |
| Get balance | `get_balance` | `get_balance_raw` | `Subaccount::Primary` sends `0` | optional; omit includes every exchange index | `crates/kalshi-api/tests/fixtures/s2b/get_balance.json` |
| Get market | `get_market` | `get_market_raw` | none | none | `crates/kalshi-api/tests/fixtures/s2b/get_market.json` |
| Get markets | `get_markets` | `get_markets_raw` | none | none | `crates/kalshi-api/tests/fixtures/s2b/get_markets.json` |
| Get order queue position | `get_order_queue_position` | `get_order_queue_position_raw` | none | none | `crates/kalshi-api/tests/fixtures/s2b/get_order_queue_position.json` |
| Get order queue positions | `get_order_queue_positions` | `get_order_queue_positions_raw` | `Subaccount::Primary` sends `0` | none | `crates/kalshi-api/tests/fixtures/s2b/get_order_queue_positions.json` |
| Get exchange status | `get_exchange_status` | `get_exchange_status_raw` | none | none | `crates/kalshi-api/tests/fixtures/s2b/get_exchange_status.json` |
| Get exchange schedule | `get_exchange_schedule` | `get_exchange_schedule_raw` | none | none | `crates/kalshi-api/tests/fixtures/s2b/get_exchange_schedule.json` |
| Get user-data timestamp | `get_user_data_timestamp` | `get_user_data_timestamp_raw` | none | none | none |
| Get historical cutoff | `get_historical_cutoff` | `get_historical_cutoff_raw` | none | none | none |
| Get account limits | `get_account_api_limits` | `get_account_api_limits_raw` | none | none | none |
| Get endpoint costs | `get_account_endpoint_costs` | `get_account_endpoint_costs_raw` | none | none | none |
| Get market order book | `get_market_orderbook` | `get_market_orderbook_raw` | none | `depth` is sent only when `Some` | none |
| Signed GET | `get_signed_raw` | the same method | none | origin or `/trade-api/v2` base; path is `/trade-api/v2/...`; a query in the path is rejected | none |

## WebSocket

`kalshi_api::ws` message types are generated from `spec/asyncapi.yaml` into `crates/kalshi-api/src/ws/generated.rs` by `python3 tools/kalshi-sdk/ws_codegen.py`. Do not edit that file. `ws_codegen.py --check` fails when it differs from a fresh generation. The socket transport is hand-written. `WsHost::Prod` is `wss://external-api-ws.kalshi.com/trade-api/ws/v2`. `WsHost::Elections` is `wss://api.elections.kalshi.com/trade-api/ws/v2`. `WsHost::Demo` is `wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2`.

The handshake is `GET /trade-api/ws/v2`. The signature is `timestamp + GET +` that path, with the query removed, and the same RSA-PSS or Ed25519 headers as the REST client. `SignMiddleware::headers` is public for a caller that owns its own socket. Create and amend prices use four decimal places, so `0.30` is sent as `0.3000`. Order book snapshots also accept the earlier `yes_dollars` and `no_dollars` ladder names. `cfbenchmarks_value` and `cfbenchmarks_value_5hz` can be subscribed and parsed, including each channel's index list. `SubscriptionAction::IndexList` sends `update_subscription` with `action` set to `indexlist` for either of those subscriptions. The subscription id selects the channel. The list request does not add or remove index ids. Ping and pong are handled on the socket task. `last_ping_rx_ns` records when a Ping frame arrives. Unread frames do not delay the pong. Incoming frames and outgoing commands use unbounded channels, so a frame the caller has not read yet is kept rather than dropped, and `send` never waits for queue space; memory grows with what is left unread. The AsyncAPI document sets no client queue depth, so the SDK sets none. Reconnect, resubscribe, and recovery are the caller's responsibility.

`WsSession::connect` takes a final `send_timeout: Option<Duration>` argument; `WsConfig` has the same field. `None` sets no local deadline, and each write is awaited until it completes or fails. The SDK has no default number. `Some` bounds each command, Ping, Pong, and Close write. A send that stalls past it discards the socket and delivers `WsEvent::SendTimedOut { timeout }` after already queued frames, then `recv` returns `None`. The terminal event bypasses the application queue. The client never resends a timed-out frame.

`user_order` status is a closed set, and the wire value `unknown` is one of those values. A status outside the set fails that frame. `last_update_reason` keeps an unrecognized reason. The market position frame type is `market_position`.

`tools/kalshi-sdk/ws_fields.py --check` compares `tools/kalshi-sdk/ws_fields.json` with the fields of the typed commands and messages in `spec/asyncapi.yaml`. A field in `NOT_SENT` is listed as optional, with `document_presence` recording that the document requires it. A required key whose schema allows null must be present. `tools/kalshi-sdk/ws_schema.py --check` validates the synthetic fixtures in `crates/kalshi-api/tests/fixtures/s3` against AsyncAPI `components.schemas`. The schema check needs `jsonschema==4.10.3` and `PyYAML==6.0.1`. The field list needs `PyYAML==6.0.1`.

## Build

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

Rust 1.93.1 is pinned in `rust-toolchain.toml`.

## Regenerate

Pins, the integer overlay, and the generator command are in `tools/kalshi-sdk/pins.toml`. The generator jar is `org.openapitools:openapi-generator-cli:7.17.0`. Its URL and sha256 are in that file.

```bash
export JAVA=java
export KALSHI_GENERATOR_JAR=/path/to/openapi-generator-cli-7.17.0.jar
tools/kalshi-sdk/regen.sh
tools/kalshi-sdk/drift.sh
```

`regen.sh` refuses to run when the OpenAPI document, the AsyncAPI document, the generator jar, or the overlay output does not match the pinned sha256. `drift.sh` regenerates into a temporary directory and fails if that tree differs from `crates/kalshi-api-openapi`.

The overlay lists every integer schema that omits `format` and sets `format: int64` on those nodes only. Integers that already declare `int32` or `int64` are left unchanged. This is a limit of the 7.17.0 Rust generator, which otherwise emits `i32` for a format-less integer.

`kalshi-api` is the hand-written client. It signs with the official RSA-PSS or Ed25519 scheme, captures each response before the generated decode, and builds order requests without `Default`. Timeouts, the base URL, and the body limits are required constructor fields.


## Release

`tools/kalshi-sdk/check_upstream.sh` reads the `source` fields in `tools/kalshi-sdk/pins.toml`, downloads the official OpenAPI document, the official AsyncAPI document, and the latest `kalshi_python_async` wheel, then compares their hashes and versions with that file.

`tools/kalshi-sdk/release.sh --check` runs the gates and prints the version selected from `cargo semver-checks`. `tools/kalshi-sdk/release.sh --publish` runs those gates again, commits the release, pushes `main`, and pushes the `v<version>` tag. It does not upload crates. `.github/workflows/release.yml` publishes `kalshi-api-openapi`, waits until that version is visible, then publishes `kalshi-api`. A version already on the index is skipped. The workflow uses trusted publishing and has no long-lived registry token. `AUTO_RELEASE=off` opens a pull request instead. A red gate does not publish.

## License

MIT. See [LICENSE](LICENSE). The generated crate's own manifest keeps the license value emitted by the generator.
