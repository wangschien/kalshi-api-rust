#!/usr/bin/env python3
"""Compare pinned Python models with the generated Rust models.

Field sets, required-versus-optional, and enum values are compared for
models whose names match case-insensitively. Explicit differences describe
only current-spec extensions absent from the pinned Python SDK.
"""

from __future__ import annotations

import argparse
import ast
import json
import re
import sys
from pathlib import Path

PROP_RE = re.compile(
    r"__properties:\s*ClassVar\[List\[str\]\]\s*=\s*\[(.*?)\]",
    re.S,
)
CLASS_RE = re.compile(r"^class\s+(\w+)\(([^)]*)\)\s*:", re.M)
ENUM_VALUE_RE = re.compile(r"^\s{4}(\w+)\s*=\s*(['\"])(.*)\2\s*$", re.M)
FIELD_RE = re.compile(
    r"#\[serde\((?P<meta>.*?)\)\]\s*"
    r"pub (?P<ident>r#[A-Za-z0-9_]+|[A-Za-z_][A-Za-z0-9_]*):\s*"
    r"(?P<ty>[^\n]+),",
    re.S,
)
RENAME_RE = re.compile(r'rename\s*=\s*"([^"]*)"')
ITEM_RE = re.compile(r"pub (struct|enum) (\w+)")


def parse_python(root: Path) -> dict[str, dict]:
    models_dir = root / "models"
    if not models_dir.is_dir():
        raise SystemExit(f"missing Python models directory: {models_dir}")
    models: dict[str, dict] = {}
    for path in sorted(models_dir.glob("*.py")):
        if path.name == "__init__.py":
            continue
        text = path.read_text()
        classes = CLASS_RE.findall(text)
        if len(classes) != 1:
            raise SystemExit(f"{path.name} has {len(classes)} classes")
        name, bases = classes[0]
        if bases.strip() == "str, Enum":
            values = [match[2] for match in ENUM_VALUE_RE.findall(text)]
            models[name] = {"kind": "enum", "values": values}
            continue
        if bases.strip() != "BaseModel":
            raise SystemExit(f"{path.name} has unsupported bases {bases}")
        found = PROP_RE.search(text)
        if not found:
            raise SystemExit(f"{path.name} has no __properties")
        fields = re.findall(r"['\"]([^'\"]+)['\"]", found.group(1))
        cls = next(node for node in ast.parse(text).body if isinstance(node, ast.ClassDef))
        assignments = {node.target.id: node for node in cls.body if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name)}
        required = []
        defaults = {}
        for field in fields:
            match = re.search(rf"^\s{{4}}{re.escape(field)}\s*:\s*(.+)$", text, re.M)
            if not match:
                raise SystemExit(f"{name}.{field} has no annotation")
            annotation = match.group(1)
            value = assignments[field].value
            optional = value is not None and (not isinstance(value, ast.Call) or any(kw.arg in {"default", "default_factory"} for kw in value.keywords))
            if isinstance(value, ast.Call):
                for keyword in value.keywords:
                    if keyword.arg == "default" and ast.literal_eval(keyword.value) is not None:
                        defaults[field] = ast.literal_eval(keyword.value)
            if not optional:
                required.append(field)
        enum_fields = {}
        for validator in cls.body:
            if not isinstance(validator, ast.FunctionDef):
                continue
            validated = [ast.literal_eval(arg) for decorator in validator.decorator_list if isinstance(decorator, ast.Call) and getattr(decorator.func, "id", "") == "field_validator" for arg in decorator.args]
            for test in ast.walk(validator):
                if not isinstance(test, ast.Compare) or not any(isinstance(op, ast.NotIn) for op in test.ops):
                    continue
                choices = test.comparators[0]
                if isinstance(choices, ast.Call) and getattr(choices.func, "id", "") == "set":
                    choices = choices.args[0]
                for field in validated:
                    enum_fields[field] = sorted(ast.literal_eval(choices))
        models[name] = {"kind": "struct", "fields": fields, "required": required, "enum_fields": enum_fields, "defaults": defaults}
    return models


