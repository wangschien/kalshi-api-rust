#!/usr/bin/env python3
"""Run the official Python SDK serialization offline and emit cross-language fixtures."""

from __future__ import annotations

import argparse
import asyncio
import datetime
import copy
import importlib
import importlib.metadata
import inspect
import json
import re
import socket
from pathlib import Path
from unittest.mock import patch
from urllib.parse import urlsplit

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ed25519, rsa

from kalshi_python_async.api_client import ApiClient
from kalshi_python_async.auth import KalshiAuth
from kalshi_python_async.configuration import Configuration
from pydantic import ValidationError


def resolve(spec, node):
    while "$ref" in node:
        value = spec
        for key in node["$ref"].removeprefix("#/").split("/"):
            value = value[key.replace("~1", "/").replace("~0", "~")]
        node = value
    return node


def sample(spec, node, full, name="value", depth=0):
    if depth > 16:
        raise ValueError(f"recursive schema at {name}")
    node = resolve(spec, node)
    if "const" in node:
        return node["const"]
    if "enum" in node:
        return next((v for v in node["enum"] if v is not None), None)
    for key in ("oneOf", "anyOf"):
        if key in node:
            return sample(spec, next((v for v in node[key] if v.get("type") != "null"), node[key][0]), full, name, depth + 1)
    if "allOf" in node:
        combined = {}
        for item in node["allOf"]:
            combined.update(resolve(spec, item))
        combined.update({key: value for key, value in node.items() if key != "allOf"})
        return sample(spec, combined, full, name, depth + 1)
    kind = node.get("type", "object" if "properties" in node else "string")
    if isinstance(kind, list):
        kind = next(k for k in kind if k != "null")
    if kind == "object":
        return {key: sample(spec, value, full, key, depth + 1) for key, value in node.get("properties", {}).items() if full or key in node.get("required", []) or "subaccount" in key}
    if kind == "array":
        length = max(1, node.get("minItems", 0))
        return [sample(spec, node.get("items", {}), full, name, depth + 1) for _ in range(length)]
    if kind == "integer":
        return min(max(1, node.get("minimum", 1)), node.get("maximum", 2**31 - 1))
    if kind == "number":
        return min(max(1, node.get("minimum", 1)), node.get("maximum", 1))
    if kind == "boolean":
        return True
    if kind == "null":
        return None
    if "example" in node and isinstance(node["example"], str):
        return node["example"]
    if node.get("format") == "uuid":
        return "00000000-0000-4000-8000-000000000001"
    if node.get("format") == "date-time":
        return "2026-01-02T03:04:05+00:00"
    if node.get("format") == "date":
        return "2026-01-02"
    if "pattern" in node:
        for value in ("0.5600", "1.00", "1", "SYNTH", "synthetic"):
            if re.fullmatch(node["pattern"], value):
                return value
        raise ValueError(f"no synthetic value matches {name}: {node['pattern']}")
    if "dollar" in name or name.endswith("_fp") or name in {"price", "count", "fee_cost", "reduced_by"}:
        return "1.00" if "count" in name or name.endswith("_fp") else "0.5600"
    value = "synthetic"
    if name.endswith("_id"):
        value = "00000000-0000-4000-8000-000000000001"
    return (value + "x" * node.get("minLength", 0))[:node.get("maxLength", 100)]


def model_class(name, manifest):
    return getattr(importlib.import_module("kalshi_python_async.models." + manifest[name]["module"]), name)


def write_cases(path, cases):
    path.write_text("[\n" + ",\n".join(json.dumps(case, sort_keys=True, separators=(",", ":")) for case in cases) + "\n]\n")


def synthetic_keys(directory):
    keys = {}
    for name, create in (("rsa", lambda: rsa.generate_private_key(public_exponent=65537, key_size=2048)), ("ed25519", ed25519.Ed25519PrivateKey.generate)):
        private = directory / f"{name}_test_private.pem"
        if not private.exists():
            private.write_bytes(create().private_bytes(serialization.Encoding.PEM, serialization.PrivateFormat.PKCS8, serialization.NoEncryption()))
        key = serialization.load_pem_private_key(private.read_bytes(), password=None)
        (directory / f"{name}_test_public.pem").write_bytes(key.public_key().public_bytes(serialization.Encoding.PEM, serialization.PublicFormat.SubjectPublicKeyInfo))
        keys[name] = private.read_text()
    return keys


class Captured(Exception):
    pass


def no_network(*args, **kwargs):
    raise AssertionError("fixture generation must not create a network session or connection")


