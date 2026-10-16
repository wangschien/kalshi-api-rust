#![allow(clippy::disallowed_methods)]

use std::fs;
use std::path::Path;

use syn::visit::{self, Visit};
use syn::{Expr, ExprCall, ItemFn};

#[test]
fn hand_written_sources_do_not_default_generated_types() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    walk_dir(&root, &mut hits);
    assert!(hits.is_empty(), "{hits:?}");
}

#[test]
fn mutant_default_builder_is_rejected() {
    let source = r#"
        fn bad() {
            let _ = kalshi_api_openapi::models::CreateOrderV2Request {
                ticker: String::new(),
                ..Default::default()
            };
        }
    "#;
    let hits = scan_source(source);
    assert!(
        !hits.is_empty(),
        "mutant using Default::default was accepted"
    );
}

#[test]
fn clippy_toml_lists_every_generated_request() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let clippy = fs::read_to_string(manifest.join("clippy.toml")).unwrap();
    let models = manifest.join("../kalshi-api-openapi/src/models");
    let mut found = 0;
    for entry in fs::read_dir(models).unwrap() {
        let entry = entry.unwrap();
        let text = fs::read_to_string(entry.path()).unwrap();
        for line in text.lines() {
            let Some(rest) = line.strip_prefix("pub struct ") else {
                continue;
            };
            let name = rest.split([' ', '{', '<']).next().unwrap_or("");
            if name.ends_with("Request") {
                found += 1;
                let needle = format!("kalshi_api_openapi::models::{name}::default");
                assert!(clippy.contains(&needle), "missing {needle}");
            }
        }
    }
    assert!(found > 0);
    let src = fs::read_to_string(manifest.join("src/lib.rs")).unwrap()
        + &fs::read_dir(manifest.join("src"))
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("rs"))
            .map(|entry| fs::read_to_string(entry.path()).unwrap())
            .collect::<String>();
    assert!(!src.contains("block_on"), "blocking runtime call in src");
}

fn walk_dir(dir: &Path, hits: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, hits);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let text = fs::read_to_string(&path).unwrap();
            for hit in scan_source(&text) {
                hits.push(format!("{}: {hit}", path.display()));
            }
        }
    }
}

#[test]
fn imported_generated_default_is_rejected() {
    let source = r#"
        use kalshi_api_openapi::models::CreateOrderV2Request;
        fn bad() {
            let _ = CreateOrderV2Request::default();
        }
    "#;
    assert!(
        !scan_source(source).is_empty(),
        "imported CreateOrderV2Request::default was accepted"
    );
    let aliased = r#"
        use kalshi_api_openapi::models::CreateOrderV2Request as Req;
        fn bad() {
            let _ = Req::default();
            let _ = <Req as Default>::default();
            let _explicit: Req = Default::default();
        }
    "#;
    let hits = scan_source(aliased);
    assert!(hits.len() >= 3, "{hits:?}");
}

fn scan_source(source: &str) -> Vec<String> {
    let Ok(file) = syn::parse_file(source) else {
        return vec!["unparsed source".to_owned()];
    };
    let mut visitor = DefaultVisitor {
        hits: Vec::new(),
        imports: std::collections::HashMap::new(),
    };
    visitor.visit_file(&file);
    visitor.hits
}

struct DefaultVisitor {
    hits: Vec<String>,
    imports: std::collections::HashMap<String, String>,
}

impl DefaultVisitor {
    fn generated(&self, path: &str) -> bool {
        if path.contains("kalshi_api_openapi") {
            return true;
        }
        self.imports
            .get(path)
            .is_some_and(|full| full.contains("kalshi_api_openapi"))
    }

    fn resolve_type(&self, segments: &[String]) -> String {
        if segments.len() == 1 {
            if let Some(full) = self.imports.get(&segments[0]) {
                return full.clone();
            }
        }
        segments.join("::")
    }
}

impl<'ast> Visit<'ast> for DefaultVisitor {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        collect_use(&node.tree, &[], &mut self.imports);
        visit::visit_item_use(self, node);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if self.call_defaults_generated(expr) {
            self.hits.push("default on a generated type".to_owned());
        }
        if let Expr::Struct(item) = expr {
            let struct_path =
                path_segments(item.path.segments.iter().map(|seg| seg.ident.to_string()));
            let resolved = self.resolve_type(&struct_path);
            if self.generated(&resolved) {
                if let Some(rest) = &item.rest {
                    if is_default_call(rest) {
                        self.hits
                            .push("struct update uses Default::default".to_owned());
                    }
                }
            }
        }
        if let Expr::Let(_) = expr {
            // handled via local
        }
        visit::visit_expr(self, expr);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        if let syn::Pat::Type(pat) = &node.pat {
            if let Some(init) = &node.init {
                if is_default_call(&init.expr) && self.type_is_generated(&pat.ty) {
                    self.hits
                        .push("Default::default typed as a generated type".to_owned());
                }
            }
        }
        visit::visit_local(self, node);
    }

    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        visit::visit_item_fn(self, item);
    }
}

impl DefaultVisitor {
    fn call_defaults_generated(&self, expr: &Expr) -> bool {
        let Expr::Call(ExprCall { func, .. }) = expr else {
            return false;
        };
        let Expr::Path(path) = func.as_ref() else {
            return false;
        };
        let segments: Vec<String> = path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        if segments.last().is_none_or(|name| name != "default") {
            return false;
        }
        if let Some(qself) = &path.qself {
            return self.type_is_generated(&qself.ty);
        }
        if segments.len() < 2 {
            return false;
        }
        let type_path = self.resolve_type(&segments[..segments.len() - 1]);
        self.generated(&type_path)
    }

    fn type_is_generated(&self, ty: &syn::Type) -> bool {
        let syn::Type::Path(path) = ty else {
            return false;
        };
        let segments: Vec<String> = path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        self.generated(&self.resolve_type(&segments))
    }
}

fn path_segments(parts: impl Iterator<Item = String>) -> Vec<String> {
    parts.collect()
}

fn is_default_call(expr: &Expr) -> bool {
    let Expr::Call(ExprCall { func, .. }) = expr else {
        return false;
    };
    let Expr::Path(path) = func.as_ref() else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "default")
}

fn collect_use(
    tree: &syn::UseTree,
    prefix: &[String],
    imports: &mut std::collections::HashMap<String, String>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            let mut next = prefix.to_vec();
            next.push(path.ident.to_string());
            collect_use(&path.tree, &next, imports);
        }
        syn::UseTree::Name(name) => {
            let mut full = prefix.to_vec();
            full.push(name.ident.to_string());
            imports.insert(name.ident.to_string(), full.join("::"));
        }
        syn::UseTree::Rename(name) => {
            let mut full = prefix.to_vec();
            full.push(name.ident.to_string());
            imports.insert(name.rename.to_string(), full.join("::"));
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use(item, prefix, imports);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}
