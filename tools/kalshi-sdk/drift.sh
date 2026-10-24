#!/usr/bin/env bash
# Regenerate the client into a temporary directory and fail if it differs.
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
cd "$ROOT"
WORK=$(mktemp -d)
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$WORK/target}"
bash "$ROOT/tools/kalshi-sdk/generate.sh" "$WORK/crate" "$WORK/generator.log"

python3 "$ROOT/tools/kalshi-sdk/fixed_point_fields.py" --check
python3 "$ROOT/tools/kalshi-sdk/ws_fields.py" --check
python3 "$ROOT/tools/kalshi-sdk/ws_codegen.py" --check
python3 "$ROOT/tools/kalshi-sdk/subaccount_semantics.py" --check
if ! diff -ru "$ROOT/crates/kalshi-api-openapi" "$WORK/crate"; then
  echo "drift: regenerated client differs from crates/kalshi-api-openapi" >&2
  exit 1
fi
echo "drift ok"
