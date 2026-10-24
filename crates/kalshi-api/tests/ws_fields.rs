#![allow(clippy::disallowed_methods)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use kalshi_api::parse_incoming;
use syn::Fields;

struct Scanned {
    owner: String,
    field: String,
    ty: String,
    default: bool,
    skip_none: bool,
    required_nullable: bool,
}

struct Row {
    owner: String,
    field: String,
    presence: String,
    schema: String,
    deprecated: bool,
    wire: Vec<String>,
}

fn type_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .iter()
            .map(|segment| {
                let args = match &segment.arguments {
                    syn::PathArguments::None => String::new(),
                    syn::PathArguments::AngleBracketed(args) => {
                        let parts: Vec<String> = args
                            .args
                            .iter()
                            .map(|arg| match arg {
                                syn::GenericArgument::Type(inner) => type_string(inner),
                                _ => "_".to_owned(),
                            })
                            .collect();
                        format!("<{}>", parts.join(","))
                    }
                    syn::PathArguments::Parenthesized(_) => "(...)".to_owned(),
                };
                format!("{}{args}", segment.ident)
            })
            .collect::<Vec<_>>()
            .join("::"),
        _ => panic!("unsupported type in ws/generated.rs"),
    }
}

fn serde_metas(field: &syn::Field) -> Vec<syn::Meta> {
    let mut metas = Vec::new();
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let syn::Meta::List(list) = &attr.meta else {
            continue;
        };
        let parsed = list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .unwrap_or_else(|err| panic!("serde attr on {:?}: {err}", field.ident));
        metas.extend(parsed);
    }
    metas
}

fn lit_str(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(text),
        ..
    }) = expr
    {
        Some(text.value())
    } else {
        None
    }
}

fn scan_fields(owner: &str, fields: &Fields, out: &mut Vec<Scanned>) {
    let Fields::Named(named) = fields else {
        return;
    };
    for field in &named.named {
        let metas = serde_metas(field);
        let mut default = false;
        let mut skip_none = false;
        let mut required_nullable = false;
        for meta in &metas {
            match meta {
                syn::Meta::Path(path) if path.is_ident("default") => default = true,
                syn::Meta::NameValue(item) if item.path.is_ident("skip_serializing_if") => {
                    skip_none = lit_str(&item.value).as_deref() == Some("Option::is_none");
                }
                syn::Meta::NameValue(item) if item.path.is_ident("deserialize_with") => {
                    required_nullable =
                        lit_str(&item.value).as_deref() == Some("required_nullable");
                }
                _ => {}
            }
        }
        out.push(Scanned {
            owner: owner.to_owned(),
            field: field.ident.as_ref().unwrap().to_string(),
            ty: type_string(&field.ty),
            default,
            skip_none,
            required_nullable,
        });
    }
}

fn scan_types() -> Vec<Scanned> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = ["src/ws/generated.rs"]
        .iter()
        .map(|path| fs::read_to_string(root.join(path)).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let file = syn::parse_file(&source).unwrap();
    let mut out = Vec::new();
    for item in file.items {
        match item {
            syn::Item::Struct(item) => {
                scan_fields(&item.ident.to_string(), &item.fields, &mut out);
            }
            syn::Item::Enum(item) => {
                for variant in item.variants {
                    let owner = format!("{}::{}", item.ident, variant.ident);
                    scan_fields(&owner, &variant.fields, &mut out);
                }
            }
            _ => {}
        }
    }
    out
}

fn load_rows() -> Vec<Row> {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tools/kalshi-sdk/ws_fields.json");
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let fields = value["fields"].as_array().unwrap();
    assert_eq!(value["count"].as_u64().unwrap() as usize, fields.len());
    fields
        .iter()
        .map(|row| Row {
            owner: row["owner"].as_str().unwrap().to_owned(),
            field: row["field"].as_str().unwrap().to_owned(),
            presence: row["presence"].as_str().unwrap().to_owned(),
            schema: row["schema"].as_str().unwrap().to_owned(),
            deprecated: row["deprecated"].as_bool().unwrap(),
            wire: row["enum"]
                .as_array()
                .map(|values| {
                    values
                        .iter()
                        .map(|value| value.as_str().unwrap().to_owned())
                        .collect()
                })
                .unwrap_or_default(),
        })
        .collect()
}

