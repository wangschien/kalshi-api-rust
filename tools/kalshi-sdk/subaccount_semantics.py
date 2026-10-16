#!/usr/bin/env python3
"""Export subaccount omission semantics from the pinned OpenAPI document.

Each row is one subaccount parameter or body field. The omission class is
read from that field's description. Drift fails when the committed JSON
differs from this derivation.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
SPEC = ROOT / "spec" / "openapi.yaml"
OUT = ROOT / "tools" / "kalshi-sdk" / "subaccount_semantics.json"

# Operations the hand-written client wraps. Every other subaccount field stays
# in the table so a new operation cannot appear without a drift failure.
WRAPPED = {
    "AmendOrderV2",
    "BatchCancelOrdersV2",
    "BatchCreateOrdersV2",
    "CancelOrderV2",
    "CreateOrderV2",
    "DecreaseOrderV2",
    "GetBalance",
    "GetFills",
    "GetOrderQueuePositions",
    "GetOrders",
    "GetPositions",
}

TYPED = {
    "all": "SubaccountFilter",
    "primary": "Subaccount",
    "choice": "SubaccountChoice",
}

HTTP_METHODS = {"get", "post", "put", "delete", "patch"}


def normalize(text: str) -> str:
    return " ".join(text.split())


def resolve(spec: dict, ref: str) -> dict:
    node: object = spec
    for part in ref.lstrip("#/").split("/"):
        if not isinstance(node, dict) or part not in node:
            raise SystemExit(f"unresolved ref {ref}")
        node = node[part]
    if not isinstance(node, dict):
        raise SystemExit(f"ref {ref} is not an object")
    return node


def deref(spec: dict, node: dict) -> dict:
    ref = node.get("$ref")
    if isinstance(ref, str) and set(node) <= {"$ref"}:
        return deref(spec, resolve(spec, ref))
    return node


def classify(description: str) -> str:
    if "If omitted, defaults to all subaccounts." in description:
        return "all"
    if "Defaults to 0." in description:
        return "primary"
    if "must omit this field or pass their locked subaccount" in description:
        return "choice"
    # The sentence is still committed. A wrapped call cannot use this class.
    return "recorded"


def snake(operation_id: str) -> str:
    out: list[str] = []
    for index, char in enumerate(operation_id):
        previous = operation_id[index - 1] if index else ""
        nxt = operation_id[index + 1] if index + 1 < len(operation_id) else ""
        if char.isupper() and index and (not previous.isupper() or nxt.islower()):
            out.append("_")
        out.append(char.lower())
    return "".join(out)


def walk_schema(spec: dict, schema: dict, pointer: str, found: list[tuple[str, str, str]]) -> None:
    schema = deref(spec, schema)
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        return
    for name, prop in properties.items():
        if not isinstance(prop, dict):
            continue
        child = f"{pointer}.{name}"
        if name in {"subaccount", "exchange_index"}:
            description = prop.get("description")
            if not isinstance(description, str) or not description.strip():
                raise SystemExit(f"{child} has no description")
            found.append((name, child, normalize(description)))
            continue
        if prop.get("type") == "array" and isinstance(prop.get("items"), dict):
            walk_schema(spec, prop["items"], f"{child}[]", found)
        elif "$ref" in prop:
            walk_schema(spec, prop, child, found)
        elif "properties" in prop:
            walk_schema(spec, prop, child, found)


def parameter_description(spec: dict, param: dict) -> tuple[str, str] | None:
    if "$ref" in param:
        param = resolve(spec, param["$ref"])
    name = param.get("name")
    if name not in {"subaccount", "exchange_index"}:
        return None
    description = param.get("description")
    if not isinstance(description, str) or not description.strip():
        raise SystemExit(f"parameter {name} has no description")
    return name, normalize(description)


def derive(spec: dict) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    for path, item in spec["paths"].items():
        if not isinstance(item, dict):
            continue
        for method, operation in item.items():
            if method not in HTTP_METHODS or not isinstance(operation, dict):
                continue
            operation_id = operation.get("operationId")
            if not isinstance(operation_id, str):
                raise SystemExit(f"{method.upper()} {path} has no operationId")
            subaccounts: list[tuple[str, str]] = []
            exchange_indexes: list[str] = []
            for param in operation.get("parameters") or []:
                if not isinstance(param, dict):
                    continue
                parsed = parameter_description(spec, param)
                if parsed is None:
                    continue
                name, description = parsed
                if name == "subaccount":
                    subaccounts.append((f"query.{name}", description))
                else:
                    exchange_indexes.append(description)
            body = operation.get("requestBody")
            if isinstance(body, dict):
                content = body.get("content")
                if isinstance(content, dict):
                    json_body = content.get("application/json")
                    if isinstance(json_body, dict) and isinstance(json_body.get("schema"), dict):
                        found: list[tuple[str, str, str]] = []
                        walk_schema(spec, json_body["schema"], "body", found)
                        for name, pointer, description in found:
                            if name == "subaccount":
                                subaccounts.append((pointer, description))
                            else:
                                exchange_indexes.append(description)
            if not subaccounts:
                continue
            unique_indexes = list(dict.fromkeys(exchange_indexes))
            wrapped = operation_id in WRAPPED
            for pointer, description in subaccounts:
                omission = classify(description)
                if wrapped and omission == "recorded":
                    raise SystemExit(
                        f"{operation_id} {pointer} has no typed omission class: {description}"
                    )
                rows.append(
                    {
                        "operation": operation_id,
                        "rust_name": snake(operation_id),
                        "method": snake(operation_id) if wrapped else None,
                        "pointer": pointer,
                        "sentence": description,
                        "omission": omission,
                        "param_type": TYPED[omission] if wrapped else None,
                        "exchange_index_sentences": unique_indexes,
                        "wrapped": wrapped,
                    }
                )
    rows.sort(key=lambda row: (str(row["operation"]), str(row["pointer"])))
    return rows


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    spec = yaml.safe_load(SPEC.read_text())
    rows = derive(spec)
    payload = {"count": len(rows), "rows": rows}
    text = json.dumps(payload, indent=2) + "\n"
    if args.check:
        current = OUT.read_text()
        if current != text:
            raise SystemExit("subaccount_semantics.json does not match the spec")
        print(f"subaccount semantics ok ({len(rows)})")
        return 0
    OUT.write_text(text)
    print(f"wrote {OUT} ({len(rows)})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
