#![allow(clippy::disallowed_methods)]

use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::Visit;
use syn::{FnArg, ImplItem, Item, ReturnType, Visibility};

#[derive(Clone)]
struct FnRec {
    owner: String,
    name: String,
    docs: String,
    params: Vec<String>,
    output: Vec<String>,
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn docs(attrs: &[syn::Attribute]) -> String {
    let mut parts = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        let Ok(meta) = attr.meta.require_name_value() else {
            continue;
        };
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(text),
            ..
        }) = &meta.value
        {
            parts.push(text.value());
        }
    }
    parts.join(" ")
}

fn type_names(ty: &syn::Type, owner: &str, out: &mut Vec<String>) {
    match ty {
        syn::Type::Reference(inner) => type_names(&inner.elem, owner, out),
        syn::Type::Slice(inner) => type_names(&inner.elem, owner, out),
        syn::Type::Paren(inner) => type_names(&inner.elem, owner, out),
        syn::Type::Group(inner) => type_names(&inner.elem, owner, out),
        syn::Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                type_names(elem, owner, out);
            }
        }
        syn::Type::Path(path) => {
            if let Some(segment) = path.path.segments.last() {
                let name = segment.ident.to_string();
                if name == "Self" {
                    out.push(owner.to_owned());
                } else {
                    out.push(name);
                }
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner) = arg {
                            type_names(inner, owner, out);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

struct Collector {
    owner: String,
    fns: Vec<FnRec>,
}

impl Collector {
    fn record(&mut self, sig: &syn::Signature, attrs: &[syn::Attribute], vis: &Visibility) {
        if !is_public(vis) {
            return;
        }
        let mut params = Vec::new();
        for input in &sig.inputs {
            if let FnArg::Typed(pat) = input {
                type_names(&pat.ty, &self.owner, &mut params);
            }
        }
        let mut output = Vec::new();
        if let ReturnType::Type(_, ty) = &sig.output {
            type_names(ty, &self.owner, &mut output);
        }
        self.fns.push(FnRec {
            owner: self.owner.clone(),
            name: sig.ident.to_string(),
            docs: docs(attrs),
            params,
            output,
        });
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        if item.trait_.is_some() {
            return;
        }
        let owner = match item.self_ty.as_ref() {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .unwrap_or_default(),
            _ => String::new(),
        };
        let previous = std::mem::replace(&mut self.owner, owner);
        for impl_item in &item.items {
            if let ImplItem::Fn(func) = impl_item {
                self.record(&func.sig, &func.attrs, &func.vis);
            }
        }
        self.owner = previous;
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let previous = std::mem::take(&mut self.owner);
        self.record(&item.sig, &item.attrs, &item.vis);
        self.owner = previous;
    }
}

fn parse_sources(root: &Path) -> (Vec<FnRec>, String) {
    let mut files = Vec::new();
    load_tree(&root.join("src/lib.rs"), &mut files);
    let mut collector = Collector {
        owner: String::new(),
        fns: Vec::new(),
    };
    let mut client_src = String::new();
    for (path, file) in &files {
        if path.ends_with("client.rs") {
            client_src = fs::read_to_string(path).unwrap();
        }
        collector.visit_file(file);
    }
    (collector.fns, client_src)
}

fn load_tree(path: &Path, out: &mut Vec<(PathBuf, syn::File)>) {
    let text = fs::read_to_string(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    let file = syn::parse_file(&text).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    for item in &file.items {
        let Item::Mod(module) = item else {
            continue;
        };
        if module.content.is_some() || cfg_test(&module.attrs) {
            continue;
        }
        let parent = path.parent().unwrap();
        let name = module.ident.to_string();
        let file_path = parent.join(format!("{name}.rs"));
        let next = if file_path.exists() {
            file_path
        } else {
            parent.join(&name).join("mod.rs")
        };
        load_tree(&next, out);
    }
    out.push((path.to_owned(), file));
}

fn cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .meta
                .require_list()
                .is_ok_and(|list| list.tokens.to_string() == "test")
    })
}

fn contains_sentence(docs: &str, sentence: &str) -> bool {
    normalize(docs).contains(&normalize(sentence))
}

