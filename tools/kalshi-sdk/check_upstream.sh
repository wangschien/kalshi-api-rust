#!/usr/bin/env bash
# Compare the official OpenAPI, AsyncAPI, and kalshi_python_async wheel to pins.
# An unchanged run exits 0 and does not start a model.
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
LIB="$ROOT/tools/kalshi-sdk/upstream_lib.py"
PINS="$ROOT/tools/kalshi-sdk/pins.toml"
JOB="$ROOT/tools/kalshi-sdk/BOT_JOB.md"
OUT=${KALSHI_UPSTREAM_CHANGE:-$ROOT/tools/kalshi-sdk/upstream_change.json}
LOCK=${KALSHI_UPSTREAM_LOCK:-/tmp/kalshi-sdk-upstream.lock}

# Children must not inherit the lock fd. flock below is the only command that keeps it.
run() { "$@" 9>&-; }

fetch() {
  run curl --fail --silent --show-error --location --max-time 60 --output "$2" "$1"
}

exec 9>"$LOCK"
if ! flock -n 9; then
  echo busy
  exit 0
fi

WORK=$(run mktemp -d)
cleanup() { run rm -rf "$WORK"; }
trap cleanup EXIT

STATE=${KALSHI_UPSTREAM_STATE:-/tmp/kalshi-sdk-upstream-state.json}
run python3 "$LIB" state-check --state "$STATE"

if [[ -z "${KALSHI_OPENAPI_URL:-}" || -z "${KALSHI_ASYNCAPI_URL:-}" || -z "${KALSHI_PYPI_JSON_URL:-}" ]]; then
  PIN_URLS=$(run python3 "$LIB" sources --pins "$PINS")
  mapfile -t PIN_URLS <<< "$PIN_URLS"
fi
OPENAPI_URL=${KALSHI_OPENAPI_URL:-${PIN_URLS[0]}}
ASYNCAPI_URL=${KALSHI_ASYNCAPI_URL:-${PIN_URLS[1]}}
PYPI_URL=${KALSHI_PYPI_JSON_URL:-${PIN_URLS[2]}}

fetch "$OPENAPI_URL" "$WORK/openapi.yaml"
fetch "$ASYNCAPI_URL" "$WORK/asyncapi.yaml"
fetch "$PYPI_URL" "$WORK/pypi.json"
WHEEL_URL=$(run python3 "$LIB" wheel-url --pypi "$WORK/pypi.json")
fetch "$WHEEL_URL" "$WORK/wheel.whl"

set +e
run python3 "$LIB" check \
  --pins "$PINS" \
  --openapi "$WORK/openapi.yaml" \
  --asyncapi "$WORK/asyncapi.yaml" \
  --pypi "$WORK/pypi.json" \
  --wheel "$WORK/wheel.whl" \
  --out "$OUT"
rc=$?
set -e

if [[ "$rc" -eq 0 ]]; then
  rm -f "$OUT"
  echo unchanged
  exit 0
fi
if [[ "$rc" -ne 10 ]]; then
  echo "upstream check failed" >&2
  exit "$rc"
fi

set +e
action=$(run python3 "$LIB" upstream-state --state "$STATE" --report "$OUT" \
  --now "$(date +%s 9>&-)" --pid "$$" --event begin)
state_rc=$?
set -e
if [[ "$state_rc" -ne 0 ]]; then
  exit "$state_rc"
fi
case "$action" in
  launch) ;;
  done) echo done; exit 0 ;;
  failed)
    echo "upstream alert: this source failed three times" >&2
    echo failed
    exit 0
    ;;
  quiet) echo failed; exit 0 ;;
  *) echo "upstream state $action" >&2; exit 1 ;;
esac

echo changed
if [[ -n "${KALSHI_BOT_CMD:-}" ]]; then
  export KALSHI_UPSTREAM_CHANGE="$OUT"
  set +e
  # The lock fd stays with this process. A child must not keep it after we exit.
  "$KALSHI_BOT_CMD" "$JOB" 9>&-
  brc=$?
  set -e
  if [[ "$brc" -eq 0 ]]; then
    run python3 "$LIB" upstream-state --state "$STATE" --report "$OUT" \
      --now "$(date +%s 9>&-)" --pid "$$" --event ok >/dev/null
  else
    run python3 "$LIB" upstream-state --state "$STATE" --report "$OUT" \
      --now "$(date +%s 9>&-)" --pid "$$" --event fail >/dev/null
  fi
  exit "$brc"
fi
exit 10