def parse_rust(models_dir: Path) -> dict[str, dict]:
    models: dict[str, dict] = {}
    for path in sorted(models_dir.glob("*.rs")):
        if path.name == "mod.rs":
            continue
        text = path.read_text()
        items = list(ITEM_RE.finditer(text))
        if not items:
            raise SystemExit(f"{path.name} has no struct or enum")
        kind, name = items[0].group(1), items[0].group(2)
        if kind == "enum":
            body_end = items[1].start() if len(items) > 1 else len(text)
            body = text[items[0].end() : body_end]
            values = RENAME_RE.findall(body)
            models[name] = {"kind": "enum", "values": values}
            continue
        body_end = items[1].start() if len(items) > 1 else len(text)
        # Stop the struct scan at its impl so later enums in the file are ignored.
        impl_at = text.find(f"impl {name}", items[0].end())
        if impl_at != -1:
            body_end = min(body_end, impl_at)
        body = text[items[0].end() : body_end]
        fields = []
        required = []
        types = {}
        for match in FIELD_RE.finditer(body):
            rename = RENAME_RE.search(match.group("meta"))
            wire = rename.group(1) if rename else match.group("ident").removeprefix("r#")
            ty = match.group("ty").strip()
            fields.append(wire)
            types[wire] = ty
            if not ty.startswith("Option<") or "deserialize_with" in match.group("meta"):
                required.append(wire)
        enum_fields = {}
        for enum_name, enum_body in re.findall(r"pub enum (\w+) \{(.*?)\n\}", text, re.S):
            for field, ty in types.items():
                if re.search(rf"\b{re.escape(enum_name)}\b", ty):
                    enum_fields[field] = sorted(RENAME_RE.findall(enum_body))
        models[name] = {"kind": "struct", "fields": fields, "required": required, "types": types, "enum_fields": enum_fields}
    return models


def by_fold(models: dict[str, dict]) -> dict[str, tuple[str, dict]]:
    folded: dict[str, tuple[str, dict]] = {}
    for name, model in models.items():
        key = name.casefold()
        if key in folded:
            raise SystemExit(f"case-insensitive name collision: {folded[key][0]} and {name}")
        folded[key] = (name, model)
    return folded


def diff_report(python_models: dict[str, dict], rust_models: dict[str, dict]) -> dict:
    py = by_fold(python_models)
    rs = by_fold(rust_models)
    report: dict[str, dict] = {
        "field_python_only": {},
        "field_rust_only": {},
        "required_python_only": {},
        "required_rust_only": {},
        "enum_python_only": {},
        "enum_rust_only": {},
        "python_only_models": sorted(py[key][0] for key in py.keys() - rs.keys()),
        "rust_only_models": sorted(rs[key][0] for key in rs.keys() - py.keys()),
    }
    for key in sorted(py.keys() & rs.keys()):
        py_name, py_model = py[key]
        rs_name, rs_model = rs[key]
        label = rs_name if py_name == rs_name else f"{py_name}/{rs_name}"
        if py_model["kind"] != rs_model["kind"]:
            raise SystemExit(f"{label} kind {py_model['kind']} != {rs_model['kind']}")
        if py_model["kind"] == "enum":
            py_values = set(py_model["values"])
            rs_values = set(rs_model["values"])
            only_py = sorted(py_values - rs_values)
            only_rs = sorted(rs_values - py_values)
            if only_py:
                report["enum_python_only"][label] = only_py
            if only_rs:
                report["enum_rust_only"][label] = only_rs
            continue
        py_fields = set(py_model["fields"])
        rs_fields = set(rs_model["fields"])
        only_py = sorted(py_fields - rs_fields)
        only_rs = sorted(rs_fields - py_fields)
        if only_py:
            report["field_python_only"][label] = only_py
        if only_rs:
            report["field_rust_only"][label] = only_rs
        shared = py_fields & rs_fields
        req_py = sorted((set(py_model["required"]) & shared) - set(rs_model["required"]))
        req_rs = sorted((set(rs_model["required"]) & shared) - set(py_model["required"]))
        if req_py:
            report["required_python_only"][label] = req_py
        if req_rs:
            report["required_rust_only"][label] = req_rs
        for field in py_model["enum_fields"].keys() | rs_model["enum_fields"].keys():
            py_values = set(py_model["enum_fields"].get(field, []))
            rs_values = set(rs_model["enum_fields"].get(field, []))
            if py_values - rs_values:
                report["enum_python_only"][f"{label}.{field}"] = sorted(py_values - rs_values)
            if rs_values - py_values:
                report["enum_rust_only"][f"{label}.{field}"] = sorted(rs_values - py_values)
    return report


