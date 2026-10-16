#!/usr/bin/env python3
"""Census the pinned Python SDK, Rust API, and official specifications offline."""

from __future__ import annotations

import argparse
import ast
import csv
import importlib.util
import json
import re
from pathlib import Path

import yaml


def module(path: Path):
    spec = importlib.util.spec_from_file_location(path.stem, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


def deref(spec: dict, value: dict) -> dict:
    while "$ref" in value:
        node = spec
        for part in value["$ref"].removeprefix("#/").split("/"):
            node = node[part.replace("~1", "/").replace("~0", "~")]
        value = node
    return value


def operations(python: Path) -> list[dict]:
    rows = []
    for path in sorted((python / "api").glob("*_api.py")):
        cls = next(n for n in ast.parse(path.read_text()).body if isinstance(n, ast.ClassDef))
        funcs = {n.name: n for n in cls.body if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))}
        for name, func in funcs.items():
            if name.startswith("_") or not isinstance(func, ast.AsyncFunctionDef):
                continue
            base = name.removesuffix("_with_http_info").removesuffix("_without_preload_content")
            serial = funcs[f"_{base}_serialize"]
            call = next(n for n in ast.walk(serial) if isinstance(n, ast.Call) and isinstance(n.func, ast.Attribute) and n.func.attr == "param_serialize")
            wire = {kw.arg: ast.literal_eval(kw.value) for kw in call.keywords if kw.arg in ("method", "resource_path")}
            auth = next(n for n in serial.body if isinstance(n, ast.AnnAssign) and getattr(n.target, "id", "") == "_auth_settings")
            rows.append({"python": f"{cls.name}.{name}", "class": cls.name, "module": path.stem, "method": name, "operation": base, "http": wire["method"], "path": wire["resource_path"], "response_python": ast.unparse(func.returns), "auth": ast.literal_eval(auth.value), "serialize_args": [n.arg for n in serial.args.args if n.arg != "self"], "source": f"api/{path.name}:{func.lineno}"})
    return rows


def models(python: Path) -> dict:
    result = {}
    for path in sorted((python / "models").glob("*.py")):
        if path.name == "__init__.py":
            continue
        cls = next(n for n in ast.parse(path.read_text()).body if isinstance(n, ast.ClassDef))
        fields = {}
        values = []
        for node in cls.body:
            if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name) and not node.target.id.startswith("_") and node.target.id != "model_config":
                annotation = ast.unparse(node.annotation)
                if annotation.startswith("ClassVar"):
                    continue
                defaults = {kw.arg: ast.unparse(kw.value) for kw in node.value.keywords} if isinstance(node.value, ast.Call) else {}
                required = node.value is None or (isinstance(node.value, ast.Call) and "default" not in defaults and "default_factory" not in defaults)
                fields[node.target.id] = {"type": annotation, "required": required, "nullable": annotation.startswith("Optional["), "enum": []}
            if isinstance(node, ast.Assign) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
                values.append(node.value.value)
            if isinstance(node, ast.FunctionDef):
                for decorator in node.decorator_list:
                    if isinstance(decorator, ast.Call) and getattr(decorator.func, "id", "") == "field_validator":
                        for argument in decorator.args:
                            field = ast.literal_eval(argument)
                            if field not in fields:
                                continue
                            for test in ast.walk(node):
                                if isinstance(test, ast.Compare) and any(isinstance(op, ast.NotIn) for op in test.ops):
                                    try:
                                        validator_values = test.comparators[0]
                                        if isinstance(validator_values, ast.Call) and getattr(validator_values.func, "id", "") == "set":
                                            validator_values = validator_values.args[0]
                                        fields[field]["enum"] = sorted(ast.literal_eval(validator_values))
                                    except (ValueError, TypeError):
                                        pass
        result[cls.name] = {"module": path.stem, "fields": fields, "enum": values, "source": f"models/{path.name}:{cls.lineno}"}
    return result


