#!/usr/bin/env python3
"""Validate synthetic WebSocket fixtures against AsyncAPI component schemas.

Requires jsonschema 4.10.3 and PyYAML 6.0.1.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import yaml
from jsonschema import Draft7Validator, FormatChecker, RefResolver

ROOT = Path(__file__).resolve().parents[2]
SPEC = ROOT / "spec" / "asyncapi.yaml"
MANIFEST = ROOT / "crates" / "kalshi-api" / "tests" / "fixtures" / "s3" / "manifest.json"
FIXTURES = MANIFEST.parent


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.parse_args(argv)
    spec = yaml.safe_load(SPEC.read_text())
    schemas = spec["components"]["schemas"]
    resolver = RefResolver.from_schema(spec)
    checker = FormatChecker()
    rows = json.loads(MANIFEST.read_text())
    if not rows:
        raise SystemExit("ws schema manifest is empty")
    for row in rows:
        name = row["schema"]
        if name not in schemas:
            raise SystemExit(f"unknown schema {name}")
        instance = json.loads((FIXTURES / row["file"]).read_text())
        validator = Draft7Validator(schemas[name], resolver=resolver, format_checker=checker)
        errors = sorted(validator.iter_errors(instance), key=lambda err: list(err.path))
        if errors:
            detail = "; ".join(err.message for err in errors)
            raise SystemExit(f"{row['file']} is not a {name}: {detail}")
    print(f"ws schemas ok ({len(rows)})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
