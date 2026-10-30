#!/usr/bin/env bash
set -euo pipefail
ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
STAGE=$(mktemp -d)
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT
python3 -I "$ROOT/tools/kalshi-sdk/overlay.py" "$ROOT/spec/openapi.yaml" "$STAGE/effective.yaml"
python3 -I "$ROOT/tools/kalshi-sdk/check_string_enums.py" "$STAGE/effective.yaml"