def write_table(path: Path, fields: list[str], rows: list[dict]) -> None:
    with path.open("w") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, delimiter="\t", extrasaction="ignore")
        writer.writeheader()
        for row in rows:
            writer.writerow({key: json.dumps(value, sort_keys=True, separators=(",", ":")) if isinstance(value, (dict, list)) else value for key, value in row.items()})
        counts = {status: sum(row.get("status") == status for row in rows) for status in ("same", "added", "changed", "missing")}
        stream.write(f"# summary rows={len(rows)} " + " ".join(f"{key}={value}" for key, value in counts.items()) + "\n")


def schema_fields(spec: dict, schema: dict, prefix: str = ""):
    schema = deref(spec, schema)
    for name, value in schema.get("properties", {}).items():
        path = f"{prefix}.{name}" if prefix else name
        resolved = deref(spec, value)
        yield path, {"required": name in schema.get("required", []), "schema": value, "resolved_schema": resolved}
        if resolved.get("type") == "object":
            yield from schema_fields(spec, resolved, path)
        elif resolved.get("type") == "array":
            yield from schema_fields(spec, resolved.get("items", {}), path + "[]")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python", type=Path, required=True)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--openapi", type=Path, required=True)
    parser.add_argument("--asyncapi", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    spec = yaml.safe_load(args.openapi.read_text())
    ws = yaml.safe_load(args.asyncapi.read_text())
    conform = module(args.root / "tools/kalshi-sdk/conformance.py")
    rust_api = conform.rust_methods(args.root / "crates/kalshi-api-openapi/src/apis")
    rust_models = conform.parse_rust(args.root / "crates/kalshi-api-openapi/src/models")
    rust_fold = {name.casefold(): (name, value) for name, value in rust_models.items()}
    fixed_point_rows = json.loads((args.root / "tools/kalshi-sdk/fixed_point_fields.json").read_text())["fields"]
    fixed_point_fields = {}
    for field in fixed_point_rows:
        fixed_point_fields.setdefault(field["struct"].casefold(), []).append(field["field"])
    ops = operations(args.python)
    py_models = models(args.python)
    official_ops = {(m.upper(), p): value for p, item in spec["paths"].items() for m, value in item.items() if m in {"get", "post", "put", "delete", "patch", "head", "options"}}
    assert {(row["http"], row["path"]) for row in ops} == set(official_ops), "Python and OpenAPI operation sets differ"
    rows = []
    for operation in ops:
        node = official_ops[operation["http"], operation["path"]]
        params = [deref(spec, item) for item in node.get("parameters", [])]
        extensions = [p for p in params if ("var_from" if p["name"] == "from" else p["name"]) not in operation["serialize_args"]]
        params = [p for p in params if p not in extensions]
        row = dict(operation)
        row.update(params=params, body=node.get("requestBody", {}), response=node.get("responses", {}), rust=f"kalshi_api_openapi::apis::{row['module']}::{row['method']}", spec_only=[{"parameter": p["name"], "note": "spec-only, not in SDK 3.34.0; Rust extension"} for p in extensions], status="same" if row["method"] in rust_api else "missing", test_id=f"parity_requests::every_python_public_method_builds_the_same_request/{row['python']}")
        rows.append(row)
    write_table(args.output / "parity_rest.tsv", ["python", "http", "path", "params", "body", "response_python", "response", "auth", "rust", "spec_only", "status", "test_id", "source"], rows)
    rows = []
    for name, value in py_models.items():
        rust_name, rust = rust_fold.get(name.casefold(), (name, None))
        matches = rust is not None and (set(rust.get("values", [])) == set(value["enum"]) if value["enum"] else set(rust.get("fields", [])) == set(value["fields"]) and set(rust.get("required", [])) == {field for field, shape in value["fields"].items() if shape["required"]})
        rows.append({"model": name, **value, "rust": f"kalshi_api_openapi::models::{rust_name}", "rust_fields": rust.get("types", {}) if rust else {}, "decimal_fields": sorted(fixed_point_fields.get(name.casefold(), [])), "status": "same" if matches else "changed" if rust else "missing", "test_id": f"parity_models::every_python_model_roundtrips_without_field_loss/{name}"})
    assert sum(len(row["decimal_fields"]) for row in rows) == len(fixed_point_rows), "a declared fixed-point field has no model parity row"
    write_table(args.output / "parity_models.tsv", ["model", "fields", "rust_fields", "enum", "decimal_fields", "rust", "status", "test_id", "source"], rows)
    types = "\n".join(path.read_text() for path in (args.root / "crates/kalshi-api/src/ws").glob("*.rs"))
    ws_mapping = module(args.root / "tools/kalshi-sdk/ws_fields.py")
    owners = dict(ws_mapping.ROOTS)
    owners.update({"subscribedIndicesResponsePayload": "Incoming::Ok", "subscribedUnderlyingsResponsePayload": "Incoming::Ok", "listSubscriptionsResponsePayload": "Incoming::Ok"})
    field_rows = json.loads((args.root / "tools/kalshi-sdk/ws_fields.json").read_text())["fields"]
    by_path = {row["schema"] + "." + row["field"]: row["owner"] + "." + row["field"] for row in field_rows}
    rows = []
    for name, channel in ws["channels"].items():
        rows.append({"kind": "channel", "name": name, "contract": channel, "rust": "WsSession" if name in {"root", "control_frames"} else "Channel", "status": "same" if name in {"root", "control_frames"} or f'"{name}"' in types else "missing", "test_id": "ws_behavior" if name in {"root", "control_frames"} else f"parity_ws/{name}"})
    for name, message in ws["components"]["messages"].items():
        payload = deref(ws, message["payload"])
        properties = payload.get("properties", {})
        discriminator = properties.get("type", properties.get("cmd", {}))
        discriminants = discriminator.get("enum", [discriminator.get("const")])
        schema_name = message["payload"].get("$ref", "").rsplit("/", 1)[-1]
        control = name in {"incomingPing", "incomingPong", "outgoingPing", "outgoingPong"}
        owner = "WsSession" if control else owners.get(schema_name)
        mapped = control or owner is not None
        rows.append({"kind": "message", "name": name, "contract": message, "rust": owner or "", "status": "same" if mapped else "missing", "test_id": "ws_behavior" if control else f"parity_ws::official_ws_examples_and_every_payload_roundtrip_without_field_loss/{name}"})
        for field, shape in schema_fields(ws, payload):
            wire_path = schema_name + "." + field.replace("[]", "")
            field_owner = by_path.get(wire_path)
            if field in {"type", "cmd"}:
                field_owner = owner
            if schema_name in {"subscribedIndicesResponsePayload", "subscribedUnderlyingsResponsePayload", "listSubscriptionsResponsePayload"} and "." not in field:
                field_owner = "Incoming::Ok." + field
            rows.append({"kind": "field", "name": f"{name}.{field}", "contract": shape, "rust": field_owner or "", "status": "same" if mapped and field_owner else "missing", "test_id": f"parity_ws/{name}; ws_fields::typed_messages_match_the_asyncapi_field_list"})
    write_table(args.output / "parity_ws.tsv", ["kind", "name", "contract", "rust", "status", "test_id"], rows)
    signer = (args.root / "crates/kalshi-api/src/sign.rs").read_text()
    auth = [
        {"contract": "message", "python": "timestamp_ms + method.upper() + URL path without query", "rust": "SignMiddleware::handle/headers; WsSession::connect", "status": "same", "test_id": "parity_signing::python_and_rust_sign_the_identical_message_for_both_key_types"},
        {"contract": "headers", "python": "KALSHI-ACCESS-KEY,KALSHI-ACCESS-SIGNATURE,KALSHI-ACCESS-TIMESTAMP", "rust": "SignMiddleware", "status": "same", "test_id": "parity_requests::every_python_public_method_builds_the_same_request"},
        {"contract": "RSA-PSS", "python": "SHA256; MGF1 SHA256; salt_length DIGEST_LENGTH (32 bytes); PKCS8/PKCS1 PEM", "rust": "SignMiddleware", "status": "same", "test_id": "parity_signing::python_and_rust_sign_the_identical_message_for_both_key_types/rsa"},
        {"contract": "Ed25519", "python": "PEM type detection; direct message signing", "rust": "SignMiddleware", "status": "same" if "Ed25519" in signer else "missing", "test_id": "parity_signing::python_and_rust_sign_the_identical_message_for_both_key_types/ed25519"},
    ]
    write_table(args.output / "parity_auth.tsv", ["contract", "python", "rust", "status", "test_id"], auth)
    config = [
        {"contract": "hosts", "python": spec["servers"], "rust": "ClientConfig.base_url; Environment::{Prod,ProdShared,Demo,DemoShared}; WsHost", "status": "same", "test_id": "parity_config::explicit_hosts_match_the_official_environment_urls; ws_behavior::production_socket_is_the_default_host"},
        {"contract": "timeouts", "python": "per-call _request_timeout; 300 seconds when omitted; signature accepts scalar or connect/read pair", "rust": "ClientConfig and Configuration::new require connect_timeout/request_timeout Durations; each generated request carries request_timeout even with a replaced client", "status": "changed", "test_id": "rest_deadlines::generated_deadline_bounds_headers_and_body_with_a_replaced_client; parity_requests::every_python_public_method_builds_the_same_request"},
        {"contract": "retries", "python": "Configuration.retries; disabled by default; GET/HEAD/PUT/DELETE/OPTIONS/TRACE; server errors only; exponential 0.1*2^attempt capped at 120s", "rust": "RetryMiddleware::new(attempts); optional generated Configuration.client middleware", "status": "same", "test_id": "parity_config::opt_in_retries_use_python_method_and_status_rules"},
        {"contract": "exceptions", "python": "ApiTypeError/ApiValueError/ApiAttributeError/ApiKeyError map to typed construction and serde failures; BadRequestException=400; UnauthorizedException=401; ForbiddenException=403; NotFoundException=404; ServiceException=5xx; other ApiException preserves status", "rust": "VenueError::{RequestBuild,Http,Transport,SchemaDrift}; kalshi_api_openapi::apis::Error::{Reqwest,ReqwestMiddleware,Serde,ResponseError}", "status": "changed", "test_id": "parity_config::python_http_exception_classes_keep_their_status_in_rust; http_behavior::capture_classifies_responses"},
    ]
    write_table(args.output / "parity_config.tsv", ["contract", "python", "rust", "status", "test_id"], config)
    (args.output / "python_surface.json").write_text(json.dumps({"operations": ops, "models": py_models}, indent=2) + "\n")
    if args.check:
        for path in args.output.glob("parity_*.tsv"):
            assert "missing=0" in path.read_text().splitlines()[-1], f"unmapped contracts in {path.name}"
        fixture_root = args.root / "crates/kalshi-api/tests/fixtures/parity"
        requests = json.loads((fixture_root / "requests.json").read_text())
        assert {row["id"].rsplit("::", 1)[0] for row in requests} == {row["python"] for row in ops}
        assert {row["model"] for row in json.loads((fixture_root / "models.json").read_text())} == set(py_models)
        assert {row["message"] for row in json.loads((fixture_root / "ws.json").read_text())} == set(ws["components"]["messages"]) - {"incomingPing", "incomingPong", "outgoingPing", "outgoingPong"}
    print(f"{len(ops)} public methods, {len(official_ops)} operations, {len(py_models)} models, {len(ws['channels'])} WS channels, {len(ws['components']['messages'])} WS messages")


if __name__ == "__main__":
    main()
