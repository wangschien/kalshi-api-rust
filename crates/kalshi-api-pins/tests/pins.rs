//! Generated-type pins. Fixed-point fields stay strings, exchange-index
//! optionality follows the model, queue position is a required string, and
//! subaccount number stays optional.
#![forbid(unsafe_code)]

use std::fs;
use std::path::PathBuf;

use kalshi_api_openapi::models::{
    CreateOrderV2Request, Fill, GetOrderQueuePositionResponse, IncentiveProgram, Market,
    MarketPosition, Order,
};

fn fill_exchange_index(model: &Fill) -> &i64 {
    &model.exchange_index
}

fn fill_subaccount_number(model: &Fill) -> &Option<Option<i64>> {
    &model.subaccount_number
}

fn market_position_exchange_index(model: &MarketPosition) -> &i64 {
    &model.exchange_index
}

fn order_exchange_index(model: &Order) -> &Option<i64> {
    &model.exchange_index
}

fn create_order_v2_exchange_index(model: &CreateOrderV2Request) -> &Option<i64> {
    &model.exchange_index
}

fn queue_position_fp(model: &GetOrderQueuePositionResponse) -> &String {
    &model.queue_position_fp
}

fn market_settlement_timer_seconds(model: &Market) -> &i64 {
    &model.settlement_timer_seconds
}

fn incentive_discount_factor_bps(model: &IncentiveProgram) -> &Option<Option<i32>> {
    &model.discount_factor_bps
}

struct Field {
    owner: String,
    wire: String,
    ty: String,
}

fn models_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../kalshi-api-openapi/src/models")
}

fn parse_models(text: &str) -> Vec<Field> {
    let mut fields = Vec::new();
    let mut owner: Option<String> = None;
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("pub struct ") {
            let name = rest
                .split([' ', '{', '<'])
                .next()
                .filter(|name| !name.is_empty());
            owner = name.map(str::to_string);
            continue;
        }
        if trimmed.starts_with("pub enum ") || trimmed.starts_with("impl ") {
            owner = None;
            continue;
        }
        if !trimmed.starts_with("#[serde(") {
            continue;
        }
        let mut attr = trimmed.to_string();
        while !attr.contains(")]") {
            let Some(next) = lines.next() else {
                break;
            };
            attr.push(' ');
            attr.push_str(next.trim());
        }
        let Some(field_line) = lines.next() else {
            break;
        };
        let field_line = field_line.trim();
        let Some(rest) = field_line.strip_prefix("pub ") else {
            continue;
        };
        let Some((ident, ty)) = rest.split_once(':') else {
            continue;
        };
        let ident = ident.trim();
        let ty = ty.trim().trim_end_matches(',').trim();
        let wire =
            serde_rename(&attr).unwrap_or_else(|| ident.trim_start_matches("r#").to_string());
        if let Some(owner) = &owner {
            fields.push(Field {
                owner: owner.clone(),
                wire,
                ty: ty.to_string(),
            });
        }
    }
    fields
}

fn serde_rename(attr: &str) -> Option<String> {
    let marker = "rename = \"";
    let start = attr.find(marker)? + marker.len();
    let end = attr[start..].find('"')? + start;
    Some(attr[start..end].to_string())
}

fn is_fixed_point(wire: &str) -> bool {
    wire.ends_with("_dollars")
        || wire.ends_with("_fp")
        || wire == "fee_cost"
        || wire.ends_with("_fee_cost")
}

fn is_fixed_point_type(ty: &str) -> bool {
    let numeric = ty
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|token| {
            matches!(
                token,
                "f64" | "f32" | "i64" | "i32" | "i16" | "u64" | "u32" | "u16" | "usize" | "isize"
            )
        });
    if numeric {
        return false;
    }
    ty.contains("String") || ty.contains("models::")
}

