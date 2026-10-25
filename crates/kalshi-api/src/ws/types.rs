use super::generated::Incoming;

pub fn parse_incoming(text: &str) -> Result<Incoming, serde_json::Error> {
    serde_json::from_str(text)
}

#[cfg(test)]
mod tests {
    use crate::ws::{parse_incoming, Incoming, LastUpdateReason, UserOrderStatus};

    #[test]
    fn wire_unknown_status_is_a_known_variant() {
        let frame = r#"{"type":"user_order","sid":1,"msg":{"order_id":"ord-synthetic-1","user_id":"user-synthetic","ticker":"SYNTH-1","exchange_index":0,"status":"unknown","side":"yes","is_yes":true,"outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.5600","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.000000","maker_fill_cost_dollars":"0.000000","taker_fees_dollars":"0.000000","maker_fees_dollars":"0.000000","client_order_id":"cid-synthetic","created_time":null,"created_ts_ms":null,"last_update_reason":"Trade"}}"#;
        match parse_incoming(frame).unwrap() {
            Incoming::UserOrder { msg, .. } => {
                assert_eq!(msg.status, UserOrderStatus::Unknown);
                assert_eq!(msg.last_update_reason, Some(LastUpdateReason::Trade));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn new_user_order_status_fails_the_frame() {
        let frame = r#"{"type":"user_order","sid":1,"msg":{"order_id":"ord-synthetic-1","user_id":"user-synthetic","ticker":"SYNTH-1","exchange_index":0,"status":"weird","side":"yes","is_yes":true,"outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.5600","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.000000","maker_fill_cost_dollars":"0.000000","taker_fees_dollars":"0.000000","maker_fees_dollars":"0.000000","client_order_id":"cid-synthetic","created_time":null,"created_ts_ms":null}}"#;
        assert!(parse_incoming(frame).is_err());
    }

    #[test]
    fn new_last_update_reason_is_kept() {
        let frame = r#"{"type":"user_order","sid":1,"msg":{"order_id":"ord-synthetic-1","user_id":"user-synthetic","ticker":"SYNTH-1","exchange_index":0,"status":"resting","side":"yes","is_yes":true,"outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.5600","fill_count_fp":"0.00","remaining_count_fp":"1.00","initial_count_fp":"1.00","taker_fill_cost_dollars":"0.000000","maker_fill_cost_dollars":"0.000000","taker_fees_dollars":"0.000000","maker_fees_dollars":"0.000000","client_order_id":"cid-synthetic","created_time":null,"created_ts_ms":null,"last_update_reason":"NotInTheList"}}"#;
        match parse_incoming(frame).unwrap() {
            Incoming::UserOrder { msg, .. } => {
                assert_eq!(
                    msg.last_update_reason,
                    Some(LastUpdateReason::Unknown("NotInTheList".to_owned()))
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn orderbook_clock_is_kept_when_sent_and_none_when_absent() {
        let snapshot = r#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"SYNTH-1","market_id":"m","clock":4182057}}"#;
        match parse_incoming(snapshot).unwrap() {
            Incoming::OrderbookSnapshot { msg, .. } => assert_eq!(msg.clock, Some(4182057)),
            other => panic!("unexpected {other:?}"),
        }
        let delta = r#"{"type":"orderbook_delta","sid":1,"seq":2,"msg":{"market_ticker":"SYNTH-1","market_id":"m","clock":4182061,"price_dollars":"0.5600","delta_fp":"1.00","side":"yes"}}"#;
        match parse_incoming(delta).unwrap() {
            Incoming::OrderbookDelta { msg, .. } => assert_eq!(msg.clock, Some(4182061)),
            other => panic!("unexpected {other:?}"),
        }
        let snapshot = r#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"SYNTH-1","market_id":"m"}}"#;
        match parse_incoming(snapshot).unwrap() {
            Incoming::OrderbookSnapshot { msg, .. } => assert_eq!(msg.clock, None),
            other => panic!("unexpected {other:?}"),
        }
        let delta = r#"{"type":"orderbook_delta","sid":1,"seq":2,"msg":{"market_ticker":"SYNTH-1","market_id":"m","price_dollars":"0.5600","delta_fp":"1.00","side":"yes"}}"#;
        match parse_incoming(delta).unwrap() {
            Incoming::OrderbookDelta { msg, .. } => assert_eq!(msg.clock, None),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn old_orderbook_dollars_alias_is_kept() {
        let frame = r#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"SYNTH-1","market_id":"m","clock":0,"yes_dollars":[["0.5600","1.00"]],"no_dollars":[["0.4400","2.00"]]}}"#;
        match parse_incoming(frame).unwrap() {
            Incoming::OrderbookSnapshot { msg, .. } => {
                assert_eq!(
                    msg.yes_dollars_fp,
                    Some(vec![vec!["0.5600".to_owned(), "1.00".to_owned()]])
                );
                assert_eq!(
                    msg.no_dollars_fp,
                    Some(vec![vec!["0.4400".to_owned(), "2.00".to_owned()]])
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn cfbenchmarks_value_and_5hz_frames_parse() {
        let value = r#"{"type":"cfbenchmarks_value","sid":1,"seq":42,"msg":{"index_id":"BRTI","received_at":1710000000123,"data":"{\"type\":\"value\"}","avg_60s_data":{"value":"68000.12000000","window_size":3,"window_start_ts_ms":1709999940123,"window_end_ts_exclusive":1710000000123}}}"#;
        match parse_incoming(value).unwrap() {
            Incoming::CfBenchmarksValue { msg, .. } => assert_eq!(msg.index_id, "BRTI"),
            other => panic!("unexpected {other:?}"),
        }
        let list = r#"{"type":"cfbenchmarks_value_indexlist","id":2,"sid":1,"seq":1,"msg":{"index_ids":["BRTI","ETHUSD_RTI"]}}"#;
        match parse_incoming(list).unwrap() {
            Incoming::CfBenchmarksIndexList { id, msg, .. } => {
                assert_eq!(id, Some(2));
                assert_eq!(msg.index_ids, ["BRTI", "ETHUSD_RTI"]);
            }
            other => panic!("unexpected {other:?}"),
        }
        let fast = r#"{"type":"cfbenchmarks_value_5hz","sid":1,"seq":42,"msg":{"index_id":"BRTI","value_usd":"68000.12000000","source_ts_ms":1710000000323,"received_at":1710000000341,"data":"{\"type\":\"value\"}"}}"#;
        match parse_incoming(fast).unwrap() {
            Incoming::CfBenchmarksValue5Hz { msg, .. } => {
                assert_eq!(msg.value_usd, "68000.12000000")
            }
            other => panic!("unexpected {other:?}"),
        }
        let fast_list = r#"{"type":"cfbenchmarks_value_5hz_indexlist","sid":1,"seq":1,"msg":{"index_ids":["BRTI"]}}"#;
        match parse_incoming(fast_list).unwrap() {
            Incoming::CfBenchmarksIndexList5Hz { id, msg, .. } => {
                assert!(id.is_none());
                assert_eq!(msg.index_ids, ["BRTI"]);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
