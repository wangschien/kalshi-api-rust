#!/usr/bin/env python3
"""Overlay pinned OpenAPI before openapi-generator runs.

1. Set format: int64 on every listed format-less integer. openapi-generator
   7.17.0 maps a format-less integer to i32 in the Rust client and does not
   honor --type-mappings integer=i64. Integers that already declare int32 or
   int64 are not modified.

2. Quote EventContractExitTrigger.side enum members as YAML strings
   "yes"/"no". The official document writes `enum: [yes, no]`; YAML 1.1
   reads those as booleans, and the generator then emits serde rename
   "true"/"false". Only that one ExitTrigger side enum is rewritten; other
   schemas are untouched.

The overlay is idempotent: running it again on its own output changes nothing.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

import yaml

# JSON pointers of every format-less integer in the pinned spec.
# A pointer that is not in this list, or a listed pointer that is not a
# format-less integer, fails the overlay.
ENTRIES: tuple[str, ...] = (
    "/paths/~1series~1{series_ticker}~1markets~1{ticker}~1candlesticks/get/parameters/4/schema",
    "/paths/~1markets~1{ticker}~1orderbook/get/parameters/1/schema",
    "/paths/~1events/get/parameters/0/schema",
    "/paths/~1events~1multivariate/get/parameters/0/schema",
    "/paths/~1milestones/get/parameters/0/schema",
    "/paths/~1incentive_programs/get/parameters/3/schema",
    "/paths/~1fcm~1orders/get/parameters/8/schema",
    "/paths/~1fcm~1positions/get/parameters/5/schema",
    "/paths/~1historical~1markets~1{ticker}~1candlesticks/get/parameters/3/schema",
    "/components/parameters/SubaccountQuery/schema",
    "/components/parameters/SubaccountQueryDefaultPrimary/schema",
    "/components/schemas/ExchangeIndex",
    "/components/schemas/ApiKey/properties/subaccount",
    "/components/schemas/CreateApiKeyRequest/properties/subaccount",
    "/components/schemas/GenerateApiKeyRequest/properties/subaccount",
    "/components/schemas/BucketLimit/properties/refill_rate",
    "/components/schemas/BucketLimit/properties/bucket_capacity",
    "/components/schemas/EndpointTokenCost/properties/cost",
    "/components/schemas/GetAccountEndpointCostsResponse/properties/default_cost",
    "/components/schemas/WeatherIndexPoint/properties/contributors",
    "/components/schemas/CreateSubaccountResponse/properties/subaccount_number",
    "/components/schemas/ApplySubaccountTransferRequest/properties/from_subaccount",
    "/components/schemas/ApplySubaccountTransferRequest/properties/to_subaccount",
    "/components/schemas/SubaccountBalance/properties/subaccount_number",
    "/components/schemas/SubaccountBalance/properties/exchange_index",
    "/components/schemas/SubaccountTransfer/properties/from_subaccount",
    "/components/schemas/SubaccountTransfer/properties/to_subaccount",
    "/components/schemas/SubaccountTransfer/properties/exchange_index",
    "/components/schemas/UpdateSubaccountNettingRequest/properties/subaccount_number",
    "/components/schemas/SubaccountNettingConfig/properties/subaccount_number",
    "/components/schemas/SubaccountNettingConfig/properties/exchange_index",
    "/components/schemas/Settlement/properties/revenue",
    "/components/schemas/Settlement/properties/value",
    "/components/schemas/GetPortfolioRestingOrderTotalValueResponse/properties/total_resting_order_value",
    "/components/schemas/Order/properties/subaccount_number",
    "/components/schemas/Fill/properties/subaccount_number",
    "/components/schemas/TargetBalanceAllocation/properties/exchange_index",
    "/components/schemas/TargetBalanceAllocation/properties/percent",
    "/components/schemas/TargetBalanceAllocationInput/properties/exchange_index",
    "/components/schemas/TargetBalanceAllocationInput/properties/percent",
    "/components/schemas/IntraExchangeInstanceTransferRequest/properties/source_exchange_shard",
    "/components/schemas/IntraExchangeInstanceTransferRequest/properties/destination_exchange_shard",
    "/components/schemas/IntraExchangeInstanceTransferRequest/properties/source_subaccount",
    "/components/schemas/IntraExchangeInstanceTransferRequest/properties/destination_subaccount",
    "/components/schemas/IntraExchangeInstanceTransfer/properties/source_exchange_shard",
    "/components/schemas/IntraExchangeInstanceTransfer/properties/destination_exchange_shard",
    "/components/schemas/CreateOrderGroupRequest/properties/subaccount",
    "/components/schemas/CreateOrderGroupResponse/properties/subaccount",
    "/components/schemas/ProposeBlockTradeRequest/properties/buyer_subaccount",
    "/components/schemas/ProposeBlockTradeRequest/properties/seller_subaccount",
    "/components/schemas/AcceptBlockTradeProposalRequest/properties/subaccount",
    "/components/schemas/RFQ/properties/creator_subaccount",
    "/components/schemas/CreateRFQRequest/properties/contracts",
    "/components/schemas/CreateRFQRequest/properties/subaccount",
    "/components/schemas/Quote/properties/creator_subaccount",
    "/components/schemas/Quote/properties/rfq_creator_subaccount",
    "/components/schemas/CreateQuoteRequest/properties/subaccount",
    "/components/schemas/CreateOrderRequest/properties/count",
    "/components/schemas/CreateOrderRequest/properties/yes_price",
    "/components/schemas/CreateOrderRequest/properties/no_price",
    "/components/schemas/CreateOrderRequest/properties/buy_max_cost",
    "/components/schemas/CreateOrderRequest/properties/sell_position_floor",
    "/components/schemas/CreateOrderRequest/properties/subaccount",
    "/components/schemas/BatchCancelOrdersRequestOrder/properties/subaccount",
    "/components/schemas/AmendOrderRequest/properties/subaccount",
    "/components/schemas/AmendOrderRequest/properties/yes_price",
    "/components/schemas/AmendOrderRequest/properties/no_price",
    "/components/schemas/AmendOrderRequest/properties/count",
    "/components/schemas/CreateOrderV2Request/properties/subaccount",
    "/components/schemas/BatchCancelOrdersV2Request/properties/orders/items/properties/subaccount",
    "/components/schemas/Market/properties/settlement_timer_seconds",
    "/components/schemas/CreateRFQBatchQuoteRequest/properties/subaccount",
    "/components/schemas/CreateRFQBatchRequest/properties/subaccount",
    "/components/schemas/RFQBatch/properties/creator_subaccount",
    "/components/schemas/RFQBatchQuote/properties/creator_subaccount",
    "/components/schemas/RFQBatchQuote/properties/rfq_creator_subaccount",
    "/components/schemas/EventContractExitTrigger/properties/subaccount_number",
)


EXIT_TRIGGER_SIDE = "/components/schemas/EventContractExitTrigger/properties/side"


def _resolve(root: yaml.Node, pointer: str) -> yaml.Node:
    """Resolve a JSON pointer against a composed YAML mapping/sequence tree."""
    if not pointer.startswith("/"):
        raise SystemExit(f"pointer must be absolute: {pointer}")
    node: yaml.Node = root
    for raw in pointer[1:].split("/"):
        key = raw.replace("~1", "/").replace("~0", "~")
        items = _items(node)
        if items is None:
            raise SystemExit(f"cannot resolve {pointer}: {key} under non-mapping")
        keyed = {name: value for name, _key, value in items}
        if key not in keyed:
            raise SystemExit(f"cannot resolve {pointer}: missing {key}")
        node = keyed[key]
    return node


def coerce_exit_trigger_side_enum(text: str, *, enabled: bool = True) -> str:
    """Rewrite EventContractExitTrigger.side enum: [yes, no] to quoted strings.

    When enabled is False the document is returned unchanged (for gate
    discrimination evidence only).
    """
    if not enabled:
        return text
    root = yaml.compose(text)
    if root is None:
        raise SystemExit("empty spec")
    side = _resolve(root, EXIT_TRIGGER_SIDE)
    items = _items(side)
    if items is None:
        raise SystemExit(f"{EXIT_TRIGGER_SIDE} is not a mapping")
    keyed = {name: (key, value) for name, key, value in items}
    type_pair = keyed.get("type")
    enum_pair = keyed.get("enum")
    if (
        type_pair is None
        or not isinstance(type_pair[1], yaml.ScalarNode)
        or str(type_pair[1].value) != "string"
    ):
        raise SystemExit(f"{EXIT_TRIGGER_SIDE} type is not string")
    if enum_pair is None or not isinstance(enum_pair[1], yaml.SequenceNode):
        raise SystemExit(f"{EXIT_TRIGGER_SIDE} enum is missing")
    members = enum_pair[1].value
    if len(members) != 2:
        raise SystemExit(
            f"{EXIT_TRIGGER_SIDE} enum length {len(members)}, expected 2"
        )
    tags = [m.tag for m in members]
    values = [str(m.value) for m in members]
    if values != ["yes", "no"]:
        raise SystemExit(
            f"{EXIT_TRIGGER_SIDE} enum values {values}, expected ['yes', 'no']"
        )
    if tags == ["tag:yaml.org,2002:str", "tag:yaml.org,2002:str"]:
        return text
    if tags != ["tag:yaml.org,2002:bool", "tag:yaml.org,2002:bool"]:
        raise SystemExit(
            f"{EXIT_TRIGGER_SIDE} enum tags {tags}, expected bool or str"
        )
    line_no = members[0].start_mark.line
    lines = text.splitlines(keepends=True)
    line = lines[line_no]
    new_line, count = re.subn(
        r"enum:\s*\[\s*yes\s*,\s*no\s*\]",
        "enum: ['yes', 'no']",
        line,
        count=1,
    )
    if count != 1:
        raise SystemExit(
            f"failed to quote ExitTrigger side enum on line {line_no + 1}: {line!r}"
        )
    lines[line_no] = new_line
    return "".join(lines)


def _escape(key: str) -> str:
    return key.replace("~", "~0").replace("/", "~1")


def _items(node: yaml.Node) -> list[tuple[str, yaml.Node, yaml.Node]] | None:
    if not isinstance(node, yaml.MappingNode):
        return None
    out = []
    for key, value in node.value:
        if not isinstance(key, yaml.ScalarNode):
            raise SystemExit(
                f"non-scalar mapping key at line {key.start_mark.line + 1}"
            )
        out.append((str(key.value), key, value))
    return out


def classify(root: yaml.Node) -> tuple[dict[str, int], dict[str, str]]:
    """Return format-less integer lines and format-by-pointer for every integer."""
    missing: dict[str, int] = {}
    formats: dict[str, str] = {}

    def walk(node: yaml.Node, pointer: str) -> None:
        items = _items(node)
        if items is None:
            if isinstance(node, yaml.SequenceNode):
                for index, child in enumerate(node.value):
                    walk(child, f"{pointer}/{index}")
            return
        keyed = {name: (key, value) for name, key, value in items}
        type_pair = keyed.get("type")
        if (
            type_pair is not None
            and isinstance(type_pair[1], yaml.ScalarNode)
            and type_pair[1].value == "integer"
            and "$ref" not in keyed
        ):
            if "format" not in keyed:
                missing[pointer] = type_pair[0].start_mark.line
                formats[pointer] = ""
            else:
                fmt_node = keyed["format"][1]
                if not isinstance(fmt_node, yaml.ScalarNode):
                    raise SystemExit(f"non-scalar format at {pointer}")
                formats[pointer] = str(fmt_node.value)
        for name, _key, value in items:
            walk(value, f"{pointer}/{_escape(name)}")

    walk(root, "")
    return missing, formats


def apply_overlay(text: str, *, coerce_side_enum: bool = True) -> str:
    text = coerce_exit_trigger_side_enum(text, enabled=coerce_side_enum)
    root = yaml.compose(text)
    if root is None:
        raise SystemExit("empty spec")
    missing, formats = classify(root)
    entries = set(ENTRIES)
    if len(ENTRIES) != len(entries):
        raise SystemExit("overlay entry list contains duplicates")

    unexpected = sorted(set(missing) - entries)
    unknown = sorted(entries - set(formats))
    wrong = sorted(
        pointer
        for pointer in entries
        if pointer in formats and formats[pointer] not in ("", "int64")
    )
    if unexpected or unknown or wrong:
        details = []
        if unexpected:
            details.append("format-less integers not listed:\n  " + "\n  ".join(unexpected))
        if unknown:
            details.append("listed pointers that are not integers:\n  " + "\n  ".join(unknown))
        if wrong:
            details.append("listed pointers with a format other than int64:\n  " + "\n  ".join(wrong))
        raise SystemExit("\n".join(details))

    int32_before = {pointer for pointer, fmt in formats.items() if fmt == "int32"}
    insert_at = [missing[pointer] for pointer in ENTRIES if formats[pointer] == ""]
    lines = text.splitlines(keepends=True)
    for line_no in sorted(insert_at, reverse=True):
        line = lines[line_no]
        body = line.lstrip(" ")
        if not body.startswith("type:") or "integer" not in body.split("#", 1)[0]:
            raise SystemExit(f"line {line_no + 1} is not 'type: integer': {line!r}")
        indent = line[: len(line) - len(body)]
        newline = "\r\n" if line.endswith("\r\n") else "\n"
        lines.insert(line_no + 1, f"{indent}format: int64{newline}")

    updated = "".join(lines)
    root2 = yaml.compose(updated)
    missing2, formats2 = classify(root2)
    if missing2:
        raise SystemExit(f"format-less integers remain: {sorted(missing2)}")
    int32_after = {pointer for pointer, fmt in formats2.items() if fmt == "int32"}
    if int32_before != int32_after:
        raise SystemExit("int32 integers changed")
    not_int64 = [pointer for pointer in ENTRIES if formats2.get(pointer) != "int64"]
    if not_int64:
        raise SystemExit("listed integers are not int64: " + ", ".join(not_int64))
    # Enum coerce runs before int64 inserts; re-run for idempotence and to
    # catch an unexpected rewrite of the ExitTrigger side line.
    updated = coerce_exit_trigger_side_enum(updated, enabled=coerce_side_enum)
    return updated


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("spec", type=Path)
    parser.add_argument("output", type=Path, nargs="?")
    parser.add_argument("--check", action="store_true", help="apply and require idempotence")
    parser.add_argument(
        "--skip-exit-trigger-side-coerce",
        action="store_true",
        help="leave EventContractExitTrigger.side enum unquoted (discrimination only)",
    )
    args = parser.parse_args(argv)
    coerce = not args.skip_exit_trigger_side_coerce
    original = args.spec.read_text()
    once = apply_overlay(original, coerce_side_enum=coerce)
    twice = apply_overlay(once, coerce_side_enum=coerce)
    if once != twice:
        raise SystemExit("overlay is not idempotent")
    if args.check and args.output is None:
        print(
            f"overlay ok: {len(ENTRIES)} integers set to int64; "
            f"exit_trigger_side_coerce={'on' if coerce else 'off'}"
        )
        return 0
    if args.output is None:
        raise SystemExit("output path is required unless --check")
    args.output.write_text(once)
    print(
        f"wrote {args.output} ({len(ENTRIES)} format: int64 insertions; "
        f"exit_trigger_side_coerce={'on' if coerce else 'off'})"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