async def requests(spec, surface, key):
    client = ApiClient(Configuration(host="https://example.invalid/trade-api/v2"))
    assert client.rest_client.pool_manager is None
    client.user_agent = "parity-fixture"
    client.kalshi_auth = KalshiAuth("synthetic-key", key)
    fixtures = []
    current = {}

    async def capture(method, url, headers, body, post_params, _request_timeout):
        assert not post_params
        parsed = urlsplit(url)
        current["expected"] = {"method": method, "url": url, "path": parsed.path, "query": parsed.query, "headers": {k.lower(): v for k, v in headers.items() if k.lower() != "kalshi-access-signature"}, "body": body}
        raise Captured

    client.rest_client.request = capture
    for op in surface["operations"]:
        node = spec["paths"][op["path"]][op["http"].lower()]
        cls = getattr(importlib.import_module("kalshi_python_async.api." + op["module"]), op["class"])
        api = cls(client)
        method = getattr(api, op["method"])
        params = inspect.signature(method).parameters
        body_schema = node.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema")
        for full in (False, True):
            arguments = {}
            for param in node.get("parameters", []):
                param = resolve(spec, param)
                name = "var_from" if param["name"] == "from" else param["name"]
                if name not in op["serialize_args"]:
                    continue
                if full or param.get("required") or "subaccount" in param["name"]:
                    value = sample(spec, param["schema"], full, param["name"])
                    ref = param["schema"].get("$ref", "").rsplit("/", 1)[-1]
                    if ref in surface["models"]:
                        cls = model_class(ref, surface["models"])
                        value = cls.model_validate(value) if hasattr(cls, "model_validate") else cls(value)
                    if resolve(spec, param["schema"]).get("format") == "date-time":
                        value = datetime.datetime.fromisoformat(value)
                    arguments[name] = value
            body = None
            body_input = None
            body_name = None
            if body_schema:
                name = body_schema["$ref"].rsplit("/", 1)[-1]
                param_names = {("var_from" if p["name"] == "from" else p["name"]) for item in node.get("parameters", []) for p in [resolve(spec, item)]}
                body_name = next(n for n in op["serialize_args"] if n not in param_names and not n.startswith("_"))
                if full or node["requestBody"].get("required"):
                    value = sample(spec, body_schema, full)
                    resolved = resolve(spec, body_schema)
                    if not value and node["requestBody"].get("required") and resolved.get("properties"):
                        field, prop = next(iter(resolved["properties"].items()))
                        value[field] = sample(spec, prop, full, field)
                    body_input = value
                    body = model_class(name, surface["models"]).model_validate(value) if name in surface["models"] else value
            kwargs = dict(arguments)
            if body is not None:
                if body_name in params:
                    kwargs[body_name] = body
                else:
                    kwargs.update(body.model_dump(exclude_unset=True))
            rust_arguments = {name: value.strftime(client.configuration.datetime_format) if isinstance(value, datetime.datetime) else value for name, value in arguments.items()}
            current = {"id": op["python"] + ("::full" if full else "::required"), "operation": op["method"], "module": op["module"], "arguments": client.sanitize_for_serialization(rust_arguments), "body_parameter": body_name, "body": body_input}
            try:
                await method(**kwargs)
            except Captured:
                fixtures.append(current)
            except Exception as error:
                raise RuntimeError(f"{current['id']}: {error}") from error
            else:
                raise AssertionError(f"{current['id']} did not reach request serialization")
    assert client.rest_client.pool_manager is None
    await client.close()
    return fixtures


