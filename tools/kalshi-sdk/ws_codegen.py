#!/usr/bin/env python3
"""Generate the WebSocket message types from spec/asyncapi.yaml.

Writes crates/kalshi-api/src/ws/generated.rs. ``--check`` fails when the
committed file differs from a fresh generation. Rust names for payloads and
nested objects come from ws_fields.py, so the field list and the types share
one naming table. Fields in ws_fields.NOT_SENT are optional even though the
document requires them. Every inline enum needs a Rust name in INLINE_ENUMS; an
unnamed enum, an unknown $ref, or a field type conflict stops generation.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from ws_fields import CHILD_OWNER, NOT_SENT, ROOTS, nullable  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
SPEC = ROOT / "spec" / "asyncapi.yaml"
OUT = ROOT / "crates" / "kalshi-api" / "src" / "ws" / "generated.rs"

# $ref targets with a fixed Rust type. Ids, sids and sequence numbers are unsigned.
REF_TYPES = {
    "commandId": "u64",
    "subscriptionId": "u64",
    "sequenceNumber": "u64",
}

# Component enums → Rust name.
REF_ENUMS = {
    "marketSide": "MarketSide",
    "bookSide": "BookSide",
    "orderAction": "OrderAction",
    "lastUpdateReason": "LastUpdateReason",
    "lifecyclePriceLevelStructure": "PriceLevelStructure",
}

# Inline enums and consts, keyed by (Rust owner, property).
INLINE_ENUMS = {
    ("SubscribeParams", "channels"): "Channel",
    ("SelectedLeg", "side"): "MarketSide",
    ("QuoteAcceptedBody", "accepted_side"): "MarketSide",
    ("BatchQuoteAcceptedBody", "accepted_side"): "MarketSide",
    ("SubscribeParams", "user_filter"): "UserFilter",
    ("UpdateSubscriptionParams", "action"): "SubscriptionAction",
    ("UserOrderBody", "status"): "UserOrderStatus",
    ("UserOrderBody", "self_trade_prevention_type"): "SelfTradePrevention",
    ("MarketLifecycleBody", "event_type"): "LifecycleEvent",
    ("MultivariateLifecycleBody", "event_type"): "MultivariateLifecycleEvent",
    ("MarketMetadataBody", "event_type"): "MetadataEvent",
    ("EventLifecycleBody", "collateral_return_type"): "CollateralReturnType",
    ("EventFeeBody", "fee_type_override"): "FeeType",
    ("OrderGroupBody", "event_type"): "OrderGroupEvent",
}

# Open enums keep values outside the document as `Unknown(String)`.
# Every other enum is closed: an unlisted value fails the frame.
OPEN_ENUMS = {"LastUpdateReason", "PriceLevelStructure"}

# Variant names the default casing cannot produce.
VARIANT_NAMES = {
    ("OrderAction", ""): "Unspecified",
    ("CollateralReturnType", "MECNET"): "MutuallyExclusive",
    ("CollateralReturnType", "DIRECNET"): "Directional",
    ("CollateralReturnType", ""): "None",
    ("UserFilter", ""): "All",
    ("UserFilter", "self"): "SelfOnly",
    ("PriceLevelStructure", ""): "Unspecified",
}
WORDS = {"cfbenchmarks": "CfBenchmarks", "5hz": "5Hz", "indexlist": "IndexList"}

# Older wire names still accepted on input.
ALIASES = {
    ("OrderbookSnapshotBody", "yes_dollars_fp"): "yes_dollars",
    ("OrderbookSnapshotBody", "no_dollars_fp"): "no_dollars",
}

# One Incoming variant can carry several payload shapes.
UNTAGGED = {
    "Incoming::Ok": (
        "OkBody",
        (
            ("Indices", "CfBenchmarksIndexListBody"),
            ("Underlyings", "UnderlyingListBody"),
            ("Subscriptions", "Vec<SubscribedBody>"),
            ("Markets", "OkMarketsBody"),
        ),
    ),
    "Incoming::MarketLifecycleV2": (
        "LifecycleMessage",
        (
            ("Metadata", "MarketMetadataBody"),
            ("Lifecycle", "Box<MarketLifecycleBody>"),
        ),
    ),
}
EXTRA_OK = (
    ("subscribedIndicesResponsePayload", "CfBenchmarksIndexListBody"),
    ("subscribedUnderlyingsResponsePayload", "UnderlyingListBody"),
    ("listSubscriptionsResponsePayload", "SubscribedBody"),
)


class Gen:
    def __init__(self, spec: dict) -> None:
        self.spec = spec
        self.schemas = spec["components"]["schemas"]
        self.structs: dict[str, dict] = {}
        self.enums: dict[str, list] = {}
        self.variants: dict[str, dict] = {}
        self.not_sent: set[tuple[str, str]] = set()

    def ref(self, prop: dict) -> str | None:
        ref = prop.get("$ref")
        if ref is None:
            return None
        name = ref.rsplit("/", 1)[-1]
        if name not in self.schemas:
            raise SystemExit(f"missing $ref {name}")
        return name

    def enum(self, name: str, values: list, wire: str) -> str:
        known = self.enums.setdefault(name, [])
        for value in values:
            if value is None:
                continue
            if not isinstance(value, str):
                raise SystemExit(f"{wire}: non-string enum value {value!r}")
            if value not in known:
                known.append(value)
        return name

    def type_of(self, owner: str, field: str, prop: dict, path: str) -> str:
        ref = self.ref(prop)
        if ref in REF_TYPES:
            return REF_TYPES[ref]
        node = self.schemas[ref] if ref else prop
        if "enum" in node or "const" in node:
            values = node.get("enum", [node.get("const")])
            if node.get("type") == "integer" or (
                isinstance(node.get("type"), list) and "integer" in node["type"]
            ):
                return "i64"
            name = REF_ENUMS.get(ref) if ref else INLINE_ENUMS.get((owner, field))
            if name is None:
                raise SystemExit(f"no Rust name for enum {owner}.{field} ({path})")
            return self.enum(name, values, path)
        kind = node.get("type")
        if isinstance(kind, list):
            kinds = [k for k in kind if k != "null"]
            if len(kinds) != 1:
                raise SystemExit(f"{path}: union type {kind}")
            kind = kinds[0]
        if kind == "array":
            item = node.get("items")
            if not isinstance(item, dict):
                raise SystemExit(f"{path}: array without items")
            return f"Vec<{self.type_of(owner, field, item, path + '[]')}>"
        if kind == "object" or "properties" in node:
            if not node.get("properties"):
                return "serde_json::Map<String, serde_json::Value>"
            child = CHILD_OWNER.get((owner, field))
            if path == "marketMetadataUpdatedPayload.msg":
                child = "MarketMetadataBody"
            if child is None:
                raise SystemExit(f"no Rust owner for object {owner}.{field} ({path})")
            self.struct(child, node, path)
            return child
        return {
            "string": "String",
            "integer": "i64",
            "number": "serde_json::Number",
            "boolean": "bool",
        }.get(kind) or self.fail(f"{path}: unsupported type {kind!r}")

    @staticmethod
    def fail(message: str):
        raise SystemExit(message)

    def fields(self, owner: str, schema: dict, path: str, skip: tuple) -> list[dict]:
        required = set(schema.get("required") or [])
        out = []
        for name, prop in (schema.get("properties") or {}).items():
            if name in skip:
                continue
            not_sent = (owner, name) in NOT_SENT
            if not_sent:
                if name not in required:
                    raise SystemExit(f"NOT_SENT {owner}.{name} is not required in the document ({path})")
                self.not_sent.add((owner, name))
            out.append({
                "name": name,
                "ty": self.type_of(owner, name, prop, f"{path}.{name}"),
                "required": name in required and not not_sent,
                "nullable": nullable(self.schemas, prop),
                "deprecated": bool(prop.get("deprecated")),
                "not_sent": not_sent,
            })
        return out

    def merge(self, table: dict, owner: str, fields: list[dict], path: str) -> None:
        if owner not in table:
            table[owner] = {"fields": fields, "seen": 1}
            return
        entry = table[owner]
        entry["seen"] += 1
        names = {f["name"] for f in fields}
        by_name = {f["name"]: f for f in entry["fields"]}
        for field in fields:
            old = by_name.get(field["name"])
            if old is None:
                entry["fields"].append(dict(field, required=False, partial=True))
                continue
            if old["ty"] != field["ty"]:
                raise SystemExit(f"{owner}.{field['name']} is {old['ty']} and {field['ty']} ({path})")
            old["required"] = old["required"] and field["required"]
            old["nullable"] = old["nullable"] or field["nullable"]
            old["deprecated"] = old["deprecated"] or field["deprecated"]
        for old in entry["fields"]:
            if old["name"] not in names:
                old["required"] = False

    def struct(self, owner: str, schema: dict, path: str) -> None:
        self.merge(self.structs, owner, self.fields(owner, schema, path, ()), path)

    def pointer(self, ref: dict, where: str) -> dict:
        node = self.spec
        for part in (ref.get("$ref") or "").lstrip("#/").split("/"):
            node = node.get(part) if isinstance(node, dict) else None
            if node is None:
                raise SystemExit(f"{where}: broken $ref {ref}")
        return node

    def reconcile(self) -> None:
        """Every tagged message in the document must have a Rust owner."""
        rooted = {name for name, _ in ROOTS} | {name for name, _ in EXTRA_OK}
        for key, message in (self.spec["components"].get("messages") or {}).items():
            payload = message.get("payload") or {}
            name = self.ref(payload)
            node = self.schemas[name] if name else payload
            props = node.get("properties") or {}
            tag = (props.get("type") or {}).get("const") or (props.get("cmd") or {}).get("const")
            if tag is None:
                continue
            if name not in rooted:
                raise SystemExit(
                    f"message {key} ({name or 'inline payload'}, tag {tag!r}) has no entry in ROOTS or EXTRA_OK"
                )
        for operation, item in (self.spec.get("operations") or {}).items():
            for ref in item.get("messages") or []:
                node = self.pointer(ref, operation)
                while "$ref" in node:
                    node = self.pointer(node, operation)
                payload = node.get("payload") or {}
                name = self.ref(payload)
                props = (self.schemas[name] if name else payload).get("properties") or {}
                tag = (props.get("type") or {}).get("const") or (props.get("cmd") or {}).get("const")
                if tag is not None and name not in rooted:
                    raise SystemExit(
                        f"operation {operation}: {name or 'inline payload'} (tag {tag!r}) has no entry in ROOTS or EXTRA_OK"
                    )
        for name, _ in EXTRA_OK:
            tag = ((self.schemas.get(name) or {}).get("properties") or {}).get("type", {}).get("const")
            if tag != "ok":
                raise SystemExit(f"EXTRA_OK {name} has type const {tag!r}, expected 'ok'")

    def run(self) -> None:
        self.reconcile()
        for schema_name, owner in ROOTS:
            schema = self.schemas.get(schema_name) or self.fail(f"missing schema {schema_name}")
            props = schema.get("properties") or {}
            tag_key = "cmd" if owner.startswith("ClientCommand::") else "type"
            tag = props.get(tag_key, {}).get("const")
            if tag is None:
                raise SystemExit(f"{schema_name} has no const {tag_key}")
            fields = self.fields(owner, schema, schema_name, ("type", "cmd"))
            if owner in UNTAGGED:
                for field in fields:
                    if field["name"] == "msg":
                        field["ty"] = UNTAGGED[owner][0]
            self.merge(self.variants, owner, fields, schema_name)
            known = self.variants[owner].setdefault("tag", tag)
            if known != tag:
                raise SystemExit(f"{owner} merges tag {known!r} and {tag!r} ({schema_name})")
        for schema_name, owner in EXTRA_OK:
            schema = self.schemas[schema_name]
            msg = schema["properties"]["msg"]
            node = msg["items"] if msg.get("type") == "array" else msg
            self.struct(owner, node, f"{schema_name}.msg")
            fields = self.fields("Incoming::Ok", {**schema, "properties": {
                k: v for k, v in schema["properties"].items() if k != "msg"}},
                schema_name, ("type",))
            fields.append({"name": "msg", "ty": "OkBody", "required": "msg" in schema.get("required", []),
                           "nullable": False, "deprecated": False, "not_sent": False})
            self.merge(self.variants, "Incoming::Ok", fields, schema_name)
        for name, channel in self.spec.get("channels", {}).items():
            address = channel.get("address")
            if address and address != "/":
                self.enum("Channel", [address], f"channels.{name}")
        if self.not_sent != NOT_SENT:
            raise SystemExit(f"NOT_SENT entries not in the document: {sorted(NOT_SENT - self.not_sent)}")


def variant_name(enum: str, value: str) -> str:
    if (enum, value) in VARIANT_NAMES:
        return VARIANT_NAMES[(enum, value)]
    parts = [p for p in value.replace("-", "_").split("_") if p]
    if not parts:
        raise SystemExit(f"{enum}: no variant name for {value!r}")
    name = "".join(WORDS.get(p, p[:1].upper() + p[1:]) for p in parts)
    if not name[0].isalpha():
        raise SystemExit(f"{enum}: variant {value!r} needs a name")
    return name


def field_lines(field: dict, indent: str, public: bool) -> list[str]:
    lines = []
    attrs = []
    ty = field["ty"]
    if field["required"] and field["nullable"]:
        ty = f"Option<{ty}>"
        attrs.append('deserialize_with = "required_nullable"')
    elif not field["required"]:
        ty = f"Option<{ty}>"
        attrs.append('default, skip_serializing_if = "Option::is_none"')
    alias = field.get("alias")
    if alias:
        attrs.append(f'alias = "{alias}"')
    if field.get("not_sent"):
        lines.append(f"{indent}/// Required in the AsyncAPI document, but the exchange does not send it.")
    if field["deprecated"]:
        lines.append(f"{indent}/// Deprecated in the AsyncAPI document. The key is still sent.")
        lines.append(f"{indent}#[deprecated]")
    if attrs:
        lines.append(f"{indent}#[serde({', '.join(attrs)})]")
    vis = "pub " if public else ""
    lines.append(f"{indent}{vis}{field['name']}: {ty},")
    return lines


def render(gen: Gen) -> str:
    out = [
        "// Generated by tools/kalshi-sdk/ws_codegen.py from spec/asyncapi.yaml. Do not edit.",
        "#![allow(deprecated)]",
        "",
        "use serde::de::Deserializer;",
        "use serde::{Deserialize, Serialize};",
        "",
        "/// The key is required. JSON null is `None`. A missing key is an error.",
        "pub(super) fn required_nullable<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>",
        "where",
        "    T: Deserialize<'de>,",
        "    D: Deserializer<'de>,",
        "{",
        "    Option::<T>::deserialize(deserializer)",
        "}",
    ]
    for name in sorted(gen.enums):
        values = gen.enums[name]
        variants = [(variant_name(name, v), v) for v in values]
        out.append("")
        if name in OPEN_ENUMS:
            out.append(f"/// Values outside the AsyncAPI list are kept as [`{name}::Unknown`].")
            out.append("#[derive(Debug, Clone, PartialEq, Eq)]")
            out.append(f"pub enum {name} {{")
            out += [f"    {v}," for v, _ in variants]
            out.append("    Unknown(String),")
            out.append("}")
            out.append("")
            out.append(f"impl Serialize for {name} {{")
            out.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
            out.append("        serializer.serialize_str(match self {")
            out += [f'            Self::{v} => "{w}",' for v, w in variants]
            out.append("            Self::Unknown(value) => value,")
            out.append("        })")
            out.append("    }")
            out.append("}")
            out.append("")
            out.append(f"impl<'de> Deserialize<'de> for {name} {{")
            out.append("    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
            out.append("        let value = String::deserialize(deserializer)?;")
            out.append("        Ok(match value.as_str() {")
            out += [f'            "{w}" => Self::{v},' for v, w in variants]
            out.append("            _ => Self::Unknown(value),")
            out.append("        })")
            out.append("    }")
            out.append("}")
        else:
            out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]")
            out.append(f"pub enum {name} {{")
            for v, w in variants:
                out.append(f'    #[serde(rename = "{w}")]')
                out.append(f"    {v},")
            out.append("}")
    for owner in sorted(gen.structs):
        out.append("")
        out.append("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]")
        out.append(f"pub struct {owner} {{")
        for field in gen.structs[owner]["fields"]:
            field = dict(field, alias=ALIASES.get((owner, field["name"])))
            out += field_lines(field, "    ", True)
        out.append("}")
    for owner, (name, arms) in UNTAGGED.items():
        out.append("")
        out.append("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]")
        out.append("#[serde(untagged)]")
        out.append(f"pub enum {name} {{")
        out += [f"    {arm}({ty})," for arm, ty in arms]
        out.append("}")
    for enum, tag_key in (("ClientCommand", "cmd"), ("Incoming", "type")):
        out.append("")
        out.append("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]")
        out.append(f'#[serde(tag = "{tag_key}")]')
        out.append(f"pub enum {enum} {{")
        owners = [o for _, o in ROOTS if o.startswith(enum + "::")]
        for owner in dict.fromkeys(owners):
            entry = gen.variants[owner]
            out.append(f'    #[serde(rename = "{entry["tag"]}")]')
            out.append(f"    {owner.split('::', 1)[1]} {{")
            for field in entry["fields"]:
                out += field_lines(field, "        ", False)
            out.append("    },")
        out.append("}")
    return "\n".join(out) + "\n"


def rustfmt(text: str) -> str:
    result = subprocess.run(
        ["rustfmt", "--edition", "2021", "--emit", "stdout"],
        input=text, capture_output=True, text=True, check=False,
    )
    if result.returncode != 0:
        raise SystemExit(f"rustfmt failed:\n{result.stderr}")
    return result.stdout


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    gen = Gen(yaml.safe_load(SPEC.read_text()))
    gen.run()
    text = rustfmt(render(gen))
    if args.check:
        if not OUT.exists() or OUT.read_text() != text:
            raise SystemExit(f"{OUT.relative_to(ROOT)} does not match spec/asyncapi.yaml; run tools/kalshi-sdk/ws_codegen.py")
        print("ws codegen ok")
        return 0
    OUT.write_text(text)
    print(f"wrote {OUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
