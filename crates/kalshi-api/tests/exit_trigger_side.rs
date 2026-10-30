//! Pin: EventContractExitTrigger.side wire values are the strings "yes" and "no".
use kalshi_api_openapi::models::event_contract_exit_trigger::{EventContractExitTrigger, Side};

fn sample(side: &str) -> String {
    format!(
        r#"{{"id":"t1","ticker":"TICK","side":"{side}","subaccount_number":0,"kind":"bracket","status":"active","created_time":"2026-01-01T00:00:00Z","updated_time":"2026-01-01T00:00:00Z"}}"#
    )
}

#[test]
fn exit_trigger_decodes_side_yes_and_no() {
    for (wire, variant) in [
        ("yes", Side::EventContractExitTriggerSideYes),
        ("no", Side::EventContractExitTriggerSideNo),
    ] {
        let parsed: EventContractExitTrigger = serde_json::from_str(&sample(wire))
            .unwrap_or_else(|e| panic!("decode side={wire:?}: {e}"));
        assert_eq!(parsed.side, variant, "decoded side for {wire}");
        let back = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(back["side"], wire, "serialize side for {wire}");
    }
}