def rust_requests(root, fixtures):
    by_operation = {f["operation"]: f for f in fixtures}
    arms = []
    for operation, fixture in sorted(by_operation.items()):
        module = fixture["module"]
        source = (root / "crates/kalshi-api-openapi/src/apis" / f"{module}.rs").read_text()
        function = re.search(r"pub async fn " + operation + r"\((.*?)\)\s*->", source, re.S)
        assert function, operation
        param = re.search(r"params:\s*(\w+)", function[1])
        call = f"kalshi_api_openapi::apis::{module}::{operation}(&configuration"
        if param:
            name = param[1]
            structure = re.search(r"pub struct " + name + r" \{(.*?)\n\}", source, re.S)[1]
            fields = re.findall(r"pub (r#)?(\w+):\s*([^\n]+),", structure)
            parts = []
            for raw, field, ty in fields:
                key = "var_from" if field == "from" else field
                if field == fixture["body_parameter"]:
                    value = 'fixture["body"].clone()'
                else:
                    value = f'fixture["arguments"]["{key}"].clone()'
                parts.append(f"{raw}{field}: serde_json::from_value({value}).unwrap()")
            call += f", kalshi_api_openapi::apis::{module}::{name} {{ " + ", ".join(parts) + " }"
        call += ").await"
        arms.append(f'            "{operation}" => {{ let _ = {call}; }},')
    return "\n".join(arms)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--openapi-json", type=Path, required=True)
    parser.add_argument("--asyncapi-json", type=Path, required=True)
    parser.add_argument("--surface", type=Path, required=True)
    args = parser.parse_args()
    assert importlib.metadata.version("kalshi_python_async") == "3.34.0"
    spec = json.loads(args.openapi_json.read_text())
    ws = json.loads(args.asyncapi_json.read_text())
    surface = json.loads(args.surface.read_text())
    target = args.root / "crates/kalshi-api/tests/fixtures/parity"
    target.mkdir(parents=True, exist_ok=True)
    keys = synthetic_keys(target)
    with patch("socket.create_connection", no_network), patch("socket.socket.connect", no_network), patch("aiohttp.ClientSession", no_network), patch("kalshi_python_async.auth.time.time", lambda: 1700000000):
        fixtures = asyncio.run(requests(spec, surface, keys["rsa"]))
        write_cases(target / "requests.json", fixtures)
        arms = rust_requests(args.root, fixtures)
        (args.root / "crates/kalshi-api/tests/fixtures/parity/request_dispatch.rs").write_text("match operation {\n" + arms + '\n            other => panic!("unmapped operation {other}"),\n}\n')
        model_fixtures = []
        model_arms = []
        defaults = {}
        required_fields = {}
        serializer = ApiClient(Configuration(host="https://example.invalid"))
        rust_names = {}
        for path in (args.root / "crates/kalshi-api-openapi/src/models").glob("*.rs"):
            found = re.search(r"pub (?:struct|enum) (\w+)", path.read_text())
            if found:
                rust_names[found[1].casefold()] = found[1]
        for name, data in surface["models"].items():
            cls = model_class(name, surface["models"])
            default_fields = {field: info.default for field, info in getattr(cls, "model_fields", {}).items() if not info.is_required() and info.default is not None}
            if default_fields:
                defaults[name] = default_fields
            node = spec["components"]["schemas"].get(name)
            if node is None:
                for parent, schema in spec["components"]["schemas"].items():
                    for field, prop in schema.get("properties", {}).items():
                        inline_name = parent + "".join(word.capitalize() for word in field.split("_")) + "Inner"
                        if inline_name == name and prop.get("type") == "array":
                            node = prop["items"]
                assert node is not None, f"unmapped Python model {name}"
            complete = sample(spec, node, True)
            cases = [("required", sample(spec, node, False)), ("full", complete)]
            if "example" in node:
                cases.append(("openapi-example", node["example"]))
            for value in data["enum"]:
                cases.append((f"enum-{value}", value))
            for field, shape in data["fields"].items():
                for value in shape["enum"]:
                    case = copy.deepcopy(complete)
                    case[field] = value
                    cases.append((f"enum-{field}-{value}", case))
                if "StrictFloat" in shape["type"] and "StrictInt" in shape["type"]:
                    for value in (9007199254740993, 0.125):
                        case = copy.deepcopy(complete)
                        case[field] = value
                        cases.append((f"number-{field}-{value}", case))
                if shape["nullable"]:
                    case = copy.deepcopy(complete)
                    case[field] = None
                    cases.append((f"null-{field}", case))
            for case_name, value in cases:
                if hasattr(cls, "model_validate"):
                    model = cls.model_validate(value)
                    expected = serializer.sanitize_for_serialization(model)
                else:
                    expected = cls(value).value
                model_fixtures.append({"model": name, "case": case_name, "input": value, "value": expected})
            required_fields[name] = []
            for field, info in getattr(cls, "model_fields", {}).items():
                if not info.is_required():
                    continue
                missing = copy.deepcopy(complete)
                del missing[field]
                try:
                    cls.model_validate(missing)
                except ValidationError:
                    required_fields[name].append(field)
                else:
                    raise AssertionError(f"{name}.{field} is not actually required")
            rust = rust_names[name.casefold()]
            model_arms.append(f'            "{name}" => roundtrip::<kalshi_api_openapi::models::{rust}>(value),')
        write_cases(target / "models.json", model_fixtures)
        (target / "required_models.json").write_text(json.dumps(required_fields, indent=2, sort_keys=True) + "\n")
        (args.root / "tools/kalshi-sdk/python_model_defaults.json").write_text(json.dumps(defaults, indent=2, sort_keys=True) + "\n")
        (target / "model_dispatch.rs").write_text('match model {\n' + '\n'.join(model_arms) + '\n            other => panic!("unmapped model {other}"),\n}\n')
        ws_fixtures = []
        for name, message in ws["components"]["messages"].items():
            node = resolve(ws, message["payload"])
            if node.get("type") != "object":
                continue
            for index, example in enumerate(message.get("examples", [])):
                ws_fixtures.append({"message": name, "case": f"example-{index}", "value": example["payload"]})
            for full in (False, True):
                ws_fixtures.append({"message": name, "case": "full" if full else "required", "value": sample(ws, node, full)})
        write_cases(target / "ws.json", ws_fixtures)
        signing = []
        for algorithm, key in keys.items():
            auth = KalshiAuth("synthetic-key", key)
            for method, path in (("get", "/trade-api/v2/markets?limit=1"), ("POST", "/trade-api/v2/portfolio/events/orders"), ("GET", "/trade-api/ws/v2"), ("GET", "/trade-api/v2/path%2Fencoded?x=1"), ("get", "/trade-api/v2/path#literal")):
                headers = auth.create_auth_headers(method, path)
                message = headers["KALSHI-ACCESS-TIMESTAMP"] + method.upper() + path.split("?")[0]
                signing.append({"algorithm": algorithm, "method": method, "path": path, "message": message, "headers": headers})
        (target / "signing.json").write_text(json.dumps(signing, indent=2, sort_keys=True) + "\n")
    print(f"offline fixtures: {len(fixtures)} requests, {len(model_fixtures)} models, {len(ws_fixtures)} WS messages, {len(signing)} signing vectors")


if __name__ == "__main__":
    main()
