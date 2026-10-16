#!/usr/bin/env bash
# Review cases for the pre-release R1 ruling. No network and no cargo slot.
set -euo pipefail
ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
LIB="$ROOT/tools/kalshi-sdk/upstream_lib.py"
BASE=$(mktemp -d)
trap 'rm -rf "$BASE"' EXIT
fail() { echo "FAIL $*" >&2; exit 1; }

test_atomic_write_keeps_the_previous_state() {
  local dir=$BASE/atomic
  mkdir -p "$dir"
  python3 - "$LIB" "$dir/state.json" << 'PY'
import os, sys
sys.path.insert(0, os.path.dirname(sys.argv[1]))
import upstream_lib
path = __import__("pathlib").Path(sys.argv[2])
upstream_lib._save_state(path, {"keys": {"a": {"status": "done", "failures": 0, "alerted": 0}}})
temporary = path.with_name(f".{path.name}.999.tmp")
temporary.write_text("{")
if upstream_lib._load_state(path)["keys"]["a"]["status"] != "done":
    raise SystemExit("partial temp replaced the good state")
PY
}

test_corrupt_json_is_kept_and_stops() {
  local dir=$BASE/corrupt
  mkdir -p "$dir"
  printf '%s\n' '{' > "$dir/state.json"
  local report=$dir/report.json
  python3 - "$report" << 'PY'
import json, sys
json.dump({"sources": {
    "openapi": {"new_sha256": "a"},
    "asyncapi": {"new_sha256": "b"},
    "python_async": {"new_version": "1"},
}}, open(sys.argv[1], "w"))
PY
  local rc=0
  python3 "$LIB" upstream-state --state "$dir/state.json" --report "$report" --now 100 --pid 1 --event begin >"$dir/out" 2>"$dir/err" || rc=$?
  cat > "$dir/bot.sh" << EOF
#!/bin/bash
echo launched > "$dir/launched"
EOF
  chmod +x "$dir/bot.sh"
  local before i
  before=$(cat "$dir/state.json")
  for i in 1 2 3; do
    set +e
    KALSHI_UPSTREAM_STATE="$dir/state.json" KALSHI_UPSTREAM_LOCK="$dir/lock-$i" \
      KALSHI_BOT_CMD="$dir/bot.sh" \
      bash "$ROOT/tools/kalshi-sdk/check_upstream.sh" >"$dir/out" 2>"$dir/err"
    rc=$?
    set -e
    [[ "$rc" -ne 0 ]] || fail "corrupt round $i exited 0"
    grep -q 'left in place' "$dir/err" || fail "round $i alert: $(cat "$dir/err")"
    grep -q "$dir/state.json" "$dir/err" || fail "round $i omitted the path"
    [[ ! -e $dir/launched ]] || fail "round $i started the bot"
    [[ "$(cat "$dir/state.json")" == "$before" ]] || fail "round $i changed the bytes"
  done
}

test_publish_order() {
  local mode=$1
  local dir=$BASE/publish-$mode
  mkdir -p "$dir/index/ka/ls"
  : > "$dir/index/ka/ls/kalshi-api-openapi"
  : > "$dir/index/ka/ls/kalshi-api"
  if [[ "$mode" == openapi-only ]]; then
    echo '{"vers":"3.32.0"}' > "$dir/index/ka/ls/kalshi-api-openapi"
  fi
  cat > "$dir/cargo" << EOF
#!/usr/bin/env bash
printf '%s\n' "\$*" >> "$dir/log"
if [[ "\$*" == *--dry-run* ]]; then exit 0; fi
name=""
prev=""
for arg in "\$@"; do
  if [[ "\$prev" == -p ]]; then name=\$arg; fi
  prev=\$arg
done
printf '%s\n' '{"vers":"3.32.0"}' > "$dir/index/ka/ls/\$name"
EOF
  chmod +x "$dir/cargo"
  CARGO="$dir/cargo" CARGO_REGISTRY_TOKEN=secret-token \
    KALSHI_CRATES_INDEX="file://$dir/index" KALSHI_INDEX_POLL_SEC=0 KALSHI_INDEX_TIMEOUT_SEC=5 \
    bash "$ROOT/tools/kalshi-sdk/publish_crates.sh" 3.32.0 >"$dir/out"
  grep -q secret-token "$dir/out" && fail "token printed in $mode"
  case "$mode" in
    empty)
      grep -q 'publish -p kalshi-api-openapi --locked --dry-run' "$dir/log"
      grep -q 'publish -p kalshi-api-openapi --locked$' "$dir/log" || grep -q 'publish -p kalshi-api-openapi --locked' "$dir/log"
      local openapi_at api_at
      openapi_at=$(grep -n 'kalshi-api-openapi --locked$' "$dir/log" | head -1 | cut -d: -f1)
      api_at=$(grep -n 'kalshi-api --locked --dry-run' "$dir/log" | head -1 | cut -d: -f1)
      [[ -n "$openapi_at" && -n "$api_at" && "$openapi_at" -lt "$api_at" ]] || fail "order $mode: $(cat "$dir/log")"
      ;;
    openapi-only)
      grep -q 'skip kalshi-api-openapi 3.32.0' "$dir/out" || fail "openapi was not skipped"
      grep -q 'publish -p kalshi-api --locked --dry-run' "$dir/log" || fail "api was not published"
      if grep -q 'kalshi-api-openapi' "$dir/log"; then
        fail "openapi was published again"
      fi
      ;;
    both)
      grep -q 'publish -p kalshi-api-openapi --locked --dry-run' "$dir/log"
      grep -q 'publish -p kalshi-api --locked --dry-run' "$dir/log"
      ;;
  esac
}

