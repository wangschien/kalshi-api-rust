#!/usr/bin/env bash
# Replace crates/kalshi-api-openapi with a fresh generation.
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
cd "$ROOT"
WORK=$(mktemp -d)
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$WORK/target}"
bash "$ROOT/tools/kalshi-sdk/generate.sh" "$WORK/crate" "$WORK/generator.log"
rm -rf "$ROOT/crates/kalshi-api-openapi"
mv "$WORK/crate" "$ROOT/crates/kalshi-api-openapi"
python3 "$ROOT/tools/kalshi-sdk/write_clippy_toml.py"
echo "regenerated crates/kalshi-api-openapi"
