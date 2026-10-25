//! Order book frames as the exchange sends them. The AsyncAPI document marks
//! `clock` required, but production and demo frames carry no `clock` key.
//! The fixtures are one recorded production snapshot and one delta, wire frame
//! only. These checks read the frame through serde values, so a build that
//! types `clock` as required still compiles and fails here.

use kalshi_api::parse_incoming;
use kalshi_api::ws::Incoming;

const SNAPSHOT: &str = include_str!("fixtures/recorded/orderbook_snapshot.json");
const DELTA: &str = include_str!("fixtures/recorded/orderbook_delta.json");

fn parse(frame: &str) -> (Incoming, serde_json::Value) {
    let parsed = parse_incoming(frame.trim())
        .unwrap_or_else(|err| panic!("recorded frame rejected: {err}: {frame}"));
    let back = serde_json::to_value(&parsed).unwrap();
    (parsed, back)
}

#[test]
fn recorded_snapshot_without_clock_parses() {
    assert!(!SNAPSHOT.contains("\"clock\""));
    let (parsed, back) = parse(SNAPSHOT);
    assert!(
        matches!(parsed, Incoming::OrderbookSnapshot { .. }),
        "{parsed:?}"
    );
    assert_eq!(back.pointer("/msg/clock"), None);
    assert_eq!(back["msg"]["market_ticker"], "KXBTC15M-26OCT250945-45");
    assert_eq!(back["seq"], 1);
}

#[test]
fn recorded_delta_without_clock_parses() {
    assert!(!DELTA.contains("\"clock\""));
    let (parsed, back) = parse(DELTA);
    assert!(
        matches!(parsed, Incoming::OrderbookDelta { .. }),
        "{parsed:?}"
    );
    assert_eq!(back.pointer("/msg/clock"), None);
    assert_eq!(back["msg"]["price_dollars"], "0.0020");
    assert_eq!(back["msg"]["delta_fp"], "12000.00");
    assert_eq!(back["msg"]["ts_ms"], 1792935000000_i64);
}

#[test]
fn recorded_frames_with_a_clock_keep_it() {
    for (frame, clock) in [(SNAPSHOT, 4182057_i64), (DELTA, 4182061_i64)] {
        let mut value: serde_json::Value = serde_json::from_str(frame).unwrap();
        value["msg"]["clock"] = clock.into();
        let (_, back) = parse(&value.to_string());
        assert_eq!(back["msg"]["clock"], clock, "{value}");
    }
}
