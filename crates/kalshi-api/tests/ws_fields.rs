#![allow(clippy::disallowed_methods)]

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
