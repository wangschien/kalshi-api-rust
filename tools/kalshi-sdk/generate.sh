#!/usr/bin/env bash
# Generate crates/kalshi-api-openapi into OUTPUT_DIR and check the warning log.
# Requires JAVA (or java on PATH) and KALSHI_GENERATOR_JAR.
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: generate.sh OUTPUT_DIR LOG_PATH" >&2
  exit 2
fi

OUT=$1
LOG=$2
ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
cd "$ROOT"

if [[ -z "${KALSHI_GENERATOR_JAR:-}" ]]; then
  echo "set KALSHI_GENERATOR_JAR to the pinned generator jar" >&2
  exit 2
fi
JAVA=${JAVA:-java}

STAGE=$(mktemp -d)
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT

python3 -I "$ROOT/tools/kalshi-sdk/check_pins.py" --root "$ROOT" --jar "$KALSHI_GENERATOR_JAR"
python3 -I "$ROOT/tools/kalshi-sdk/overlay.py" "$ROOT/spec/openapi.yaml" "$STAGE/effective.yaml"
python3 -I "$ROOT/tools/kalshi-sdk/check_pins.py" --root "$ROOT" --effective "$STAGE/effective.yaml"

rm -rf "$OUT"
mkdir -p "$(dirname "$OUT")"
"$JAVA" -jar "$KALSHI_GENERATOR_JAR" generate \
  -i "$STAGE/effective.yaml" \
  -g rust \
  -o "$OUT" \
  --package-name kalshi-api-openapi \
  --additional-properties=library=reqwest,supportAsync=true,packageVersion=3.34.0,preferUnsignedInt=false,bestFitInt=false,useSingleRequestParameter=true,supportMiddleware=true,avoidBoxedModels=true \
  >"$LOG" 2>&1

python3 -I "$ROOT/tools/kalshi-sdk/check_warnings.py" "$LOG"
python3 -I "$ROOT/tools/kalshi-sdk/clippy_allows.py" "$OUT/src/lib.rs"
python3 -I "$ROOT/tools/kalshi-sdk/patch_manifest.py" "$OUT/Cargo.toml"
python3 -I "$ROOT/tools/kalshi-sdk/patch_requests.py" "$ROOT/spec/openapi.yaml" "$OUT"
python3 -I "$ROOT/tools/kalshi-sdk/patch_deadlines.py" "$OUT"
python3 -I "$ROOT/tools/kalshi-sdk/patch_numbers.py" "$OUT" --defaults "$ROOT/tools/kalshi-sdk/python_model_defaults.json"
python3 -I "$ROOT/tools/kalshi-sdk/patch_responses.py" "$OUT"
python3 -I "$ROOT/tools/kalshi-sdk/trim_generated.py" "$OUT"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$STAGE/target}"
"${CARGO:-cargo}" fmt --manifest-path "$OUT/Cargo.toml" --all
rm -f "$OUT/Cargo.lock"