test_workflow_has_no_registry_secret() {
  local text
  text=$(cat "$ROOT/.github/workflows/release.yml" "$ROOT/.github/workflows/ci.yml")
  grep -q 'CRATES_IO_TOKEN' <<<"$text" && fail "registry secret remains"
  grep -q 'crates-io-auth-action@c6f97d42243bad5fab37ca0427f495c86d5b1a18' "$ROOT/.github/workflows/release.yml" || fail "auth action is not pinned"
  grep -q 'actions/checkout@11d5960a326750d5838078e36cf38b85af677262' "$ROOT/.github/workflows/release.yml" || fail "checkout is not pinned"
}

test_done_is_not_launched_again() {
  local dir=$BASE/done
  mkdir -p "$dir"
  python3 - "$dir/report.json" << 'PY'
import json, sys
json.dump({"sources": {"openapi": {"new_sha256": "a"}, "asyncapi": {"new_sha256": "b"}, "python_async": {"new_version": "1"}}}, open(sys.argv[1], "w"))
PY
  python3 "$LIB" upstream-state --state "$dir/state.json" --report "$dir/report.json" --now 100 --pid 1 --event ok >/dev/null
  local out
  out=$(python3 "$LIB" upstream-state --state "$dir/state.json" --report "$dir/report.json" --now 200 --pid 1 --event begin)
  [[ "$out" == done ]] || fail "done key launched again: $out"
}

test_parent_death_releases_the_lock() {
  local dir=$BASE/lock
  mkdir -p "$dir/tools/kalshi-sdk" "$dir/upstream"
  cp "$ROOT/tools/kalshi-sdk/check_upstream.sh" "$ROOT/tools/kalshi-sdk/upstream_lib.py" "$ROOT/tools/kalshi-sdk/BOT_JOB.md" "$dir/tools/kalshi-sdk/"
  printf 'info:\n  title: demo\n  version: 9.9.9\n' > "$dir/upstream/openapi.yaml"
  printf 'info:\n  title: demo\n  version: 2.0.0\n' > "$dir/upstream/asyncapi.yaml"
  printf 'not-a-wheel' > "$dir/upstream/async.whl"
  python3 - "$dir" << 'PY'
import hashlib, json, pathlib, sys
root = pathlib.Path(sys.argv[1])
wheel = root / "upstream/async.whl"
digest = hashlib.sha256(wheel.read_bytes()).hexdigest()
(root / "upstream/pypi.json").write_text(json.dumps({
    "info": {"version": "3.30.0"},
    "urls": [{"packagetype": "bdist_wheel", "filename": "kalshi_python_async-3.30.0-py3-none-any.whl",
              "url": "file://" + str(wheel), "digests": {"sha256": digest}}],
}))
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
(root / "tools/kalshi-sdk/pins.toml").write_text(f'''[openapi]
info_version = "3.31.0"
sha256 = "{sha(root / "upstream/openapi.yaml")}"
path = "spec/openapi.yaml"
source = "file://{root / "upstream/openapi.yaml"}"

[asyncapi]
info_version = "2.0.0"
sha256 = "different"
path = "spec/asyncapi.yaml"
source = "file://{root / "upstream/asyncapi.yaml"}"

[python_sdk]
version = "3.30.0"
wheel_sha256 = "{digest}"
source = "file://{root / "upstream/pypi.json"}"
''')
PY
  cat > "$dir/bot.sh" << EOF
#!/usr/bin/env bash
echo \$\$ > "$dir/bot.pid"
if ls -l /proc/\$\$/fd 2>/dev/null | grep -q "$dir/upstream.lock"; then
  echo held > "$dir/bot.lock"
else
  echo free > "$dir/bot.lock"
fi
sleep 30
EOF
  chmod +x "$dir/bot.sh"
  (
    cd "$dir"
    export KALSHI_OPENAPI_URL="file://$dir/upstream/openapi.yaml"
    export KALSHI_ASYNCAPI_URL="file://$dir/upstream/asyncapi.yaml"
    export KALSHI_PYPI_JSON_URL="file://$dir/upstream/pypi.json"
    export KALSHI_UPSTREAM_LOCK="$dir/upstream.lock"
    export KALSHI_UPSTREAM_STATE="$dir/state.json"
    export KALSHI_BOT_CMD="$dir/bot.sh"
    bash tools/kalshi-sdk/check_upstream.sh
  ) >"$dir/out" &
  local parent=$!
  for _ in 1 2 3 4 5 6 7 8 9 10; do
    [[ -s $dir/bot.pid ]] && break
    sleep 0.2
  done
  [[ -s $dir/bot.pid ]] || fail "bot did not start: $(cat "$dir/out" 2>/dev/null)"
  kill "$parent" 2>/dev/null || true
  wait "$parent" 2>/dev/null || true
  [[ "$(cat "$dir/bot.lock")" == free ]] || fail "child kept the lock"
  local second
  second=$(
    cd "$dir"
    export KALSHI_OPENAPI_URL="file://$dir/upstream/openapi.yaml"
    export KALSHI_ASYNCAPI_URL="file://$dir/upstream/asyncapi.yaml"
    export KALSHI_PYPI_JSON_URL="file://$dir/upstream/pypi.json"
    export KALSHI_UPSTREAM_LOCK="$dir/upstream.lock"
    export KALSHI_UPSTREAM_STATE="$dir/state.json"
    export KALSHI_BOT_CMD=/bin/true
    bash tools/kalshi-sdk/check_upstream.sh
  )
  [[ "$second" != busy ]] || fail "orphan still blocks the lock: $second"
  kill "$(cat "$dir/bot.pid")" 2>/dev/null || true
}

