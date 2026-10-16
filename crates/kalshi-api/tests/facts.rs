use kalshi_api::{
    parse_decimal, AmendCount, AmendOrderV2, CreateOrderV2, Decrease, DecreaseOrderV2,
    ExchangeIndexChoice, OrderFlag, QtyCenti, SubaccountChoice,
};
use kalshi_api_openapi::models::create_order_v2_request::TimeInForce;
use kalshi_api_openapi::models::{BookSide, SelfTradePreventionType};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn production_hosts_match_the_spec() {
    assert_eq!(
        kalshi_api::Environment::Prod.base_url(),
        "https://external-api.kalshi.com/trade-api/v2"
    );
    assert_eq!(
        kalshi_api::Environment::ProdShared.base_url(),
        "https://api.elections.kalshi.com/trade-api/v2"
    );
}

#[test]
fn exchange_index_query_fits_i32() {
    assert_eq!(kalshi_api::exchange_index_query(3).unwrap(), 3);
    assert!(kalshi_api::exchange_index_query(i64::from(i32::MAX) + 1).is_err());
}

#[test]
fn qty_centi_parses_two_decimals() {
    assert_eq!(QtyCenti::parse("10").unwrap().0, 1000);
    assert_eq!(QtyCenti::parse("10.0").unwrap().0, 1000);
    assert_eq!(QtyCenti::parse("2.50").unwrap().wire(), "2.50");
    assert!(QtyCenti::parse("2.501").is_err());
    assert!(QtyCenti::parse("nope").is_err());
    assert!(parse_decimal("0.5600").is_ok());
    assert!(parse_decimal("nope").is_err());
}

#[test]
fn amend_count_is_filled_plus_remaining() {
    let filled = QtyCenti::parse("1.25").unwrap();
    let remaining = QtyCenti::parse("0.50").unwrap();
    let count = AmendCount::from_filled_and_remaining(filled, remaining).unwrap();
    assert_eq!(count.as_wire(), "1.75");
}

#[test]
fn builder_serialises_the_given_side() {
    for (side, wire) in [(BookSide::Bid, "bid"), (BookSide::Ask, "ask")] {
        let order = CreateOrderV2::build(
            "SYNTH-1",
            side,
            QtyCenti::parse("1.00").unwrap(),
            Decimal::from_str("0.56").unwrap(),
            TimeInForce::GoodTillCanceled,
            SelfTradePreventionType::Maker,
            "cid-synthetic",
            ExchangeIndexChoice::Shard(0),
            SubaccountChoice::Primary,
            OrderFlag::VenueDefault,
            OrderFlag::VenueDefault,
            OrderFlag::VenueDefault,
        )
        .unwrap();
        let value = order.to_value().unwrap();
        assert_eq!(value["side"], wire);
        assert_eq!(value["count"], "1.00");
        assert_eq!(value["exchange_index"], 0);
        assert_eq!(value["client_order_id"], "cid-synthetic");
        assert_eq!(value["subaccount"], 0);
        assert_eq!(value["price"], "0.5600");
        assert!(value.get("post_only").is_none());
        assert!(value.get("reduce_only").is_none());
        assert!(value.get("cancel_order_on_pause").is_none());
    }
}

#[test]
fn create_order_sends_flags_and_four_decimal_price() {
    let order = CreateOrderV2::build(
        "SYNTH-1",
        BookSide::Bid,
        QtyCenti::parse("1.00").unwrap(),
        Decimal::from_str("0.30").unwrap(),
        TimeInForce::GoodTillCanceled,
        SelfTradePreventionType::Maker,
        "cid-synthetic",
        ExchangeIndexChoice::Omit,
        SubaccountChoice::Primary,
        OrderFlag::True,
        OrderFlag::False,
        OrderFlag::True,
    )
    .unwrap();
    let value = order.to_value().unwrap();
    assert_eq!(value["price"], "0.3000");
    assert_eq!(value["post_only"], true);
    assert_eq!(value["reduce_only"], false);
    assert_eq!(value["cancel_order_on_pause"], true);
}

#[test]
fn omit_exchange_index_is_an_explicit_choice() {
    let order = CreateOrderV2::build(
        "SYNTH-1",
        BookSide::Bid,
        QtyCenti::parse("1").unwrap(),
        Decimal::from_str("0.5600").unwrap(),
        TimeInForce::ImmediateOrCancel,
        SelfTradePreventionType::TakerAtCross,
        "cid-synthetic",
        ExchangeIndexChoice::Omit,
        SubaccountChoice::KeyRestricted,
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
        OrderFlag::VenueDefault,
    )
    .unwrap();
    let value = order.to_value().unwrap();
    assert!(value.get("exchange_index").is_none());
    assert!(
        value.get("subaccount").is_none(),
        "KeyRestricted must omit subaccount"
    );
}

#[test]
fn amend_count_is_the_only_wire_count() {
    let filled = QtyCenti::parse("1.25").unwrap();
    let remaining = QtyCenti::parse("0.50").unwrap();
    let count = AmendCount::from_filled_and_remaining(filled, remaining).unwrap();
    let amend = AmendOrderV2::build(
        "SYNTH-1",
        BookSide::Ask,
        Decimal::from_str("0.5700").unwrap(),
        count,
        "cid-synthetic",
        None,
        ExchangeIndexChoice::Omit,
    )
    .unwrap();
    let value = amend.to_value().unwrap();
    assert_eq!(value["count"], "1.75");
    assert_eq!(value["side"], "ask");
    assert!(value.get("exchange_index").is_none());
    assert!(value.get("updated_client_order_id").is_none());
    assert!(value.get("expiration_time").is_none());
    let expiry = amend.clone().with_expiration_time(0).to_value().unwrap();
    assert_eq!(expiry["expiration_time"], 0);
    assert_eq!(expiry["client_order_id"], "cid-synthetic");
    assert_eq!(expiry["count"], "1.75");
    assert_eq!(
        amend
            .with_expiration_time(1_800_000_000)
            .to_value()
            .unwrap()["expiration_time"],
        1_800_000_000
    );
}

#[test]
fn decrease_sets_exactly_one_field() {
    let by = DecreaseOrderV2::build(
        Decrease::By(QtyCenti::parse("1.00").unwrap()),
        ExchangeIndexChoice::Shard(-1),
        Some("SYNTH-1".to_owned()),
    )
    .unwrap();
    let by_value = by.to_value().unwrap();
    assert_eq!(by_value["reduce_by"], "1.00");
    assert!(by_value.get("reduce_to").is_none());
    assert_eq!(by_value["exchange_index"], -1);
    let to = DecreaseOrderV2::build(
        Decrease::To(QtyCenti::parse("0.00").unwrap()),
        ExchangeIndexChoice::Omit,
        None,
    )
    .unwrap();
    let to_value = to.to_value().unwrap();
    assert_eq!(to_value["reduce_to"], "0.00");
    assert!(to_value.get("reduce_by").is_none());
    assert!(DecreaseOrderV2::build(
        Decrease::By(QtyCenti::parse("-1.00").unwrap()),
        ExchangeIndexChoice::Omit,
        None,
    )
    .is_err());
}