def canonical(report: dict) -> dict:
    return {
        key: report[key]
        for key in (
            "field_python_only",
            "field_rust_only",
            "required_python_only",
            "required_rust_only",
            "enum_python_only",
            "enum_rust_only",
            "python_only_models",
            "rust_only_models",
        )
    }


def python_methods(root: Path) -> dict[str, list[str]]:
    methods = {}
    api = root / "api"
    for path in sorted(api.glob("*.py")):
        if path.name == "__init__.py":
            continue
        parts = re.split(r"\n    async def ", path.read_text())
        for part in parts[1:]:
            name = part.split("(", 1)[0]
            params = [
                item
                for item in re.findall(r":param ([A-Za-z_][A-Za-z0-9_]*)", part)
                if not item.startswith("_")
            ]
            methods[name] = sorted(set(params))
    return methods


def rust_methods(apis: Path) -> dict[str, list[str]]:
    structs = {}
    for path in apis.glob("*.rs"):
        text = path.read_text()
        for match in re.finditer(r"pub struct (\w+)\s*\{(.*?)\n\}", text, re.S):
            fields = []
            for field in re.findall(r"\n    pub (r#)?([A-Za-z0-9_]+):", match.group(2)):
                fields.append("type" if field[0] else field[1])
            structs[match.group(1)] = fields
    methods = {}
    for path in apis.glob("*.rs"):
        if path.name in {"mod.rs", "configuration.rs"}:
            continue
        for match in re.finditer(r"pub async fn ([A-Za-z0-9_]+)\((.*?)\)\s*->", path.read_text(), re.S):
            fields = []
            for arg_name, ty in re.findall(r"([A-Za-z0-9_]+)\s*:\s*([A-Za-z0-9_:]+)", match.group(2)):
                if arg_name == "configuration":
                    continue
                short = ty.split("::")[-1]
                if short.endswith("Params") and short in structs:
                    fields.extend(structs[short])
                else:
                    fields.append(arg_name)
            methods[match.group(1)] = sorted(set(fields))
    return methods


def api_report(python_root: Path, rust_apis: Path) -> dict:
    py = python_methods(python_root)
    rs = rust_methods(rust_apis)
    report = {
        "python_only_methods": sorted(set(py) - set(rs)),
        "rust_only_methods": sorted(set(rs) - set(py)),
        "param_python_only": {},
        "param_rust_only": {},
    }
    for name in sorted(set(py) & set(rs)):
        only_py = sorted(set(py[name]) - set(rs[name]))
        only_rs = sorted(set(rs[name]) - set(py[name]))
        if only_py:
            report["param_python_only"][name] = only_py
        if only_rs:
            report["param_rust_only"][name] = only_rs
    return report


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python", type=Path, required=True, help="unpacked kalshi_python_async package")
    parser.add_argument("--rust", type=Path, required=True)
    parser.add_argument("--rust-api", type=Path, required=True)
    parser.add_argument("--expected", type=Path, required=True)
    parser.add_argument("--api-expected", type=Path, required=True)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)
    python_models = parse_python(args.python)
    defaults = {name: model["defaults"] for name, model in python_models.items() if model.get("defaults")}
    expected_defaults = json.loads(Path(__file__).with_name("python_model_defaults.json").read_text())
    if defaults != expected_defaults:
        raise SystemExit("python_model_defaults.json differs from the pinned SDK model defaults")
    report = canonical(diff_report(python_models, parse_rust(args.rust)))
    expected = json.loads(args.expected.read_text())
    expected_cmp = canonical(expected)
    if report != expected_cmp:
        print("conformance mismatch")
        print("observed:")
        print(json.dumps(report, indent=2, sort_keys=True))
        print("expected:")
        print(json.dumps(expected_cmp, indent=2, sort_keys=True))
        return 1
    observed_api = api_report(args.python, args.rust_api)
    expected_api = json.loads(args.api_expected.read_text())
    if observed_api != expected_api:
        print("api conformance mismatch")
        print("observed:")
        print(json.dumps(observed_api, indent=2, sort_keys=True))
        print("expected:")
        print(json.dumps(expected_api, indent=2, sort_keys=True))
        return 1
    if args.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        print(
            "conformance ok: "
            f"{len(report['python_only_models'])} python-only models, "
            f"{len(report['rust_only_models'])} rust-only models, "
            f"{sum(len(v) for v in report['field_python_only'].values()) + sum(len(v) for v in report['field_rust_only'].values())} field diffs"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
