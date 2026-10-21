#!/usr/bin/env python3
"""Export typed WebSocket fields from the pinned AsyncAPI document.

Each row is one property of a message this crate types by hand. ``presence``
is ``required``, ``optional``, or ``required_nullable`` (the key must appear;
JSON null is accepted). The committed list is checked by drift. Nested
objects that this crate types are expanded. Channel payloads outside that set
are not listed.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
SPEC = ROOT / "spec" / "asyncapi.yaml"
OUT = ROOT / "tools" / "kalshi-sdk" / "ws_fields.json"

# Payload schema → Rust enum variant. ``type`` / ``cmd`` are the discriminant.
ROOTS = (
    ("subscribeCommandPayload", "ClientCommand::Subscribe"),
    ("unsubscribeCommandPayload", "ClientCommand::Unsubscribe"),
    ("updateSubscriptionCommandPayload", "ClientCommand::UpdateSubscription"),
    ("cfbenchmarksUpdateSubscriptionCommandPayload", "ClientCommand::UpdateSubscription"),
    ("pythUpdateSubscriptionCommandPayload", "ClientCommand::UpdateSubscription"),
    ("listSubscriptionsCommandPayload", "ClientCommand::ListSubscriptions"),
    ("subscribedResponsePayload", "Incoming::Subscribed"),
    ("unsubscribedResponsePayload", "Incoming::Unsubscribed"),
    ("okResponsePayload", "Incoming::Ok"),
    ("errorResponsePayload", "Incoming::Error"),
    ("orderbookSnapshotPayload", "Incoming::OrderbookSnapshot"),
    ("orderbookDeltaPayload", "Incoming::OrderbookDelta"),
    ("tradePayload", "Incoming::Trade"),
    ("fillPayload", "Incoming::Fill"),
    ("marketLifecycleV2Payload", "Incoming::MarketLifecycleV2"),
    ("marketPositionPayload", "Incoming::MarketPosition"),
    ("userOrderPayload", "Incoming::UserOrder"),
    ("pythValuePayload", "Incoming::PythValue"),
    ("pythUnderlyingListPayload", "Incoming::PythUnderlyingList"),
    ("tickerPayload", "Incoming::Ticker"),
    ("multivariateMarketLifecyclePayload", "Incoming::MultivariateMarketLifecycle"),
    ("marketMetadataUpdatedPayload", "Incoming::MarketLifecycleV2"),
    ("eventLifecyclePayload", "Incoming::EventLifecycle"),
    ("eventFeeUpdatePayload", "Incoming::EventFeeUpdate"),
    ("orderGroupUpdatesPayload", "Incoming::OrderGroupUpdates"),
    ("rfqCreatedPayload", "Incoming::RfqCreated"),
    ("rfqDeletedPayload", "Incoming::RfqDeleted"),
    ("quoteCreatedPayload", "Incoming::QuoteCreated"),
    ("quoteAcceptedPayload", "Incoming::QuoteAccepted"),
    ("quoteExecutedPayload", "Incoming::QuoteExecuted"),
    ("rfqBatchCreatedPayload", "Incoming::RfqBatchCreated"),
    ("rfqBatchDeletedPayload", "Incoming::RfqBatchDeleted"),
    ("batchQuoteCreatedPayload", "Incoming::BatchQuoteCreated"),
    ("rfqBatchBestQuotesPayload", "Incoming::RfqBatchBestQuotes"),
    ("batchQuoteAcceptedPayload", "Incoming::BatchQuoteAccepted"),
    ("rfqBatchExecutedPayload", "Incoming::RfqBatchExecuted"),
    ("batchQuoteExecutedPayload", "Incoming::BatchQuoteExecuted"),
    ("cfbenchmarksValuePayload", "Incoming::CfBenchmarksValue"),
    ("cfbenchmarksIndexListPayload", "Incoming::CfBenchmarksIndexList"),
    ("cfbenchmarksValue5HzPayload", "Incoming::CfBenchmarksValue5Hz"),
    (
        "cfbenchmarks5HzIndexListPayload",
        "Incoming::CfBenchmarksIndexList5Hz",
    ),
)

# (parent owner, property) → Rust type of a nested object or array of objects.
CHILD_OWNER = {
    ("ClientCommand::Subscribe", "params"): "SubscribeParams",
    ("ClientCommand::Unsubscribe", "params"): "UnsubscribeParams",
    ("ClientCommand::UpdateSubscription", "params"): "UpdateSubscriptionParams",
    ("Incoming::Subscribed", "msg"): "SubscribedBody",
    ("Incoming::Ok", "msg"): "OkMarketsBody",
    ("Incoming::Error", "msg"): "ErrorBody",
    ("Incoming::OrderbookSnapshot", "msg"): "OrderbookSnapshotBody",
    ("Incoming::OrderbookDelta", "msg"): "OrderbookDeltaBody",
    ("Incoming::Trade", "msg"): "TradeBody",
    ("Incoming::Fill", "msg"): "FillBody",
    ("Incoming::MarketLifecycleV2", "msg"): "MarketLifecycleBody",
    ("Incoming::MarketPosition", "msg"): "MarketPositionBody",
    ("Incoming::UserOrder", "msg"): "UserOrderBody",
    ("Incoming::CfBenchmarksValue", "msg"): "CfBenchmarksValueBody",
    ("CfBenchmarksValueBody", "avg_60s_data"): "CfBenchmarksAvgData",
    (
        "CfBenchmarksValueBody",
        "last_60s_windowed_average_15min",
    ): "CfBenchmarksAvgData",
    ("Incoming::CfBenchmarksIndexList", "msg"): "CfBenchmarksIndexListBody",
    ("Incoming::CfBenchmarksValue5Hz", "msg"): "CfBenchmarksValue5HzBody",
    ("Incoming::CfBenchmarksIndexList5Hz", "msg"): "CfBenchmarksIndexListBody",
    ("MarketLifecycleBody", "additional_metadata"): "LifecycleAdditionalMetadata",
    ("MarketLifecycleBody", "price_ranges"): "PriceRange",
    ("Incoming::PythValue", "msg"): "PythValueBody",
    ("Incoming::PythUnderlyingList", "msg"): "UnderlyingListBody",
    ("Incoming::Ticker", "msg"): "TickerBody",
    ("Incoming::MultivariateMarketLifecycle", "msg"): "MultivariateLifecycleBody",
    ("MultivariateLifecycleBody", "additional_metadata"): "LifecycleAdditionalMetadata",
    ("Incoming::EventLifecycle", "msg"): "EventLifecycleBody",
    ("Incoming::EventFeeUpdate", "msg"): "EventFeeBody",
    ("Incoming::OrderGroupUpdates", "msg"): "OrderGroupBody",
    ("Incoming::RfqCreated", "msg"): "RfqCreatedBody",
    ("RfqCreatedBody", "mve_selected_legs"): "SelectedLeg",
    ("Incoming::RfqDeleted", "msg"): "RfqDeletedBody",
    ("Incoming::QuoteCreated", "msg"): "QuoteCreatedBody",
    ("Incoming::QuoteAccepted", "msg"): "QuoteAcceptedBody",
    ("Incoming::QuoteExecuted", "msg"): "QuoteExecutedBody",
    ("Incoming::RfqBatchCreated", "msg"): "RfqBatchCreatedBody",
    ("RfqBatchCreatedBody", "legs"): "SelectedLeg",
    ("RfqBatchCreatedBody", "rfqs"): "RfqBatchChildRfq",
    ("Incoming::RfqBatchDeleted", "msg"): "RfqBatchDeletedBody",
    ("Incoming::BatchQuoteCreated", "msg"): "BatchQuoteCreatedBody",
    ("BatchQuoteCreatedBody", "quotes"): "BatchQuoteCreatedChild",
    ("Incoming::RfqBatchBestQuotes", "msg"): "RfqBatchBestQuotesBody",
    ("RfqBatchBestQuotesBody", "rfqs"): "RfqBatchBestQuote",
    ("Incoming::BatchQuoteAccepted", "msg"): "BatchQuoteAcceptedBody",
    ("BatchQuoteAcceptedBody", "quotes"): "BatchQuoteAcceptedChild",
    ("Incoming::RfqBatchExecuted", "msg"): "RfqBatchExecutedBody",
    ("Incoming::BatchQuoteExecuted", "msg"): "BatchQuoteExecutedBody",
}


def resolve(schemas: dict, prop: object) -> dict:
    if isinstance(prop, dict) and isinstance(prop.get("$ref"), str):
        name = prop["$ref"].rsplit("/", 1)[-1]
        if name not in schemas:
            raise SystemExit(f"missing $ref {name}")
        return schemas[name]
    if not isinstance(prop, dict):
        raise SystemExit(f"property is not an object: {prop!r}")
    return prop


def nullable(schemas: dict, prop: dict) -> bool:
    for node in (prop, resolve(schemas, prop)):
        kind = node.get("type")
        if isinstance(kind, list) and "null" in kind:
            return True
        if node.get("nullable") is True:
            return True
        for key in ("anyOf", "oneOf"):
            items = node.get(key)
            if isinstance(items, list) and any(
                isinstance(item, dict) and item.get("type") == "null" for item in items
            ):
                return True
    return False


def presence(schemas: dict, name: str, prop: dict, required: set[str]) -> str:
    if name in required and nullable(schemas, prop):
        return "required_nullable"
    if name in required:
        return "required"
    return "optional"


def child_object(schemas: dict, prop: dict) -> dict | None:
    resolved = resolve(schemas, prop)
    if resolved.get("type") == "array" and isinstance(resolved.get("items"), dict):
        item = resolve(schemas, resolved["items"])
        if item.get("properties"):
            return item
        return None
    if resolved.get("properties"):
        return resolved
    return None


def walk(
    schemas: dict,
    owner: str,
    schema: dict,
    path: str,
    rows: list[dict[str, str]],
    skip_discriminant: bool,
) -> None:
    required = set(schema.get("required") or [])
    props = schema.get("properties") or {}
    if not isinstance(props, dict):
        raise SystemExit(f"{path} properties are not an object")
    for name, prop in props.items():
        if skip_discriminant and name in ("type", "cmd"):
            continue
        if not isinstance(prop, dict):
            raise SystemExit(f"{path}.{name} is not a schema object")
        rows.append(
            {
                "owner": owner,
                "field": name,
                "presence": presence(schemas, name, prop, required),
                "schema": path,
            }
        )
        child = child_object(schemas, prop)
        if child is None:
            continue
        child_owner = CHILD_OWNER.get((owner, name))
        if path == "marketMetadataUpdatedPayload" and name == "msg":
            child_owner = "MarketMetadataBody"
        if child_owner is None:
            raise SystemExit(f"no Rust owner for object {owner}.{name} ({path})")
        walk(schemas, child_owner, child, f"{path}.{name}", rows, False)


def derive() -> list[dict[str, str]]:
    spec = yaml.safe_load(SPEC.read_text())
    schemas = spec["components"]["schemas"]
    rows: list[dict[str, str]] = []
    for schema_name, owner in ROOTS:
        if schema_name not in schemas:
            raise SystemExit(f"missing schema {schema_name}")
        walk(schemas, owner, schemas[schema_name], schema_name, rows, True)
    for schema_name, owner in (
        ("subscribedIndicesResponsePayload", "CfBenchmarksIndexListBody"),
        ("subscribedUnderlyingsResponsePayload", "UnderlyingListBody"),
        ("listSubscriptionsResponsePayload", "SubscribedBody"),
    ):
        child = child_object(schemas, schemas[schema_name]["properties"]["msg"])
        walk(schemas, owner, child, schema_name + ".msg", rows, False)
    rows.sort(key=lambda row: (row["owner"], row["field"], row["schema"]))
    return rows


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    rows = derive()
    payload = {"count": len(rows), "fields": rows}
    text = json.dumps(payload, indent=2) + "\n"
    if args.check:
        current = OUT.read_text()
        if current != text:
            raise SystemExit("ws_fields.json does not match spec/asyncapi.yaml")
        print(f"ws fields ok ({len(rows)})")
        return 0
    OUT.write_text(text)
    print(f"wrote {OUT} ({len(rows)})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