test_curl_does_not_inherit_the_lock() {
  local dir=$BASE/curl-lock
  mkdir -p "$dir/bin"
  cat > "$dir/bin/curl" << EOF
#!/bin/bash
{
  echo \$\$
  for fd in /proc/\$\$/fd/*; do
    readlink "\$fd" || true
  done
} > "$dir/fds"
sleep 30
EOF
  chmod +x "$dir/bin/curl"
  PATH="$dir/bin:$PATH" KALSHI_UPSTREAM_LOCK="$dir/upstream.lock" \
    KALSHI_UPSTREAM_STATE="$dir/state.json" \
    bash "$ROOT/tools/kalshi-sdk/check_upstream.sh" >"$dir/out" 2>"$dir/err" &
  local parent=$!
  local _
  for _ in 1 2 3 4 5 6 7 8 9 10; do
    [[ -s $dir/fds ]] && break
    sleep 0.2
  done
  [[ -s $dir/fds ]] || fail "curl did not start: $(cat "$dir/err" 2>/dev/null)"
  if grep -F "$dir/upstream.lock" "$dir/fds"; then
    fail "curl inherited the lock fd"
  fi
  kill "$parent" 2>/dev/null || true
  wait "$parent" 2>/dev/null || true
}

test_readme_api_version_comes_from_the_spec() {
  local dir=$BASE/readme
  mkdir -p "$dir"
  rsync -a --exclude .git --exclude target "$ROOT/" "$dir/tree/"
  python3 - "$LIB" "$dir/tree" << 'PY'
import re, sys
from datetime import datetime, timezone
from pathlib import Path
sys.path.insert(0, str(Path(sys.argv[1]).parent))
import upstream_lib
root = Path(sys.argv[2])
now = datetime(2026, 9, 28, tzinfo=timezone.utc)
upstream_lib.apply_release(root, "4.0.0", now)
spec = root / "spec" / "openapi.yaml"
lines = spec.read_text().splitlines()
out = []
in_info = False
done = False
for line in lines:
    if line.startswith("info:"):
        in_info = True
        out.append(line)
        continue
    if in_info and line and not line.startswith((" ", "\t")):
        in_info = False
    if in_info and not done and line.strip().startswith("version:"):
        out.append(re.sub(r"version:\s*\S+", "version: 3.32.0", line))
        done = True
        continue
    out.append(line)
if not done:
    raise SystemExit("spec info.version was not rewritten")
spec.write_text("\n".join(out) + "\n")
pins_path = root / "tools" / "kalshi-sdk" / "pins.toml"
pins_text = pins_path.read_text()
pins_text = re.sub(
    r"(?m)^(\[openapi\]\n(?:.*\n)*?info_version = )\"[^\"]+\"",
    r'\g<1>"3.32.0"',
    pins_text,
    count=1,
)
digest = __import__("hashlib").sha256(spec.read_bytes()).hexdigest()
pins_text = re.sub(
    r"(?m)^(\[openapi\]\n(?:.*\n)*?sha256 = )\"[^\"]+\"",
    rf'\g<1>"{digest}"',
    pins_text,
    count=1,
)
pins_path.write_text(pins_text)
upstream_lib.apply_release(root, "4.1.0", now)
readme = (root / "README.md").read_text().splitlines()
api = [line for line in readme if line.startswith("- API version:")]
crate = [line for line in readme if line.startswith("- Crate version:")]
if api != ["- API version: 3.32.0"] or crate != ["- Crate version: 4.1.0"]:
    raise SystemExit(f"readme versions api={api} crate={crate}")
PY
}

test_atomic_write_keeps_the_previous_state
test_corrupt_json_is_kept_and_stops
test_curl_does_not_inherit_the_lock
test_readme_api_version_comes_from_the_spec
test_publish_order empty
test_publish_order openapi-only
test_publish_order both
test_workflow_has_no_registry_secret
test_done_is_not_launched_again
test_parent_death_releases_the_lock
echo "prerelease r1 tests ok"
