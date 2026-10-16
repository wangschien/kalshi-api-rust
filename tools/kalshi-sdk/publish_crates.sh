#!/usr/bin/env bash
# Publish kalshi-api-openapi, wait until that version is on the index, then kalshi-api.
# A version already on the index is skipped. The token is never printed.
set -euo pipefail

version=${1:?version}
if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
  echo "trusted publishing token is missing" >&2
  exit 1
fi

index=${KALSHI_CRATES_INDEX:-https://index.crates.io}
timeout=${KALSHI_INDEX_TIMEOUT_SEC:-600}
interval=${KALSHI_INDEX_POLL_SEC:-5}
cargo=${CARGO:-cargo}

index_prefix() {
  local name=$1
  local n=${#name}
  if (( n <= 1 )); then
    echo "1/$name"
  elif (( n == 2 )); then
    echo "2/$name"
  elif (( n == 3 )); then
    echo "3/${name:0:1}/$name"
  else
    echo "${name:0:2}/${name:2:2}/$name"
  fi
}

published() {
  local name=$1
  local url="${index%/}/$(index_prefix "$name")"
  local body
  body=$(curl --fail --silent --show-error --location --max-time 20 "$url" 2>/dev/null || true)
  python3 -c 'import json, sys
version = sys.argv[1]
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    try:
        row = json.loads(line)
    except json.JSONDecodeError:
        continue
    if row.get("vers") == version:
        raise SystemExit(0)
raise SystemExit(1)' "$version" <<<"$body"
}

wait_visible() {
  local name=$1
  local start=$SECONDS
  while (( SECONDS - start < timeout )); do
    if published "$name"; then
      return 0
    fi
    sleep "$interval"
  done
  echo "crates.io index did not show ${name} ${version} within ${timeout}s" >&2
  return 1
}

publish_one() {
  local name=$1
  if published "$name"; then
    echo "skip ${name} ${version}"
    return 0
  fi
  "$cargo" publish -p "$name" --locked --dry-run
  "$cargo" publish -p "$name" --locked
}

publish_one kalshi-api-openapi
wait_visible kalshi-api-openapi
publish_one kalshi-api