fn load_fields() -> Vec<Field> {
    let mut fields = Vec::new();
    for entry in fs::read_dir(models_dir()).expect("models directory") {
        let entry = entry.expect("model entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("model source");
        fields.extend(parse_models(&text));
    }
    fields
}

const JSON_NUMBER_FIELDS: &[(&str, &str)] = &[
    ("EventData", "fee_multiplier_override"),
    ("EventFeeChange", "fee_multiplier_override"),
    ("GetMarketIndexResponse", "value"),
    ("Market", "cap_strike"),
    ("Market", "floor_strike"),
    ("MarketIndexLatest", "value"),
    ("MarketIndexTimeseriesPoint", "v"),
    ("PercentilePoint", "numerical_forecast"),
    ("PercentilePoint", "raw_numerical_forecast"),
    ("Series", "fee_multiplier"),
    ("SeriesFeeChange", "fee_multiplier"),
    ("WeatherIndexCalibration", "city_reference_c"),
    ("WeatherIndexCalibrationStation", "offset_c"),
    ("WeatherIndexCalibrationStation", "weight"),
    ("WeatherIndexPoint", "v"),
    ("WeatherIndexStationReading", "temp_f"),
];

#[test]
fn named_type_pins_compile() {
    let _ = fill_exchange_index as fn(&Fill) -> &i64;
    let _ = fill_subaccount_number as fn(&Fill) -> &Option<Option<i64>>;
    let _ = market_position_exchange_index as fn(&MarketPosition) -> &i64;
    let _ = order_exchange_index as fn(&Order) -> &Option<i64>;
    let _ = create_order_v2_exchange_index as fn(&CreateOrderV2Request) -> &Option<i64>;
    let _ = queue_position_fp as fn(&GetOrderQueuePositionResponse) -> &String;
    let _ = market_settlement_timer_seconds as fn(&Market) -> &i64;
    let _ = incentive_discount_factor_bps as fn(&IncentiveProgram) -> &Option<Option<i32>>;
}

#[test]
fn source_pins() {
    let fields = load_fields();
    assert!(
        fields.iter().any(|field| field.owner == "Fill"
            && field.wire == "exchange_index"
            && field.ty == "i64"),
        "scanner missed Fill.exchange_index"
    );

    let mut number_fields = Vec::new();
    for field in &fields {
        assert!(
            !field.ty.contains("f64") && !field.ty.contains("f32"),
            "{}.{} must preserve its JSON number",
            field.owner,
            field.wire
        );
        if field.ty.contains("serde_json::Number") {
            number_fields.push((field.owner.as_str(), field.wire.as_str()));
        }
        if is_fixed_point(&field.wire) {
            assert!(
                is_fixed_point_type(&field.ty),
                "{} . {} has type {}",
                field.owner,
                field.wire,
                field.ty
            );
        }
        if field.wire == "exchange_index" {
            assert!(
                field.ty == "i64" || field.ty == "Option<i64>",
                "{} . exchange_index has type {}",
                field.owner,
                field.ty
            );
            if field.owner == "Fill" || field.owner == "MarketPosition" {
                assert_eq!(field.ty, "i64");
            }
            if field.owner == "Order" || field.owner.ends_with("Request") {
                assert_eq!(field.ty, "Option<i64>", "{}", field.owner);
            }
        }
        if field.owner == "Fill" && field.wire == "subaccount_number" {
            assert_eq!(field.ty, "Option<Option<i64>>");
        }
        if field.owner == "GetOrderQueuePositionResponse" && field.wire == "queue_position_fp" {
            assert_eq!(field.ty, "String");
        }
    }

    number_fields.sort_unstable();
    let mut expected: Vec<(&str, &str)> = JSON_NUMBER_FIELDS.to_vec();
    expected.sort_unstable();
    assert_eq!(number_fields, expected);
    assert_fixed_point_fields(&fields);
}

#[test]
fn fixed_point_integer_mutant_is_rejected() {
    let mut fields = load_fields();
    let field = fields
        .iter_mut()
        .find(|field| field.owner == "AmendOrderV2Response" && field.wire == "fill_count")
        .expect("AmendOrderV2Response.fill_count");
    field.ty = "Option<i64>".to_owned();
    let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_fixed_point_fields(&fields);
    }));
    assert!(error.is_err(), "Option<i64> fixed-point field was accepted");
}

fn assert_fixed_point_fields(fields: &[Field]) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/kalshi-sdk/fixed_point_fields.json");
    let text = fs::read_to_string(path).expect("fixed-point list");
    let value: serde_json::Value = serde_json::from_str(&text).expect("fixed-point json");
    let listed = value["fields"].as_array().expect("fields");
    assert_eq!(value["count"].as_u64().unwrap() as usize, listed.len());
    assert!(!listed.is_empty(), "the fixed-point field census is empty");
    for row in listed {
        let owner = row["struct"].as_str().unwrap();
        let wire = row["field"].as_str().unwrap();
        let found = fields
            .iter()
            .find(|field| field.owner == owner && field.wire == wire)
            .unwrap_or_else(|| panic!("missing {owner}.{wire}"));
        assert!(
            matches!(
                found.ty.as_str(),
                "String" | "Option<String>" | "Option<Option<String>>"
            ),
            "{owner}.{wire} has type {}",
            found.ty
        );
    }
}
