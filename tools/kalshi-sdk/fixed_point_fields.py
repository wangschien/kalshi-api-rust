#!/usr/bin/env python3
"""Export FixedPoint model fields from the pinned OpenAPI document.

A field counts when a component schema property, or a property of an inline
object under it, is a direct $ref or a single-entry allOf $ref to
FixedPointCount or FixedPointDollars. The committed list is checked by drift.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
SPEC = ROOT / "spec" / "openapi.yaml"
MODELS = ROOT / "crates" / "kalshi-api-openapi" / "src" / "models"
OUT = ROOT / "tools" / "kalshi-sdk" / "fixed_point_fields.json"
STRUCT_RE = re.compile(r"^pub struct (\w+)", re.M)


def fp_kind(prop: object) -> str | None:
    if not isinstance(prop, dict):
        return None
    ref = prop.get("$ref")
    if isinstance(ref, str) and ref.rsplit("/", 1)[-1].startswith("FixedPoint"):
        return ref.rsplit("/", 1)[-1]
    allof = prop.get("allOf")
    if isinstance(allof, list) and len(allof) == 1 and isinstance(allof[0], dict):
        ref = allof[0].get("$ref")
        if isinstance(ref, str) and ref.rsplit("/", 1)[-1].startswith("FixedPoint"):
            return ref.rsplit("/", 1)[-1]
    return None


def walk(owner: str, node: object, hits: list[tuple[str, str, str]]) -> None:
    if not isinstance(node, dict):
        return
    props = node.get("properties")
    if isinstance(props, dict):
        for name, prop in props.items():
            kind = fp_kind(prop)
            if kind:
                hits.append((owner, name, kind))
            if isinstance(prop, dict):
                if prop.get("type") == "array" and isinstance(prop.get("items"), dict):
                    walk(f"{owner}.{name}[]", prop["items"], hits)
                elif "properties" in prop:
                    walk(f"{owner}.{name}", prop, hits)


def rust_structs() -> dict[str, str]:
    found: dict[str, str] = {}
    for path in MODELS.glob("*.rs"):
        for name in STRUCT_RE.findall(path.read_text()):
            key = name.casefold()
            if key in found and found[key] != name:
                raise SystemExit(f"struct casefold collision {found[key]} {name}")
            found[key] = name
    return found


def rust_name(owner: str, structs: dict[str, str]) -> str:
    if "." not in owner and "[]" not in owner:
        key = owner.casefold()
        if key not in structs:
            raise SystemExit(f"no generated struct for schema {owner}")
        return structs[key]
    parent, _rest = owner.split(".", 1)
    parent_key = parent.casefold()
    matches = [name for key, name in structs.items() if key.startswith(parent_key) and key.endswith("inner")]
    if len(matches) != 1:
        raise SystemExit(f"inline owner {owner} matches {matches}")
    return matches[0]


def derive() -> list[dict[str, str]]:
    spec = yaml.safe_load(SPEC.read_text())
    hits: list[tuple[str, str, str]] = []
    for name, schema in spec["components"]["schemas"].items():
        if name.startswith("FixedPoint"):
            continue
        walk(name, schema, hits)
    structs = rust_structs()
    rows = []
    for owner, field, kind in hits:
        rows.append(
            {
                "struct": rust_name(owner, structs),
                "field": field,
                "schema": owner,
                "kind": kind,
            }
        )
    rows.sort(key=lambda row: (row["struct"], row["field"], row["schema"]))
    return rows


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    rows = derive()
    named = sum(1 for row in rows if "." not in row["schema"])
    payload = {
        "count": len(rows),
        "named_schema_properties": named,
        "fields": rows,
    }
    text = json.dumps(payload, indent=2) + "\n"
    if args.check:
        current = OUT.read_text()
        if current != text:
            raise SystemExit("fixed_point_fields.json does not match the spec")
        print(f"fixed-point fields ok ({len(rows)})")
        return 0
    OUT.write_text(text)
    print(f"wrote {OUT} ({len(rows)})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