fn surface_errors(fns: &[FnRec], client_src: &str, rows: &[serde_json::Value]) -> Vec<String> {
    let mut errors = Vec::new();
    for row in rows {
        let rust_name = row["rust_name"].as_str().unwrap();
        if !row["wrapped"].as_bool().unwrap() {
            let needle = format!("fn {rust_name}(");
            if client_src.contains(&needle) {
                errors.push(format!("{rust_name} is wrapped"));
            }
            continue;
        }
        let method = row["method"].as_str().unwrap();
        let param_type = row["param_type"].as_str().unwrap();
        let sentence = row["sentence"].as_str().unwrap();
        let Some(func) = fns
            .iter()
            .find(|func| func.owner == "Client" && func.name == method)
        else {
            errors.push(format!("missing {method}"));
            continue;
        };
        let carried = func.params.iter().any(|name| name == param_type)
            || func.params.iter().any(|owner| {
                fns.iter().any(|build| {
                    build.owner == *owner
                        && build.name == "build"
                        && build.params.iter().any(|name| name == param_type)
                })
            });
        if !carried {
            errors.push(format!("{method} does not carry {param_type}"));
        }
        if param_type == "SubaccountChoice" {
            let documented = func.params.iter().any(|owner| {
                fns.iter().any(|build| {
                    build.owner == *owner
                        && build.name == "build"
                        && build.params.iter().any(|name| name == param_type)
                        && contains_sentence(&build.docs, sentence)
                })
            });
            if !documented {
                errors.push(format!("{method} builder docs omit the spec sentence"));
            }
        }
        if !contains_sentence(&func.docs, sentence) {
            errors.push(format!("{method} docs omit the spec sentence"));
        }
        for sentence in row["exchange_index_sentences"].as_array().unwrap() {
            let sentence = sentence.as_str().unwrap();
            if !contains_sentence(&func.docs, sentence) {
                errors.push(format!("{method} docs omit the exchange index sentence"));
            }
        }
        let raw_name = format!("{method}_raw");
        let Some(raw) = fns
            .iter()
            .find(|func| func.owner == "Client" && func.name == raw_name)
        else {
            errors.push(format!("missing {raw_name}"));
            continue;
        };
        if !contains_sentence(&raw.docs, sentence) {
            errors.push(format!("{raw_name} docs omit the spec sentence"));
        }
        for sentence in row["exchange_index_sentences"].as_array().unwrap() {
            let sentence = sentence.as_str().unwrap();
            if !contains_sentence(&raw.docs, sentence) {
                errors.push(format!("{raw_name} docs omit the exchange index sentence"));
            }
        }
    }
    errors
}

fn forbidden_signatures(fns: &[FnRec]) -> Vec<String> {
    let mut hits = Vec::new();
    for func in fns {
        for name in func.params.iter().chain(func.output.iter()) {
            if name.ends_with("Request") || name.ends_with("Params") {
                hits.push(format!("{}::{} exposes {name}", func.owner, func.name));
            }
        }
    }
    hits
}

fn table() -> Vec<serde_json::Value> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/kalshi-sdk/subaccount_semantics.json");
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    value["rows"].as_array().unwrap().clone()
}

#[test]
fn typed_methods_match_the_spec_table() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (fns, client_src) = parse_sources(&root);
    let errors = surface_errors(&fns, &client_src, &table());
    assert!(errors.is_empty(), "{errors:?}");
}

#[test]
fn mutant_wrong_subaccount_type_is_rejected() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (mut fns, client_src) = parse_sources(&root);
    let changed = client_src.replacen("subaccount: SubaccountFilter", "subaccount: Subaccount", 1);
    assert_ne!(changed, client_src);
    let file = syn::parse_file(&changed).unwrap();
    fns.retain(|func| func.owner != "Client");
    let mut collector = Collector {
        owner: String::new(),
        fns,
    };
    collector.visit_file(&file);
    let errors = surface_errors(&collector.fns, &changed, &table());
    assert!(
        errors.iter().any(|err| err.contains("get_orders")),
        "wrong subaccount type was accepted: {errors:?}"
    );
}

#[test]
fn mutant_missing_spec_sentence_is_rejected() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (mut fns, client_src) = parse_sources(&root);
    let sentence = "If omitted, defaults to all subaccounts.";
    let changed = client_src.replace(sentence, "omission text removed.");
    assert_ne!(changed, client_src);
    let file = syn::parse_file(&changed).unwrap();
    fns.retain(|func| func.owner != "Client");
    let mut collector = Collector {
        owner: String::new(),
        fns,
    };
    collector.visit_file(&file);
    let errors = surface_errors(&collector.fns, &changed, &table());
    assert!(!errors.is_empty(), "deleting a spec sentence was accepted");
}

#[test]
fn public_signatures_hide_generated_requests_and_params() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (fns, _) = parse_sources(&root);
    let hits = forbidden_signatures(&fns);
    assert!(hits.is_empty(), "{hits:?}");
}

#[test]
fn mutant_generated_request_signature_is_rejected() {
    let source = r#"
        pub async fn bad(body: kalshi_api_openapi::models::AmendOrderV2Request) {}
        pub async fn also_bad(params: kalshi_api_openapi::apis::orders_api::GetOrdersParams) {}
    "#;
    let file = syn::parse_file(source).unwrap();
    let mut collector = Collector {
        owner: String::new(),
        fns: Vec::new(),
    };
    collector.visit_file(&file);
    let hits = forbidden_signatures(&collector.fns);
    assert!(
        hits.len() >= 2,
        "generated request and params signatures were accepted: {hits:?}"
    );
}

#[test]
fn amend_count_has_one_constructor() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (fns, _) = parse_sources(&root);
    let constructors: Vec<_> = fns
        .iter()
        .filter(|func| {
            func.owner == "AmendCount" && func.output.iter().any(|name| name == "AmendCount")
        })
        .map(|func| func.name.as_str())
        .collect();
    assert_eq!(constructors, ["from_filled_and_remaining"]);
}

#[test]
fn decrease_is_exactly_one_of_by_or_to() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/decrease.rs");
    let file = syn::parse_file(&fs::read_to_string(path).unwrap()).unwrap();
    let variants = file.items.iter().find_map(|item| match item {
        Item::Enum(item) if item.ident == "Decrease" => Some(
            item.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>(),
        ),
        _ => None,
    });
    assert_eq!(variants.unwrap(), ["By", "To"]);
}