fn assert_ws_fields(scanned: &[Scanned], rows: &[Row]) {
    for row in rows {
        let found = scanned
            .iter()
            .find(|field| field.owner == row.owner && field.field == row.field)
            .unwrap_or_else(|| panic!("missing {}.{}", row.owner, row.field));
        match row.presence.as_str() {
            "required" => {
                assert!(
                    !found.ty.starts_with("Option<") && !found.default && !found.required_nullable,
                    "{}.{} is required but typed as {} (default={}, nullable={})",
                    row.owner,
                    row.field,
                    found.ty,
                    found.default,
                    found.required_nullable
                );
            }
            "optional" => {
                assert!(
                    found.ty.starts_with("Option<")
                        && found.default
                        && found.skip_none
                        && !found.required_nullable,
                    "{}.{} is optional but typed as {} (default={}, skip_none={}, nullable={})",
                    row.owner,
                    row.field,
                    found.ty,
                    found.default,
                    found.skip_none,
                    found.required_nullable
                );
            }
            "required_nullable" => {
                assert!(
                    found.ty.starts_with("Option<")
                        && found.required_nullable
                        && !found.default
                        && !found.skip_none,
                    "{}.{} must be present and nullable, typed as {} (default={}, skip_none={}, nullable={})",
                    row.owner,
                    row.field,
                    found.ty,
                    found.default,
                    found.skip_none,
                    found.required_nullable
                );
            }
            other => panic!("unknown presence {other}"),
        }
    }
    let owners: std::collections::BTreeSet<&str> =
        rows.iter().map(|row| row.owner.as_str()).collect();
    for field in scanned {
        if !owners.contains(field.owner.as_str()) {
            continue;
        }
        assert!(
            rows.iter()
                .any(|row| row.owner == field.owner && row.field == field.field),
            "extra field {}.{}",
            field.owner,
            field.field
        );
    }
}

fn fixture(schema: &str) -> serde_json::Value {
    let message = schema
        .split('.')
        .next()
        .unwrap()
        .strip_suffix("Payload")
        .unwrap();
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/parity/ws.json")).unwrap();
    cases
        .into_iter()
        .find(|case| case["message"] == message && case["case"] == "full")
        .unwrap_or_else(|| panic!("no complete fixture for {schema}"))["value"]
        .clone()
}

fn descend<'a>(root: &'a mut serde_json::Value, schema: &str) -> &'a mut serde_json::Value {
    let mut cursor = root;
    for segment in schema.split('.').skip(1) {
        let (key, array) = segment
            .strip_suffix("[]")
            .map(|key| (key, true))
            .unwrap_or((segment, false));
        cursor = cursor
            .get_mut(key)
            .unwrap_or_else(|| panic!("{schema} missing {key}"));
        if array {
            cursor = cursor
                .as_array_mut()
                .and_then(|items| items.get_mut(0))
                .unwrap_or_else(|| panic!("{schema} has no element"));
        }
    }
    cursor
}

fn pointer(schema: &str, field: &str) -> String {
    let mut parts = Vec::new();
    for segment in schema.split('.').skip(1) {
        let key = segment.strip_suffix("[]").unwrap_or(segment);
        parts.push(key.to_owned());
        if segment.ends_with("[]") {
            parts.push("0".to_owned());
        }
    }
    parts.push(field.to_owned());
    format!("/{}", parts.join("/"))
}

#[test]
fn typed_messages_match_the_asyncapi_field_list() {
    let scanned = scan_types();
    let rows = load_rows();
    assert!(rows.len() > 100, "field list is unexpectedly small");
    assert_ws_fields(&scanned, &rows);
}

#[test]
fn missing_required_nullable_attribute_is_rejected() {
    let mut scanned = scan_types();
    let field = scanned
        .iter_mut()
        .find(|field| field.owner == "UserOrderBody" && field.field == "created_time")
        .unwrap();
    field.required_nullable = false;
    field.default = true;
    let rows = load_rows();
    let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_ws_fields(&scanned, &rows);
    }));
    assert!(
        error.is_err(),
        "Option with default was accepted for a required key"
    );
}

