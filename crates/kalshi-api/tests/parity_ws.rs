use kalshi_api::ws::{parse_incoming, ClientCommand};
use serde_json::Value;

#[test]
fn official_ws_examples_and_every_payload_roundtrip_without_field_loss() {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/parity/ws.json")).unwrap();
    assert_eq!(fixtures.len(), 143);
    let mut failures = Vec::new();
    for fixture in fixtures {
        let value = fixture["value"].clone();
        let actual = if value.get("cmd").is_some() {
            serde_json::from_value::<ClientCommand>(value.clone()).and_then(serde_json::to_value)
        } else {
            parse_incoming(&value.to_string()).and_then(serde_json::to_value)
        };
        match actual {
            Ok(actual) if actual == value => {}
            actual => failures.push(format!(
                "{} {}: {actual:?}; expected {value}",
                fixture["message"], fixture["case"]
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "{} WS failures.\n{}",
        failures.len(),
        failures
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
