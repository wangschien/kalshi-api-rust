# Changelog

## 3.34.0

- Require caller-supplied connection and request deadlines for generated REST configuration; remove its unbounded `Default`.
- Require a WebSocket send deadline and report `WsEvent::SendTimedOut` when a write stalls, ending the session without resending.

- Align 118 REST operations and 207 models with the official Python async SDK 3.34.0. Keep the current OpenAPI's two additional optional event filters as documented extensions.
- Preserve JSON numbers and Python model defaults. Match media headers and omitted request bodies.
- Add Ed25519 signing, remaining WebSocket channels and payloads, sending timestamps, subscription updates, and client control Ping.
- Add explicit server-error retry middleware, offline cross-language fixtures, and a demo read-only example.

## [Unreleased]

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
- A dropped WebSocket frame ends the stream with `WsEvent::Lost`. Rejected frames keep their original text. Typed messages follow the AsyncAPI field list, including keys that are required and may be null.
- A full WebSocket application channel is the channel's own full result. Emitting `WsEvent::Lost` closes the socket without waiting for another frame.