#[test]
fn required_nullable_fields_reject_a_missing_key_and_keep_null() {
    let rows = load_rows();
    let nullable: Vec<&Row> = rows
        .iter()
        .filter(|row| row.presence == "required_nullable")
        .collect();
    assert!(!nullable.is_empty());
    for row in nullable {
        let base = fixture(&row.schema);
        let mut missing = base.clone();
        let removed = descend(&mut missing, &row.schema)
            .as_object_mut()
            .unwrap()
            .remove(&row.field);
        assert!(
            removed.is_some(),
            "{}.{} was already absent",
            row.schema,
            row.field
        );
        assert!(
            parse_incoming(&missing.to_string()).is_err(),
            "{}.{} missing key was accepted",
            row.owner,
            row.field
        );

        let mut null = base;
        descend(&mut null, &row.schema)[&row.field] = serde_json::Value::Null;
        let parsed = parse_incoming(&null.to_string())
            .unwrap_or_else(|err| panic!("{}.{} null rejected: {err}", row.owner, row.field));
        let back = serde_json::to_value(&parsed).unwrap();
        assert_eq!(
            back.pointer(&pointer(&row.schema, &row.field)),
            Some(&serde_json::Value::Null),
            "{}.{}",
            row.owner,
            row.field
        );
    }
}

fn generated_file() -> syn::File {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ws/generated.rs");
    syn::parse_file(&fs::read_to_string(path).unwrap()).unwrap()
}

/// (owner, field) → (has `#[deprecated]`, innermost type name).
fn field_markers(file: &syn::File) -> BTreeMap<(String, String), (bool, String)> {
    fn scan(owner: &str, fields: &Fields, out: &mut BTreeMap<(String, String), (bool, String)>) {
        let Fields::Named(named) = fields else {
            return;
        };
        for field in &named.named {
            let deprecated = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("deprecated"));
            let mut ty = type_string(&field.ty);
            while let Some(inner) = ty
                .strip_prefix("Option<")
                .or_else(|| ty.strip_prefix("Vec<"))
                .and_then(|rest| rest.strip_suffix('>'))
            {
                ty = inner.to_owned();
            }
            let name = field.ident.as_ref().unwrap().to_string();
            out.insert((owner.to_owned(), name), (deprecated, ty));
        }
    }
    let mut out = BTreeMap::new();
    for item in &file.items {
        match item {
            syn::Item::Struct(item) => scan(&item.ident.to_string(), &item.fields, &mut out),
            syn::Item::Enum(item) => {
                for variant in &item.variants {
                    scan(
                        &format!("{}::{}", item.ident, variant.ident),
                        &variant.fields,
                        &mut out,
                    );
                }
            }
            _ => {}
        }
    }
    out
}

/// Enum name → wire names it reads. Closed enums use `#[serde(rename)]`;
/// open enums list their names in the `Deserialize` match.
fn enum_wire_names(file: &syn::File) -> BTreeMap<String, BTreeSet<String>> {
    struct Arms<'a>(&'a mut BTreeSet<String>);
    impl<'ast> syn::visit::Visit<'ast> for Arms<'_> {
        fn visit_arm(&mut self, arm: &'ast syn::Arm) {
            if let syn::Pat::Lit(syn::PatLit {
                lit: syn::Lit::Str(text),
                ..
            }) = &arm.pat
            {
                self.0.insert(text.value());
            }
            syn::visit::visit_arm(self, arm);
        }
    }
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for item in &file.items {
        match item {
            syn::Item::Enum(item) => {
                let names = out.entry(item.ident.to_string()).or_default();
                for variant in &item.variants {
                    for attr in &variant.attrs {
                        if !attr.path().is_ident("serde") {
                            continue;
                        }
                        let syn::Meta::List(list) = &attr.meta else {
                            continue;
                        };
                        let metas = list
                            .parse_args_with(
                                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                            )
                            .unwrap();
                        for meta in metas {
                            if let syn::Meta::NameValue(item) = meta {
                                if item.path.is_ident("rename") {
                                    names.insert(lit_str(&item.value).unwrap());
                                }
                            }
                        }
                    }
                }
            }
            syn::Item::Impl(item) => {
                let is_deserialize = item.trait_.as_ref().is_some_and(|(_, path, _)| {
                    path.segments
                        .last()
                        .is_some_and(|s| s.ident == "Deserialize")
                });
                if !is_deserialize {
                    continue;
                }
                let name = type_string(&item.self_ty);
                let names = out.entry(name).or_default();
                syn::visit::Visit::visit_item_impl(&mut Arms(names), item);
            }
            _ => {}
        }
    }
    out
}

