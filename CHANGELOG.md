# Changelog

## 3.34.0

- Require caller-supplied connection and request deadlines for generated REST configuration; remove its unbounded `Default`.
- Require a WebSocket send deadline and report `WsEvent::SendTimedOut` when a write stalls, ending the session without resending.

- Align 118 REST operations and 207 models with the official Python async SDK 3.34.0. Keep the current OpenAPI's two additional optional event filters as documented extensions.
- Preserve JSON numbers and Python model defaults. Match media headers and omitted request bodies.
- Add Ed25519 signing, remaining WebSocket channels and payloads, sending timestamps, subscription updates, and client control Ping.
- Add explicit server-error retry middleware, offline cross-language fixtures, and a demo read-only example.

## [Unreleased]

- WebSocket incoming frames and outgoing commands use unbounded channels. Remove `WsConfig::incoming_capacity`, `WsEvent::Lost`, and `WsSession::dropped_frames`: an unread frame is kept instead of ending the stream. The AsyncAPI document sets no client queue depth.
- `WsConfig::send_timeout` and the last argument of `WsSession::connect` are `Option<Duration>`; `None` sets no local send deadline. The SDK has no default number. A `Some` deadline keeps the `WsEvent::SendTimedOut` behavior.
- `ClientConfig::connect_timeout`, `ClientConfig::request_timeout`, and generated `Configuration::new` take `Option<Duration>`; `None` sets no local deadline. `ClientConfig::max_body_bytes` is `Option<usize>`; `None` sets no response body limit. The SDK has no default number for any of them.
- Remove `RetryMiddleware`. The OpenAPI document specifies no retry or backoff policy; server errors are returned on the first response.
- `_raw` methods report a request that was sent but got no reply as `Transport { kind: Timeout }`, as the typed methods do, instead of `Connect`.
- The `ok` reply body accepts keys the AsyncAPI document does not list.
- `ws_codegen.py` fails when a tagged AsyncAPI message has no Rust owner, when payloads merged into one variant disagree on the tag, or when an extra `ok` payload is not tagged `ok`. `ws_fields.json` also records enum wire values and deprecated flags, which the tests check against the generated types.
- WebSocket message types are generated from the pinned AsyncAPI document. Fields the document marks deprecated carry `#[deprecated]`. `SubscribeParams::user_filter` is `UserFilter`, and lifecycle `price_level_structure` is `PriceLevelStructure`, which keeps values outside the document as `Unknown`.
- Typed reads for the user-data timestamp, historical cutoff, account limits, endpoint costs, and a market order book. `get_signed_raw` accepts an origin or a `/trade-api/v2` base, requests `/trade-api/v2/...`, and signs that path. A query in the path is rejected.
- `CreateOrderV2::build` requires `OrderFlag` for `post_only`, `reduce_only`, and `cancel_order_on_pause`. `True` and `False` are sent. `VenueDefault` omits the field.
- Order and amend prices are written with four decimal places.
- WebSocket subscribe and receive cover `cfbenchmarks_value` and `cfbenchmarks_value_5hz`, including their index lists. `SubscriptionAction::IndexList` sends `update_subscription` with action `indexlist` on either channel.
- An order book snapshot accepts `yes_dollars` and `no_dollars` as names for the fixed-point ladders.
- `SignMiddleware` is public. Its signature uses the path with the query removed, so another crate can sign `GET /trade-api/ws/v2`.
- Generated `kalshi-api-openapi` 3.31.0 from the pinned OpenAPI document.
- Added the `kalshi-api` client: signing, response capture, fixed-point helpers, and order builders.
- Added a hand-written WebSocket client for the trading socket, including typed frames and AsyncAPI checks for the synthetic fixtures.
- Rejected WebSocket frames keep their original text. Typed messages follow the AsyncAPI field list, including keys that are required and may be null.
