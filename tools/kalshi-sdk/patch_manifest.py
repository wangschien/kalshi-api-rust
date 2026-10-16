#!/usr/bin/env python3
"""Set the generated crate manifest fields after openapi-generator runs."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPLACEMENTS = {
    "license": 'license = "MIT"',
    "authors": 'authors = ["wangschien <schienwang@gmail.com>"]',
    "description": 'description = "Rust models and client generated from the official Kalshi OpenAPI"',
    "repository": 'repository = "https://github.com/wangschien/kalshi-api-rust"',
    "homepage": 'homepage = "https://github.com/wangschien/kalshi-api-rust"',
    "readme": 'readme = "README.md"',
}


def patch(text: str) -> str:
    lines = text.splitlines()
    seen = set()
    out = []
    in_package = False
    for line in lines:
        if line.startswith("["):
            if in_package:
                for key, rendered in REPLACEMENTS.items():
                    if key not in seen:
                        out.append(rendered)
                        seen.add(key)
                if out and out[-1] != "":
                    out.append("")
            in_package = line.strip() == "[package]"
        if in_package:
            key = line.split("=", 1)[0].strip()
            if key in REPLACEMENTS:
                out.append(REPLACEMENTS[key])
                seen.add(key)
                continue
        out.append(line)
    if in_package:
        for key, rendered in REPLACEMENTS.items():
            if key not in seen:
                out.append(rendered)
    rendered = "\n".join(out)
    if text.endswith("\n"):
        rendered += "\n"
    return rendered


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("cargo_toml", type=Path)
    args = parser.parse_args(argv)
    original = args.cargo_toml.read_text()
    updated = patch(original)
    if updated != original:
        args.cargo_toml.write_text(updated)
    if patch(updated) != updated:
        raise SystemExit("manifest patch is not idempotent")
    print("manifest patch ok")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
