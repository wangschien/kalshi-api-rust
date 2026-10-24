#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
cd "$ROOT"
: "${KALSHI_PYTHON_SDK:?set KALSHI_PYTHON_SDK to the unpacked pinned async package}"
CARGO_TOOL=${CARGO:-cargo}
LOGS=${1:-verification-logs}
mkdir -p "$LOGS"

run() {
    local name=$1
    shift
    set +e
    "$@" >"$LOGS/$name.log" 2>&1
    local rc=$?
    set -e
    printf '%s\n' "$rc" >"$LOGS/$name.rc"
    if [[ "$rc" -ne 0 ]]; then
        tail -n 60 "$LOGS/$name.log"
        return "$rc"
    fi
    echo "$name rc=$rc"
}

run pins python3 -I tools/kalshi-sdk/check_pins.py --root "$ROOT"
run conformance python3 -I tools/kalshi-sdk/conformance.py \
    --python "$KALSHI_PYTHON_SDK" \
    --rust crates/kalshi-api-openapi/src/models \
    --rust-api crates/kalshi-api-openapi/src/apis \
    --expected tools/kalshi-sdk/expected_py334.json \
    --api-expected tools/kalshi-sdk/expected_api_async.json
run parity python3 -I tools/kalshi-sdk/parity.py \
    --python "$KALSHI_PYTHON_SDK" --root "$ROOT" \
    --openapi spec/openapi.yaml --asyncapi spec/asyncapi.yaml \
    --output "$LOGS/parity" --check
run fixed_point python3 -I tools/kalshi-sdk/fixed_point_fields.py --check
run ws_fields python3 -I tools/kalshi-sdk/ws_fields.py --check
run ws_codegen python3 -I tools/kalshi-sdk/ws_codegen.py --check
run subaccounts python3 -I tools/kalshi-sdk/subaccount_semantics.py --check
run lib timeout 1800 "$CARGO_TOOL" test --workspace --lib --locked -j 2
run integration timeout 1800 "$CARGO_TOOL" test --workspace --test '*' --no-fail-fast --locked -j 2
run doc timeout 1800 "$CARGO_TOOL" test --workspace --doc --locked -j 2
run clippy timeout 1800 "$CARGO_TOOL" clippy --workspace --all-targets --locked -j 2 -- -D warnings
run formatting "$CARGO_TOOL" fmt --all --check
run public_content bash tools/kalshi-sdk/public_content_grep.sh