fn assert_wire_and_deprecated(file: &syn::File, rows: &[Row]) {
    let markers = field_markers(file);
    let enums = enum_wire_names(file);
    let mut deprecated: BTreeMap<(String, String), bool> = BTreeMap::new();
    for row in rows {
        *deprecated
            .entry((row.owner.clone(), row.field.clone()))
            .or_default() |= row.deprecated;
        if row.wire.is_empty() {
            continue;
        }
        let (_, ty) = markers
            .get(&(row.owner.clone(), row.field.clone()))
            .unwrap_or_else(|| panic!("missing {}.{}", row.owner, row.field));
        let names = enums
            .get(ty)
            .unwrap_or_else(|| panic!("{}.{} is {ty}, not a generated enum", row.owner, row.field));
        for value in &row.wire {
            assert!(
                names.contains(value),
                "{}.{}: {ty} does not read wire name {value:?} ({})",
                row.owner,
                row.field,
                row.schema
            );
        }
    }
    for (key, spec) in &deprecated {
        let (rust, _) = markers
            .get(key)
            .unwrap_or_else(|| panic!("missing {}.{}", key.0, key.1));
        assert_eq!(
            rust, spec,
            "{}.{}: #[deprecated] is {rust}, the AsyncAPI document says {spec}",
            key.0, key.1
        );
    }
}

#[test]
fn enum_wire_names_and_deprecated_markers_match_the_asyncapi_field_list() {
    let rows = load_rows();
    assert!(rows.iter().filter(|row| row.deprecated).count() >= 10);
    assert!(rows.iter().filter(|row| !row.wire.is_empty()).count() >= 20);
    assert_wire_and_deprecated(&generated_file(), &rows);
}

#[test]
fn closed_enum_wire_names_round_trip() {
    use kalshi_api::ws::{LifecycleEvent, MarketSide, UserOrderStatus};
    for (value, wire) in [
        (UserOrderStatus::Resting, "resting"),
        (UserOrderStatus::Canceled, "canceled"),
        (UserOrderStatus::Executed, "executed"),
    ] {
        assert_eq!(serde_json::to_value(value).unwrap(), wire);
        assert_eq!(
            serde_json::from_value::<UserOrderStatus>(serde_json::json!(wire)).unwrap(),
            value
        );
    }
    assert_eq!(serde_json::to_value(MarketSide::Yes).unwrap(), "yes");
    assert_eq!(
        serde_json::to_value(LifecycleEvent::Created).unwrap(),
        "created"
    );
}

#[test]
fn ok_reply_with_an_unlisted_key_still_parses() {
    use kalshi_api::ws::{Incoming, OkBody};
    let markets = r#"{"type":"ok","id":1,"msg":{"market_tickers":["A"],"note":"x"}}"#;
    match parse_incoming(markets).unwrap() {
        Incoming::Ok {
            msg: Some(OkBody::Markets(body)),
            ..
        } => assert_eq!(body.market_tickers.as_deref(), Some(&["A".to_owned()][..])),
        other => panic!("extra key changed the ok reply: {other:?}"),
    }
    let cases = [
        (
            r#"{"type":"ok","id":1,"sid":1,"seq":1,"msg":{"index_ids":["I"]}}"#,
            "Indices",
        ),
        (
            r#"{"type":"ok","id":1,"sid":1,"seq":1,"msg":{"underlying_tickers":["U"]}}"#,
            "Underlyings",
        ),
        (
            r#"{"type":"ok","id":3,"msg":[{"channel":"ticker","sid":2}]}"#,
            "Subscriptions",
        ),
        (r#"{"type":"ok","id":3,"msg":[]}"#, "Subscriptions"),
        (
            r#"{"type":"ok","id":1,"msg":{"market_tickers":["A"]}}"#,
            "Markets",
        ),
        (r#"{"type":"ok","id":1,"msg":{}}"#, "Markets"),
    ];
    for (frame, arm) in cases {
        let got = match parse_incoming(frame).unwrap() {
            Incoming::Ok {
                msg: Some(body), ..
            } => match body {
                OkBody::Indices(_) => "Indices",
                OkBody::Underlyings(_) => "Underlyings",
                OkBody::Subscriptions(_) => "Subscriptions",
                OkBody::Markets(_) => "Markets",
            },
            other => panic!("{frame}: {other:?}"),
        };
        assert_eq!(got, arm, "{frame}");
    }
}
