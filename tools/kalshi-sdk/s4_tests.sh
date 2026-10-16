#!/usr/bin/env bash
# Fake-upstream tests for the hash check and the release entry.
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
LIB="$ROOT/tools/kalshi-sdk/upstream_lib.py"
BASE=$(mktemp -d)
TG_PID=
FLOCK_PID=
TG_PORT=
KALSHI_RELEASE_LOG=
CARGO_LOG=

cleanup() {
  [[ -n "${TG_PID:-}" ]] && kill "$TG_PID" 2>/dev/null || true
  [[ -n "${FLOCK_PID:-}" ]] && kill "$FLOCK_PID" 2>/dev/null || true
  rm -rf "$BASE"
}
trap cleanup EXIT

fail() {
  printf 'FAIL %s\n' "$*" >&2
  if [[ -n "${KALSHI_RELEASE_LOG:-}" && -f ${KALSHI_RELEASE_LOG} ]]; then
    printf 'release log:\n' >&2
    cat "$KALSHI_RELEASE_LOG" >&2 || true
  fi
  if [[ -n "${CARGO_LOG:-}" && -f ${CARGO_LOG} ]]; then
    printf 'cargo log:\n' >&2
    cat "$CARGO_LOG" >&2 || true
  fi
  exit 1
}

assert_eq() {
  [[ "$1" == "$2" ]] || fail "$3: got [$1] want [$2]"
}

start_telegram() {
  local script=$BASE/telegram.py
  local port_file=$BASE/telegram.port
  cat > "$script" <<'PY'
import os
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

log_path = os.environ["TG_LOG"]
port_path = os.environ["TG_PORT_FILE"]

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        size = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(size)
        with open(log_path, "ab") as handle:
            handle.write(body + b"\n")
        self.send_response(200)
        self.end_headers()

    def log_message(self, fmt, *args):
        return

server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
with open(port_path, "w", encoding="utf-8") as handle:
    handle.write(str(server.server_address[1]))
server.serve_forever()
PY
  TG_LOG=$BASE/telegram.log TG_PORT_FILE=$port_file python3 "$script" &
  TG_PID=$!
  for _ in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20; do
    if [[ -s $port_file ]]; then
      TG_PORT=$(cat "$port_file")
      return 0
    fi
    sleep 0.05
  done
  fail "telegram sink did not start"
}

telegram_has() {
  python3 -c 'import pathlib,sys,urllib.parse; text=urllib.parse.unquote(pathlib.Path(sys.argv[1]).read_text());
raise SystemExit(0 if sys.argv[2] in text else 1)' "$BASE/telegram.log" "$1" || fail "telegram missed $1"
}

write_wheel() {
  local path=$1 name=$2
  python3 - "$path" "$name" <<'PY'
import sys
import zipfile
path, name = sys.argv[1], sys.argv[2]
with zipfile.ZipFile(path, "w") as archive:
    archive.writestr(f"{name}/__init__.py", "")
PY
}

sha256_file() {
  python3 -c 'import hashlib,sys; print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest())' "$1"
}

decide_is() {
  local current=$1 kind=$2 changed=$3 version=$4 publish=$5
  local got
  got=$(python3 "$LIB" decide --current "$current" --kind "$kind" --changed "$changed")
  python3 -c 'import json,sys; got=json.loads(sys.argv[1]); assert got["version"]==sys.argv[2]; assert got["publish"]==(sys.argv[3]=="yes")' \
    "$got" "$version" "$publish" || fail "decide $current $kind $changed -> $got"
}

unit_decide() {
  decide_is 4.0.0 patch yes 4.0.1 yes
  decide_is 4.0.0 minor yes 4.1.0 yes
  decide_is 4.0.0 major yes 5.0.0 yes
  decide_is 4.0.0 patch no 4.0.0 no
  decide_is 3.31.0 major yes 4.0.0 yes
  decide_is 0.3.1 patch yes 0.3.2 yes
  decide_is 0.3.1 minor yes 0.3.2 yes
  decide_is 0.3.1 major yes 0.4.0 yes
  decide_is 3.31.0 initial yes 3.31.0 yes
  if python3 "$LIB" decide --current 1.2 --kind patch --changed yes >/dev/null 2>&1; then
    fail "short version was accepted"
  fi
  local badge
  badge=$(python3 "$LIB" badge --spec 3.32.0 --now 2026-09-28T15:04:00Z)
  [[ "$badge" == *3.32.0* && "$badge" == *2026--09--28* && "$badge" == *15%3A04* ]] || fail "badge: $badge"
}

unit_apply_real_tree() {
  local dest=$BASE/apply-real
  mkdir -p "$dest/spec" "$dest/crates/kalshi-api" "$dest/crates/kalshi-api-openapi" "$dest/crates/kalshi-api-pins" "$dest/tools/kalshi-sdk"
  cp "$ROOT/spec/openapi.yaml" "$ROOT/spec/asyncapi.yaml" "$dest/spec/"
  cp "$ROOT/crates/kalshi-api/Cargo.toml" "$dest/crates/kalshi-api/"
  cp "$ROOT/crates/kalshi-api-openapi/Cargo.toml" "$ROOT/crates/kalshi-api-openapi/README.md" "$dest/crates/kalshi-api-openapi/"
  cp "$ROOT/crates/kalshi-api-pins/Cargo.toml" "$dest/crates/kalshi-api-pins/"
  cp "$ROOT/Cargo.lock" "$ROOT/CHANGELOG.md" "$ROOT/README.md" "$dest/"
  cp "$ROOT/tools/kalshi-sdk/pins.toml" "$ROOT/tools/kalshi-sdk/generate.sh" "$dest/tools/kalshi-sdk/"
  cp "$LIB" "$dest/tools/kalshi-sdk/upstream_lib.py"
  python3 "$dest/tools/kalshi-sdk/upstream_lib.py" apply --root "$dest" --version 3.32.0 --now 2026-09-28T15:04:00Z >/dev/null
  assert_eq "$(python3 "$LIB" package-version --manifest "$dest/crates/kalshi-api/Cargo.toml")" "3.32.0" "applied api version"
  assert_eq "$(python3 "$LIB" package-version --manifest "$dest/crates/kalshi-api-openapi/Cargo.toml")" "3.32.0" "applied openapi version"
  assert_eq "$(python3 "$LIB" aligned --root "$dest")" "3.32.0" "applied alignment"
  grep -q 'packageVersion=3.32.0' "$dest/tools/kalshi-sdk/generate.sh" || fail "generator version was not updated"
  grep -q 'packageVersion=3.31.0' "$dest/tools/kalshi-sdk/generate.sh" && fail "old generator version remains"
  grep -qx -- '- Package version: 3.32.0' "$dest/crates/kalshi-api-openapi/README.md" || fail "generated README version"
  grep -q 'package_version = "3.32.0"' "$dest/tools/kalshi-sdk/pins.toml" || fail "pin package version"
  grep -q 'source = "https://docs.kalshi.com/openapi.yaml"' "$dest/tools/kalshi-sdk/pins.toml" || fail "source survived apply"
  python3 - "$dest/Cargo.lock" <<'PY' || fail "lock api version"
import pathlib, sys
text = pathlib.Path(sys.argv[1]).read_text().split("[[package]]")
found = set()
for part in text[1:]:
    lines = part.splitlines()
    name = next(line.split("=", 1)[1].strip().strip('"') for line in lines if line.startswith("name = "))
    if name in {"kalshi-api", "kalshi-api-openapi", "kalshi-api-pins"}:
        version = next(line for line in lines if line.startswith("version = "))
        if version != 'version = "3.32.0"':
            raise SystemExit(name)
        found.add(name)
if found != {"kalshi-api", "kalshi-api-openapi", "kalshi-api-pins"}:
    raise SystemExit("missing")
PY
  grep -q '## \[3.32.0\]' "$dest/CHANGELOG.md" || fail "changelog heading"
  grep -q 'OpenAPI 3.31.0 sha256' "$dest/CHANGELOG.md" || fail "changelog spec pin"
  grep -q 'Pending official Python SDK conformance' "$dest/CHANGELOG.md" || fail "pending note"
  grep -q '## \[Unreleased\]' "$dest/CHANGELOG.md" || fail "unreleased section"
  grep -qx -- '- Crate version: 3.32.0' "$dest/README.md" || fail "crate version line"
  grep -q 'last synced' "$dest/README.md" && fail "badge returned"
  python3 "$dest/tools/kalshi-sdk/upstream_lib.py" apply --root "$dest" --version 3.32.0 --now 2026-09-28T15:04:00Z >/dev/null
  local count
  count=$(grep -c '## \[3.32.0\]' "$dest/CHANGELOG.md")
  assert_eq "$count" "1" "changelog section repeated"
}

unit_zip_slip() {
  local wheel=$BASE/slip.whl dest=$BASE/slip-out
  python3 - "$wheel" <<'PY'
import sys, zipfile
with zipfile.ZipFile(sys.argv[1], "w") as archive:
    archive.writestr("../escape.txt", "nope")
PY
  if python3 "$LIB" unpack --wheel "$wheel" --dest "$dest" --name kalshi_python_async >/dev/null 2>&1; then
    fail "zip slip was extracted"
  fi
}

prepare_upstream_files() {
  local dir=$1 version=$2
  mkdir -p "$dir"
  printf 'info:\n  title: demo\n  version: %s\n' "$version" > "$dir/openapi.yaml"
  printf 'info:\n  title: demo\n  version: 2.0.0\n' > "$dir/asyncapi.yaml"
  write_wheel "$dir/async.whl" kalshi_python_async
  local wheel_sha
  wheel_sha=$(sha256_file "$dir/async.whl")
  python3 - "$dir/pypi.json" "$dir/async.whl" "$wheel_sha" <<'PY'
import json, sys
path, wheel, digest = sys.argv[1:]
json.dump({
    "info": {"version": "3.30.0"},
    "urls": [{
        "packagetype": "bdist_wheel",
        "filename": "kalshi_python_async-3.30.0-py3-none-any.whl",
        "url": "file://" + wheel,
        "digests": {"sha256": digest},
    }],
}, open(path, "w"), indent=2)
PY
}

write_check_pins() {
  local pins=$1 openapi=$2 asyncapi=$3 wheel=$4 info_version=$5
  python3 - "$pins" "$openapi" "$asyncapi" "$wheel" "$info_version" <<'PY'
import hashlib, pathlib, sys
pins, openapi, asyncapi, wheel, info_version = sys.argv[1:]
def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
openapi_path = pathlib.Path(openapi)
async_path = pathlib.Path(asyncapi)
wheel_path = pathlib.Path(wheel)
pypi = wheel_path.parent / "pypi.json"
text = f'''[openapi]
info_version = "{info_version}"
sha256 = "{sha(openapi)}"
path = "spec/openapi.yaml"
source = "file://{openapi_path}"

[asyncapi]
info_version = "2.0.0"
sha256 = "{sha(asyncapi)}"
path = "spec/asyncapi.yaml"
source = "file://{async_path}"

[python_sdk]
version = "3.30.0"
wheel_sha256 = "{sha(wheel)}"
source = "file://{pypi}"
'''
pathlib.Path(pins).write_text(text)
PY
}

install_check_tool() {
  local fix=$1
  mkdir -p "$fix/tools/kalshi-sdk"
  cp "$ROOT/tools/kalshi-sdk/check_upstream.sh" "$ROOT/tools/kalshi-sdk/upstream_lib.py" "$ROOT/tools/kalshi-sdk/BOT_JOB.md" "$fix/tools/kalshi-sdk/"
}

run_check() {
  local fix=$1
  (
    cd "$fix"
    export KALSHI_OPENAPI_URL="file://$fix/upstream/openapi.yaml"
    export KALSHI_ASYNCAPI_URL="file://$fix/upstream/asyncapi.yaml"
    export KALSHI_PYPI_JSON_URL="file://$fix/upstream/pypi.json"
    export KALSHI_UPSTREAM_LOCK="$fix/upstream.lock"
    export KALSHI_UPSTREAM_STATE="$fix/upstream-state.json"
    export KALSHI_BOT_CMD="${BOT_CMD:-}"
    bash tools/kalshi-sdk/check_upstream.sh
  )
}

test_pin_sources() {
  local fix=$BASE/pin-sources
  install_check_tool "$fix"
  prepare_upstream_files "$fix/upstream" 3.31.0
  write_check_pins "$fix/tools/kalshi-sdk/pins.toml" \
    "$fix/upstream/openapi.yaml" "$fix/upstream/asyncapi.yaml" "$fix/upstream/async.whl" 3.31.0
  local out
  out=$(
    cd "$fix"
    unset KALSHI_OPENAPI_URL KALSHI_ASYNCAPI_URL KALSHI_PYPI_JSON_URL
    export KALSHI_UPSTREAM_LOCK="$fix/upstream.lock"
    bash tools/kalshi-sdk/check_upstream.sh
  )
  assert_eq "$out" "unchanged" "pin source stdout"
}

test_unchanged() {
  local fix=$BASE/unchanged
  install_check_tool "$fix"
  prepare_upstream_files "$fix/upstream" 3.31.0
  write_check_pins "$fix/tools/kalshi-sdk/pins.toml" \
    "$fix/upstream/openapi.yaml" "$fix/upstream/asyncapi.yaml" "$fix/upstream/async.whl" 3.31.0
  local launch=$fix/launched
  BOT_CMD=$fix/launch.sh
  cat > "$fix/launch.sh" <<EOF
#!/bin/bash
printf '%s\n' "\$1" > "$launch"
EOF
  chmod +x "$fix/launch.sh"
  local out
  out=$(BOT_CMD="$fix/launch.sh" run_check "$fix")
  assert_eq "$out" "unchanged" "unchanged stdout"
  [[ ! -e $launch ]] || fail "unchanged run started the job"
  [[ ! -e $fix/tools/kalshi-sdk/upstream_change.json ]] || fail "unchanged run wrote a change file"
}

test_changed_launches() {
  local fix=$BASE/changed
  install_check_tool "$fix"
  prepare_upstream_files "$fix/upstream" 3.32.0
  write_check_pins "$fix/tools/kalshi-sdk/pins.toml" \
    "$fix/upstream/openapi.yaml" "$fix/upstream/asyncapi.yaml" "$fix/upstream/async.whl" 3.31.0
  # Pin the previous OpenAPI hash while serving 3.32.0.
  python3 - "$fix/tools/kalshi-sdk/pins.toml" <<'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
path.write_text(path.read_text().replace('info_version = "3.32.0"', 'info_version = "3.31.0"', 1))
PY
  local old_sha new_file
  new_file=$fix/upstream/openapi.yaml
  old_sha=$(sha256_file "$new_file")
  printf '\nextra: true\n' >> "$new_file"
  [[ "$(sha256_file "$new_file")" != "$old_sha" ]] || fail "fixture did not change the document"
  # The pin still has the hash from before the extra line only if write_check_pins ran before the append.
  # write_check_pins hashed the 3.32 file, then we rewrote the pin version, then appended bytes.
  local launch=$fix/launched
  cat > "$fix/launch.sh" <<EOF
#!/bin/bash
printf '%s\n' "\$1" > "$launch"
printf '%s\n' "\${KALSHI_UPSTREAM_CHANGE:-}" >> "$launch"
EOF
  chmod +x "$fix/launch.sh"
  local out rc
  set +e
  out=$(BOT_CMD="$fix/launch.sh" run_check "$fix")
  rc=$?
  set -e
  assert_eq "$rc" "0" "changed launcher status"
  [[ "$out" == changed* ]] || fail "changed stdout: $out"
  [[ -s $launch ]] || fail "launcher was not executed"
  grep -q 'BOT_JOB.md' "$launch" || fail "launcher did not receive the job file"
  local report
  report=$(sed -n '2p' "$launch")
  [[ -s $report ]] || fail "change report missing"
  python3 - "$report" <<'PY'
import json, sys
report = json.load(open(sys.argv[1]))
assert report["changed"] is True
assert report["sources"]["openapi"]["changed"] is True
assert report["sources"]["openapi"]["new_version"] == "3.32.0"
assert report["sources"]["openapi"]["old_version"] == "3.31.0"
PY
}

test_version_only() {
  local fix=$BASE/version-only
  install_check_tool "$fix"
  prepare_upstream_files "$fix/upstream" 3.31.0
  write_check_pins "$fix/tools/kalshi-sdk/pins.toml" \
    "$fix/upstream/openapi.yaml" "$fix/upstream/asyncapi.yaml" "$fix/upstream/async.whl" 9.9.9
  local rc
  set +e
  BOT_CMD= run_check "$fix" >"$fix/out" 2>"$fix/err"
  rc=$?
  set -e
  assert_eq "$rc" "10" "version-only status"
  assert_eq "$(cat "$fix/out")" "changed" "version-only stdout"
  python3 - "$fix/tools/kalshi-sdk/upstream_change.json" <<'PY'
import json, sys
report = json.load(open(sys.argv[1]))
row = report["sources"]["openapi"]
assert row["changed"] is True
assert row["old_version"] == "9.9.9"
assert row["new_version"] == "3.31.0"
assert row["old_sha256"] == row["new_sha256"]
PY
}

test_download_failure() {
  local fix=$BASE/download-fail
  install_check_tool "$fix"
  prepare_upstream_files "$fix/upstream" 3.31.0
  write_check_pins "$fix/tools/kalshi-sdk/pins.toml" \
    "$fix/upstream/openapi.yaml" "$fix/upstream/asyncapi.yaml" "$fix/upstream/async.whl" 3.31.0
  local launch=$fix/launched
  cat > "$fix/launch.sh" <<EOF
#!/bin/bash
printf launched > "$launch"
EOF
  chmod +x "$fix/launch.sh"
  local rc
  set +e
  (
    cd "$fix"
    export KALSHI_OPENAPI_URL="http://127.0.0.1:1/missing"
    export KALSHI_ASYNCAPI_URL="file://$fix/upstream/asyncapi.yaml"
    export KALSHI_PYPI_JSON_URL="file://$fix/upstream/pypi.json"
    export KALSHI_UPSTREAM_LOCK="$fix/upstream.lock"
    export KALSHI_BOT_CMD="$fix/launch.sh"
    bash tools/kalshi-sdk/check_upstream.sh
  ) >"$fix/out" 2>"$fix/err"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "download failure exited 0"
  [[ ! -e $launch ]] || fail "download failure started the job"
}

test_busy() {
  local fix=$BASE/busy
  install_check_tool "$fix"
  local lock=$fix/upstream.lock
  flock "$lock" -c 'sleep 30' &
  FLOCK_PID=$!
  for _ in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20; do
    if ! flock -n "$lock" -c true; then
      break
    fi
    sleep 0.05
  done
  local out rc
  set +e
  out=$(
    cd "$fix"
    export KALSHI_OPENAPI_URL="http://127.0.0.1:1/missing"
    export KALSHI_UPSTREAM_LOCK="$lock"
    export KALSHI_BOT_CMD="$fix/missing-launcher"
    bash tools/kalshi-sdk/check_upstream.sh
  )
  rc=$?
  set -e
  kill "$FLOCK_PID" 2>/dev/null || true
  wait "$FLOCK_PID" 2>/dev/null || true
  FLOCK_PID=
  assert_eq "$rc" "0" "busy status"
  assert_eq "$out" "busy" "busy stdout"
}

build_release_fixture() {
  local fix=$1
  local spec=${SPEC_VERSION:-3.32.0}
  local crate=${CRATE_VERSION:-3.31.0}
  local registry=${REGISTRY_VERSION:-$crate}
  local py_spec=${PY_SPEC:-3.30.0}
  mkdir -p "$fix/bin" "$fix/spec" "$fix/upstream" "$fix/crates/kalshi-api" \
    "$fix/crates/kalshi-api-openapi/src/models" "$fix/crates/kalshi-api-openapi/src/apis" \
    "$fix/crates/kalshi-api-pins" "$fix/tools/kalshi-sdk" "$fix.meta" "$fix.bare"
  printf 'info:\n  title: demo\n  version: %s\n' "$spec" > "$fix/spec/openapi.yaml"
  printf 'info:\n  title: demo\n  version: 2.0.0\n' > "$fix/spec/asyncapi.yaml"
  write_wheel "$fix/upstream/async.whl" kalshi_python_async
  write_wheel "$fix/upstream/sync.whl" kalshi_python_sync
  python3 - "$fix" "$spec" "$crate" "$py_spec" <<'PY'
import hashlib, json, pathlib, sys
fix, spec, crate, py_spec = sys.argv[1:]
root = pathlib.Path(fix)
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
async_wheel = root / "upstream" / "async.whl"
sync_wheel = root / "upstream" / "sync.whl"
async_sha = sha(async_wheel)
sync_sha = sha(sync_wheel)
index = {
    "info": {"version": "3.30.0"},
    "urls": [{
        "packagetype": "bdist_wheel",
        "filename": "kalshi_python_async-3.30.0-py3-none-any.whl",
        "url": "file://" + str(async_wheel),
        "digests": {"sha256": async_sha},
    }],
}
(root / "upstream" / "async.json").write_text(json.dumps(index))
sync_index = {
    "info": {"version": "3.30.0"},
    "urls": [{
        "packagetype": "bdist_wheel",
        "filename": "kalshi_python_sync-3.30.0-py3-none-any.whl",
        "url": "file://" + str(sync_wheel),
        "digests": {"sha256": sync_sha},
    }],
}
(root / "upstream" / "sync.json").write_text(json.dumps(sync_index))
pins = f'''[openapi]
info_version = "{spec}"
sha256 = "{sha(root / "spec" / "openapi.yaml")}"
path = "spec/openapi.yaml"
source = "file://{root / "spec" / "openapi.yaml"}"

[asyncapi]
info_version = "2.0.0"
sha256 = "{sha(root / "spec" / "asyncapi.yaml")}"
path = "spec/asyncapi.yaml"
source = "file://{root / "spec" / "asyncapi.yaml"}"

[generator]
package_version = "{crate}"
additional_properties = "library=reqwest,packageVersion={crate}"

[python_sdk]
distribution = "kalshi_python_async"
version = "3.30.0"
openapi_info_version = "{py_spec}"
wheel_sha256 = "{async_sha}"
source = "file://{root / "upstream" / "async.json"}"

[python_sdk_sync]
distribution = "kalshi_python_sync"
version = "3.30.0"
wheel_sha256 = "{sync_sha}"
source = "file://{root / "upstream" / "sync.json"}"
'''
(root / "tools" / "kalshi-sdk" / "pins.toml").write_text(pins)
PY
  cat > "$fix/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = ["crates/kalshi-api-openapi", "crates/kalshi-api-pins", "crates/kalshi-api"]
EOF
  cat > "$fix/Cargo.lock" <<EOF
# lock fixture
version = 3

[[package]]
name = "kalshi-api"
version = "${crate}"

[[package]]
name = "kalshi-api-openapi"
version = "${crate}"

[[package]]
name = "kalshi-api-pins"
version = "${crate}"
EOF
  for name in kalshi-api kalshi-api-openapi kalshi-api-pins; do
    cat > "$fix/crates/$name/Cargo.toml" <<EOF
[package]
name = "${name}"
version = "${crate}"
edition = "2021"
EOF
  done
  if [[ "${PUBLISH_FALSE:-}" == "1" ]]; then
    printf '\npublish = false\n' >> "$fix/crates/kalshi-api/Cargo.toml"
  fi
  printf -- '- Package version: %s\n' "$crate" > "$fix/crates/kalshi-api-openapi/README.md"
  cat > "$fix/tools/kalshi-sdk/generate.sh" <<EOF
#!/bin/bash
echo packageVersion=${crate}
EOF
  if [[ "${PENDING:-}" == "1" ]]; then
    cat > "$fix/CHANGELOG.md" <<'EOF'
# Changelog

## [Unreleased]

- Pending official Python SDK conformance: Python OpenAPI 0.0.1, spec 0.0.1.
EOF
  else
    cat > "$fix/CHANGELOG.md" <<'EOF'
# Changelog

## [Unreleased]

- Fixture notes.
EOF
  fi
  cat > "$fix/README.md" <<EOF
# fixture

Generated from OpenAPI ${spec} and AsyncAPI 2.0.0.

![last synced](https://img.shields.io/badge/last%20synced-OpenAPI%203.31.0%20%7C%20UTC%20placeholder-blue)
EOF
  printf '{}\n' > "$fix/tools/kalshi-sdk/expected_py334.json"
  printf '{}\n' > "$fix/tools/kalshi-sdk/expected_api_async.json"
  for tool in fixed_point_fields.py subaccount_semantics.py ws_fields.py conformance.py; do
    printf '#!/usr/bin/env python3\nimport sys\nsys.exit(0)\n' > "$fix/tools/kalshi-sdk/$tool"
  done
  cat > "$fix/tools/kalshi-sdk/drift.sh" <<'EOF'
#!/bin/bash
set -euo pipefail
count=${DRIFT_COUNT:?}
n=0
if [[ -f $count ]]; then n=$(cat "$count"); fi
n=$((n + 1))
printf '%s\n' "$n" > "$count"
if [[ "${DRIFT_FAIL:-}" == "1" ]]; then
  echo "drift failed" >&2
  exit 1
fi
if [[ "$n" -ge 2 && -n "${EXPECT_VERSION:-}" ]]; then
  root=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
  grep -q "packageVersion=${EXPECT_VERSION}" "$root/tools/kalshi-sdk/generate.sh"
  grep -qx -- "- Package version: ${EXPECT_VERSION}" "$root/crates/kalshi-api-openapi/README.md"
fi
exit 0
EOF
  printf '#!/bin/bash\nexit 0\n' > "$fix/tools/kalshi-sdk/public_content_grep.sh"
  cp "$ROOT/tools/kalshi-sdk/release.sh" "$ROOT/tools/kalshi-sdk/upstream_lib.py" "$fix/tools/kalshi-sdk/"
  cat > "$fix/bin/cargo" <<'EOF'
#!/bin/bash
set -euo pipefail
printf '%s\n' "$*" >> "${CARGO_LOG:?}"
if [[ -n "${CARGO_REGISTRY_TOKEN:-}" && "${1:-}" != "publish" ]]; then
  printf 'leak\n' >> "$CARGO_LOG"
fi
case "${1:-}" in
  fmt|build|test|clippy)
    if [[ "${FINAL_RED:-}" == "$1" ]] && ! grep -q "version = \"${ORIGINAL_VERSION}\"" crates/kalshi-api/Cargo.toml; then exit 9; fi
    exit 0
    ;;
  semver-checks)
    mode=$(cat "${SEMVER_MODE:?}")
    case "$mode" in
      patch) exit 0 ;;
      compatible)
        [[ "$*" != *"--release-type patch"* ]] && exit 0
        exit 100
        ;;
      baseline-error)
        echo "No available baseline versions" >&2
        exit 101
        ;;
      breaking)
        if [[ "$*" == *"--release-type major"* ]]; then exit 0; fi
        exit 100
        ;;
      unfixable) exit 100 ;;
      initial)
        echo "kalshi-api-openapi not found in registry (crates.io)." >&2
        exit 101
        ;;
      error)
        echo "rustdoc failed" >&2
        exit 101
        ;;
      *) echo "bad semver mode" >&2; exit 101 ;;
    esac
    ;;
  publish)
    mark=missing
    if [[ -n "${CARGO_REGISTRY_TOKEN:-}" ]]; then mark=set; fi
    pkg=
    prev=
    for arg in "$@"; do
      if [[ "$prev" == "-p" ]]; then pkg=$arg; fi
      prev=$arg
    done
    printf 'publish %s token=%s\n' "$pkg" "$mark" >> "${PUBLISH_LOG:?}"
    exit 0
    ;;
  *)
    echo "unexpected cargo $*" >&2
    exit 9
    ;;
esac
EOF
  cat > "$fix/bin/gh" <<'EOF'
#!/bin/bash
printf '%s\n' "$*" >> "${GH_LOG:?}"
if [[ "${1:-}" == "issue" && "${2:-}" == "create" ]]; then exit 0; fi
if [[ "${1:-}" == "pr" && "${2:-}" == "create" ]]; then exit 0; fi
echo "unexpected gh $*" >&2
exit 3
EOF
  cat > "$fix/bin/curl" <<'EOF'
#!/usr/bin/env python3
import json, os, pathlib, subprocess, sys
args = sys.argv[1:]
url = next(arg for arg in args if arg.startswith(("https://", "http://", "file://")))
if url.startswith("https://crates.io/api/v1/crates/"):
    name = url.rsplit("/", 1)[-1]
    state = pathlib.Path(os.environ["REGISTRY_STATE"]).read_text().strip()
    if state == "transport-error": sys.exit(7)
    if state == "mixed": state = "404" if name == "kalshi-api-openapi" else "3.31.0"
    status = state if state in ("404", "403", "429", "500") else "200"
    body = {"crate": {"id": name}, "versions": [{"num": state}]}
    pathlib.Path(args[args.index("--output") + 1]).write_text(json.dumps(body))
    print(status, end="")
    sys.exit(0)
if not (url.startswith("file://") or url.startswith("http://127.0.0.1:")): sys.exit(97)
sys.exit(subprocess.call(["/usr/bin/curl", *args]))
EOF
  chmod +x "$fix/bin/curl"
  printf '%s\n' "$registry" > "$fix.meta/registry"
  if [[ "${SEMVER_MODE_VALUE:-}" == initial ]]; then echo 404 > "$fix.meta/registry"; fi
  chmod +x "$fix/bin/cargo" "$fix/bin/gh" "$fix/tools/kalshi-sdk/"*.sh "$fix/tools/kalshi-sdk/"*.py
  printf 'fixture-token-600\n' > "$fix.meta/token"
  chmod 600 "$fix.meta/token"
  printf '%s\n' "${SEMVER_MODE_VALUE:-compatible}" > "$fix.meta/semver"
  : > "$fix.meta/cargo.log"
  : > "$fix.meta/publish.log"
  : > "$fix.meta/gh.log"
  : > "$fix.meta/drift.count"
  git init -q -b main "$fix"
  git -C "$fix" config user.name wangschien
  git -C "$fix" config user.email schienwang@gmail.com
  git -C "$fix" config commit.gpgsign false
  git -C "$fix" add -A
  git -C "$fix" commit -q -m "fixture"
  if [[ "${SEMVER_MODE_VALUE:-}" != initial ]]; then git -C "$fix" tag "v${crate}"; fi
  if [[ "$spec" != "$crate" ]]; then
    printf '// compatible generated addition\n' > "$fix/crates/kalshi-api-openapi/src/models/update.rs"
  fi
  (cd "$fix" && git ls-files -z | xargs -0 sha256sum) > "$fix.meta/before.sha256"
  git -C "$fix" diff > "$fix.meta/before.diff"
  git -C "$fix" status --porcelain > "$fix.meta/before.status"
  git init -q --bare "$fix.bare"
  git -C "$fix" remote add origin "$fix.bare"
  git -C "$fix" push -q origin main
}

run_release() {
  local fix=$1
  shift
  KALSHI_RELEASE_LOG=$fix.meta/release.log
  CARGO_LOG=$fix.meta/cargo.log
  (
    cd "$fix"
    export PATH="$fix/bin:${PATH}"
    export KALSHI_RELEASE_LOG="$fix.meta/release.log"
    export KALSHI_RELEASE_NOW=2026-09-28T15:04:00Z
    export KALSHI_OPENAPI_URL=${OPENAPI_OVERRIDE:-file://$fix/spec/openapi.yaml}
    export KALSHI_ASYNCAPI_URL="file://$fix/spec/asyncapi.yaml"
    export KALSHI_PYPI_JSON_URL="file://$fix/upstream/async.json"
    export KALSHI_PYPI_SYNC_JSON_URL="file://$fix/upstream/sync.json"
    export KALSHI_CARGO_REGISTRY_TOKEN_FILE="$fix.meta/token"
    export TELEGRAM_CHAT_ID=fixture-chat
    export KALSHI_TELEGRAM_URL="http://127.0.0.1:${TG_PORT}/send"
    export DRIFT_COUNT="$fix.meta/drift.count"
    export SEMVER_MODE="$fix.meta/semver"
    export REGISTRY_STATE="$fix.meta/registry"
    export ORIGINAL_VERSION="$(python3 "$LIB" package-version --manifest "$fix/crates/kalshi-api/Cargo.toml")"
    export CARGO_LOG="$fix.meta/cargo.log"
    export PUBLISH_LOG="$fix.meta/publish.log"
    export GH_LOG="$fix.meta/gh.log"
    export EXPECT_VERSION="${EXPECT_VERSION:-}"
    export DRIFT_FAIL="${DRIFT_FAIL:-}"
    bash tools/kalshi-sdk/release.sh "$@"
  )
}

release_line() {
  local fix=$1
  tail -n 1 "$fix.meta/release.log"
}

test_check_compatible() {
  local fix=$BASE/check-ok
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  run_release "$fix" --check >"$fix.meta/stdout"
  assert_eq "$(wc -l < "$fix.meta/stdout" | tr -d ' ')" "1" "check stdout lines"
  grep -q 'action=check version=3.32.0 kind=minor bump=minor publish=yes' "$fix.meta/stdout" || fail "check line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "check published"
  assert_eq "$(cat "$fix.meta/drift.count")" "2" "check drift count"
  assert_restored "$fix"
}

test_publish_compatible() {
  local fix=$BASE/publish-ok
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  EXPECT_VERSION=3.32.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=tag version=3.32.0' "$fix.meta/stdout" || fail "tag line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "tag path uploaded crates"
  grep -q leak "$fix.meta/cargo.log" && fail "token leaked into a gate"
  grep -q 'fixture-token-600' "$fix.meta/release.log" && fail "token written to the release log"
  assert_eq "$(cat "$fix.meta/drift.count")" "2" "publish reruns drift"
  assert_eq "$(git -C "$fix" rev-parse --abbrev-ref HEAD)" "main" "publish stays on main"
  git -C "$fix" rev-parse "v3.32.0" >/dev/null
  git -C "$fix.bare" rev-parse "v3.32.0" >/dev/null
  assert_eq "$(git -C "$fix" log -1 --format='%an %ae')" "wangschien schienwang@gmail.com" "commit author"
  assert_eq "$(git -C "$fix" log -1 --format='%s')" "Release 3.32.0 for OpenAPI 3.32.0." "subject"
  grep -q 'Pending official Python SDK conformance: Python OpenAPI 3.30.0, spec 3.32.0.' "$fix/CHANGELOG.md" || fail "pending changelog"
  grep -q 'packageVersion=3.32.0' "$fix/tools/kalshi-sdk/generate.sh" || fail "published generator version"
  telegram_has 'action=tag'
}

test_publish_breaking() {
  local fix=$BASE/publish-break
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.32.0 SEMVER_MODE_VALUE=breaking build_release_fixture "$fix"
  EXPECT_VERSION=4.0.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=tag version=4.0.0 kind=major' "$fix.meta/stdout" || fail "breaking line: $(cat "$fix.meta/stdout")"
  git -C "$fix" rev-parse "v4.0.0" >/dev/null
  grep -q 'OpenAPI 3.32.0 sha256' "$fix/CHANGELOG.md" || fail "breaking changelog spec"
  grep -q 'Pending official Python SDK conformance' "$fix/CHANGELOG.md" && fail "pending note was not removed"
  assert_eq "$(python3 "$LIB" package-version --manifest "$fix/crates/kalshi-api/Cargo.toml")" "4.0.0" "breaking crate version"
}

test_initial() {
  local fix=$BASE/initial
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 SEMVER_MODE_VALUE=initial build_release_fixture "$fix"
  EXPECT_VERSION=3.31.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=tag version=3.31.0 kind=initial' "$fix.meta/stdout" || fail "initial line: $(cat "$fix.meta/stdout")"
  git -C "$fix" rev-parse "v3.31.0" >/dev/null
  [[ ! -s $fix.meta/publish.log ]] || fail "initial uploaded crates"
}

test_pull_request() {
  local fix=$BASE/pr
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  local main_before
  main_before=$(git -C "$fix" rev-parse main)
  EXPECT_VERSION=3.32.0 AUTO_RELEASE=off run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=pr version=3.32.0' "$fix.meta/stdout" || fail "pr line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "pull request published"
  git -C "$fix" rev-parse "v3.32.0" >/dev/null 2>&1 && fail "pull request tagged"
  assert_eq "$(git -C "$fix" rev-parse --abbrev-ref HEAD)" "sync/openapi-3.32.0" "pr branch"
  assert_eq "$(git -C "$fix" rev-parse main)" "$main_before" "pr left main"
  git -C "$fix.bare" rev-parse "sync/openapi-3.32.0" >/dev/null
  grep -q 'pr create' "$fix.meta/gh.log" || fail "gh pr was not called"
  grep -q 'sync/openapi-3.32.0' "$fix.meta/gh.log" || fail "gh head branch"
}

test_hold() {
  local fix=$BASE/hold
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  local rc
  set +e
  EXPECT_VERSION=3.32.0 run_release "$fix" --publish >"$fix.meta/stdout"
  rc=$?
  set -e
  assert_eq "$rc" "2" "hold status"
  grep -q 'action=hold' "$fix.meta/stdout" || fail "hold line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "hold published"
  [[ ! -s $fix.meta/gh.log ]] || fail "hold opened an issue or pr"
  assert_restored "$fix"
}

test_noop() {
  local fix=$BASE/noop
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=noop' "$fix.meta/stdout" || fail "noop line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "noop published"
  assert_eq "$(git -C "$fix" rev-list --count HEAD)" "1" "noop committed"
}

test_note() {
  local fix=$BASE/note
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 PENDING=1 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  EXPECT_VERSION=3.31.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=note' "$fix.meta/stdout" || fail "note line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "note published"
  assert_eq "$(git -C "$fix" rev-list --count v3.31.0)" "1" "note moved the release tag"
  git -C "$fix" diff v3.31.0 -- README.md crates/kalshi-api-openapi crates/kalshi-api Cargo.lock | grep -q . && fail "note changed published inputs"
  grep -q 'Pending official Python SDK conformance' "$fix/CHANGELOG.md" && fail "note remained"
  assert_eq "$(git -C "$fix" rev-list --count HEAD)" "2" "note commit"
}

test_drift_red() {
  local fix=$BASE/drift-red
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  local rc
  set +e
  DRIFT_FAIL=1 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "red drift exited 0"
  grep -q 'gates=fail:drift action=refuse' "$fix.meta/stdout" || fail "drift refuse line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "red drift published"
  grep -q 'issue create' "$fix.meta/gh.log" || fail "red drift did not open an issue"
  grep -q 'release refused: drift' "$fix.meta/gh.log" || fail "issue title"
  telegram_has 'gates=fail:drift'
}

test_unfixable_break() {
  local fix=$BASE/unfixable
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=unfixable build_release_fixture "$fix"
  local rc
  set +e
  AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "unfixable break exited 0"
  grep -q 'gates=fail:semver-checks' "$fix.meta/stdout" || fail "unfixable line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "unfixable break published"
}

test_semver_error() {
  local fix=$BASE/semver-error
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=error build_release_fixture "$fix"
  local rc
  set +e
  AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "semver error exited 0"
  grep -q 'gates=fail:semver-checks' "$fix.meta/stdout" || fail "semver line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "semver error published"
}

test_bad_token() {
  local fix=$BASE/bad-token
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  chmod 644 "$fix.meta/token"
  EXPECT_VERSION=3.32.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=tag version=3.32.0' "$fix.meta/stdout" || fail "token file blocked the tag: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "token file uploaded crates"
  git -C "$fix" rev-parse "v3.32.0" >/dev/null
}

test_publish_false() {
  local fix=$BASE/no-publish-flag
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PUBLISH_FALSE=1 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  local rc
  set +e
  AUTO_RELEASE=on run_release "$fix" --check >"$fix.meta/stdout"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "publish=false check exited 0"
  grep -q 'gates=fail:publishable' "$fix.meta/stdout" || fail "publishable line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "publish=false published"
}

test_upstream_mismatch() {
  local fix=$BASE/mismatch
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  printf 'info:\n  title: other\n  version: 9.9.9\n' > "$fix/upstream/other.yaml"
  local rc
  set +e
  OPENAPI_OVERRIDE="file://$fix/upstream/other.yaml" AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "mismatch exited 0"
  grep -q 'gates=fail:upstream-pin' "$fix.meta/stdout" || fail "mismatch line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "mismatch published"
}

test_unexpected_path() {
  local fix=$BASE/unexpected
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=compatible build_release_fixture "$fix"
  printf 'notes\n' > "$fix/NOTES.txt"
  local rc
  set +e
  EXPECT_VERSION=3.32.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "unexpected path exited 0"
  grep -q 'gates=fail:unexpected-path' "$fix.meta/stdout" || fail "unexpected line: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "unexpected path published"
  assert_eq "$(git -C "$fix" rev-list --count HEAD)" "1" "unexpected path committed"
}

test_phrase_and_urls() {
  local phrase hits c p
  c=$(printf '%s' cargo)
  p=$(printf '%s' publish)
  phrase="$c $p"
  hits=$(grep -RIl --exclude-dir=.git --exclude-dir=target -e "$phrase" "$ROOT" || true)
  [[ -z "$hits" ]] || fail "upload phrase returned: $hits"
  grep -q 'publish -p' "$ROOT/tools/kalshi-sdk/publish_crates.sh" || fail "publish script missing"
  if grep -q -e "$phrase" "$ROOT/tools/kalshi-sdk/release.sh" "$ROOT/.github/workflows/release.yml"; then
    fail "release entry still uploads crates"
  fi
  local pins=$ROOT/tools/kalshi-sdk/pins.toml
  grep -q 'source = "https://docs.kalshi.com/openapi.yaml"' "$pins" || fail "openapi source"
  grep -q 'source = "https://docs.kalshi.com/asyncapi.yaml"' "$pins" || fail "asyncapi source"
  grep -q 'source = "https://pypi.org/pypi/kalshi_python_async/json"' "$pins" || fail "pypi source"
  grep -q 'sources --pins' "$ROOT/tools/kalshi-sdk/check_upstream.sh" || fail "check does not read pins"
  grep -q 'sources --pins' "$ROOT/tools/kalshi-sdk/release.sh" || fail "release does not read pins"
  if grep -q 'https://docs.kalshi.com/openapi.yaml' "$ROOT/tools/kalshi-sdk/check_upstream.sh"; then
    fail "openapi url is hardcoded in the check script"
  fi
  bash "$ROOT/tools/kalshi-sdk/public_content_grep.sh"
}

assert_restored() {
  local fix=$1
  git -C "$fix" diff > "$fix.meta/after.diff"
  git -C "$fix" status --porcelain > "$fix.meta/after.status"
  cmp "$fix.meta/before.diff" "$fix.meta/after.diff" || fail "rollback diff"
  cmp "$fix.meta/before.status" "$fix.meta/after.status" || fail "rollback status"
  (cd "$fix" && sha256sum --check "$fix.meta/before.sha256" >/dev/null) || fail "rollback file bytes"
}

test_compatible_after_major() {
  # Audit repro-compatible-after-major: SDK already at 4.0.0, OpenAPI moves
  # 3.32 -> 3.33, Python stays behind, semver is compatible. The old decision
  # returned action=note version=4.0.0 bump=none and published nothing.
  local mode fix expected
  for mode in compatible patch; do
    fix=$BASE/after-major-$mode
    SPEC_VERSION=3.33.0 CRATE_VERSION=4.0.0 PY_SPEC=3.30.0 SEMVER_MODE_VALUE=$mode build_release_fixture "$fix"
    expected=4.0.1
    [[ "$mode" == compatible ]] && expected=4.1.0
    EXPECT_VERSION=$expected AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
    grep -q "action=tag version=$expected" "$fix.meta/stdout" || fail "compatible update after major: $(cat "$fix.meta/stdout")"
    grep -q 'bump=none' "$fix.meta/stdout" && fail "compatible update was swallowed as a note"
    grep -q 'Pending official Python SDK conformance: Python OpenAPI 3.30.0, spec 3.33.0.' "$fix/CHANGELOG.md" || fail "pending note missing from the release"
    [[ ! -s $fix.meta/publish.log ]] || fail "compatible update uploaded crates"
    grep -q -- '--baseline-version 4.0.0' "$fix.meta/cargo.log" || fail "explicit registry baseline"
    # The next run compares against the new published tag, not the older spec.
    echo "$expected" > "$fix.meta/registry"
    AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/repeat"
    grep -q 'action=noop' "$fix.meta/repeat" || fail "repeat release was not noop: $(cat "$fix.meta/repeat")"
    [[ ! -s $fix.meta/publish.log ]] || fail "repeat uploaded crates"
  done
}

test_async_only() {
  # Audit repro-async-only-no-release changed only the AsyncAPI bytes and the
  # pin hash. That must be committed and not published. A published-file change
  # from the same AsyncAPI update must release.
  local fix=$BASE/async-pins
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 SEMVER_MODE_VALUE=patch build_release_fixture "$fix"
  python3 - "$fix" <<'PY'
import hashlib, pathlib, re, sys
root = pathlib.Path(sys.argv[1])
spec = root / "spec/asyncapi.yaml"
spec.write_text(spec.read_text() + "channels:\n  new_channel: {}\n")
pins = root / "tools/kalshi-sdk/pins.toml"
text = pins.read_text()
start = text.index("[asyncapi]")
end = text.index("[generator]")
digest = hashlib.sha256(spec.read_bytes()).hexdigest()
text = text[:start] + re.sub(r'sha256 = "[a-f0-9]+"', f'sha256 = "{digest}"', text[start:end]) + text[end:]
pins.write_text(text)
PY
  AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=note' "$fix.meta/stdout" || fail "async pin-only was not committed: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "async pin-only published"
  assert_eq "$(git -C "$fix" rev-list --count HEAD)" "2" "async pin-only commit"
  git -C "$fix" diff v3.31.0 -- README.md crates Cargo.toml Cargo.lock | grep -q . && fail "async pin-only changed published files"

  fix=$BASE/async-only
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 SEMVER_MODE_VALUE=patch build_release_fixture "$fix"
  python3 - "$fix" <<'PY'
import hashlib, pathlib, re, sys
root = pathlib.Path(sys.argv[1])
spec = root / 'spec/asyncapi.yaml'
spec.write_text(spec.read_text().replace('2.0.0', '2.1.0'))
pins = root / 'tools/kalshi-sdk/pins.toml'
text = pins.read_text().replace('info_version = "2.0.0"', 'info_version = "2.1.0"')
text = re.sub(r'(\[asyncapi\]\ninfo_version = "2.1.0"\nsha256 = ")[^"]+',
              lambda m: m[1] + hashlib.sha256(spec.read_bytes()).hexdigest(), text)
pins.write_text(text)
(root / 'crates/kalshi-api/ws.rs').write_text('// compatible websocket schema update\n')
PY
  EXPECT_VERSION=3.31.1 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=tag version=3.31.1' "$fix.meta/stdout" || fail "async-only release: $(cat "$fix.meta/stdout")"
  grep -q 'AsyncAPI 2.1.0 sha256' "$fix/CHANGELOG.md" || fail "async version missing from changelog"
  grep -q 'Generated from OpenAPI 3.31.0 and AsyncAPI 2.1.0.' "$fix/README.md" || fail "readme spec versions"
}

test_registry_refusals() {
  local state mode gate fix
  for state in 4.0.0 403 429 500 transport-error malformed mixed; do
    fix=$BASE/registry-$state
    mode=compatible
    gate=registry
    if [[ "$state" == 4.0.0 ]]; then mode=baseline-error; gate=semver-checks; fi
    SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 REGISTRY_VERSION=$state SEMVER_MODE_VALUE=$mode build_release_fixture "$fix"
    if AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"; then fail "registry $state accepted"; fi
    grep -q "gates=fail:$gate" "$fix.meta/stdout" || fail "wrong registry refusal for $state"
    [[ ! -s $fix.meta/publish.log ]] || fail "registry $state published"
    assert_eq "$(git -C "$fix" rev-list --count HEAD)" "1" "registry refusal committed"
    assert_restored "$fix"
  done
  # Audit repro-baseline-error-treated-initial: registry has only 4.0.0, the
  # worktree is 3.31.0, and semver-checks exits 101 with "No available baseline
  # versions". That used to publish as kind=initial. It must refuse.
  fix=$BASE/registry-4.0.0
  if grep -q 'kind=initial' "$fix.meta/stdout"; then fail "baseline error became initial"; fi
  if grep -F 'No available baseline versions' "$ROOT/tools/kalshi-sdk/release.sh"; then
    fail "baseline error is still classified from semver output"
  fi
  # An existing registry crate with the old 'not found' diagnostic is not initial.
  fix=$BASE/registry-existing-not-found
  SEMVER_MODE_VALUE=initial build_release_fixture "$fix"
  echo 3.31.0 > "$fix.meta/registry"
  if AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"; then fail "diagnostic authorized initial"; fi
  grep -q 'gates=fail:semver-checks' "$fix.meta/stdout" || fail "existing not-found refusal"
  [[ ! -s $fix.meta/publish.log ]] || fail "existing crate published initially"
  # A tool that exits 0 still must not publish a checkout older than the registry.
  fix=$BASE/registry-older-checkout
  SPEC_VERSION=3.32.0 CRATE_VERSION=3.31.0 REGISTRY_VERSION=4.0.0 SEMVER_MODE_VALUE=patch build_release_fixture "$fix"
  if AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"; then fail "older checkout accepted"; fi
  grep -q 'gates=fail:published-tree' "$fix.meta/stdout" || fail "older checkout gate: $(cat "$fix.meta/stdout")"
  [[ ! -s $fix.meta/publish.log ]] || fail "older checkout published"
  assert_eq "$(git -C "$fix" rev-list --count HEAD)" "1" "older checkout committed"
}

test_missing_tag() {
  local fix=$BASE/missing-tag
  build_release_fixture "$fix"
  git -C "$fix" tag -d v3.31.0 >/dev/null
  if AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"; then fail "missing published tag accepted"; fi
  grep -q 'gates=fail:published-tree' "$fix.meta/stdout" || fail "missing tag gate"
  [[ ! -s $fix.meta/publish.log ]] || fail "missing tag published"
}

test_final_gate_rollback() {
  local gate fix tool
  for gate in local-pin aligned publishable public-content fmt fixed-point subaccount field-list clippy build test upstream-pin conformance drift semver-checks; do
    fix=$BASE/final-$gate
    SEMVER_MODE_VALUE=breaking build_release_fixture "$fix"
    # Count each real gate invocation at the fixture boundary. Fail only after
    # version application; the corresponding gate has already passed once.
    case "$gate" in
      local-pin|aligned|publishable|upstream-pin)
        mv "$fix/tools/kalshi-sdk/upstream_lib.py" "$fix/tools/kalshi-sdk/upstream_real.py"
        cat > "$fix/tools/kalshi-sdk/upstream_lib.py" <<'PY'
import os, pathlib, subprocess, sys
root = pathlib.Path(__file__).resolve().parents[2]
if sys.argv[1] == os.environ['FINAL_RED'] and 'version = "4.0.0"' in (root / 'crates/kalshi-api/Cargo.toml').read_text():
    sys.exit(9)
sys.exit(subprocess.call([sys.executable, str(pathlib.Path(__file__).with_name('upstream_real.py')), *sys.argv[1:]]))
PY
        ;;
      fmt|clippy|build|test) : ;;
      semver-checks)
        # The initial major classification succeeds; final confirmation fails.
        sed -i '/case "${1:-}" in/i if [[ "${1:-}" == semver-checks ]] \&\& grep -q '\''version = "4.0.0"'\'' crates/kalshi-api/Cargo.toml; then exit 101; fi' "$fix/bin/cargo"
        ;;
      *)
        case "$gate" in
          fixed-point) tool=fixed_point_fields.py ;;
          subaccount) tool=subaccount_semantics.py ;;
          field-list) tool=ws_fields.py ;;
          conformance) tool=conformance.py ;;
          public-content) tool=public_content_grep.sh ;;
          drift) tool=drift.sh ;;
        esac
        if [[ "$tool" == *.py ]]; then
          printf '\nimport pathlib\nif '\''version = "4.0.0"'\'' in pathlib.Path("crates/kalshi-api/Cargo.toml").read_text(): raise SystemExit(9)\n' > "$fix/tools/kalshi-sdk/$tool"
        else
          sed -i '2i if grep -q '\''version = "4.0.0"'\'' crates/kalshi-api/Cargo.toml; then exit 9; fi' "$fix/tools/kalshi-sdk/$tool"
        fi
        ;;
    esac
    git -C "$fix" add -A
    git -C "$fix" commit -q -m 'final gate fixture'
    git -C "$fix" diff > "$fix.meta/before.diff"
    git -C "$fix" status --porcelain > "$fix.meta/before.status"
    (cd "$fix" && git ls-files -z | xargs -0 sha256sum) > "$fix.meta/before.sha256"
    if FINAL_RED=$gate EXPECT_VERSION=4.0.0 AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"; then fail "final $gate accepted"; fi
    grep -q "gates=fail:$gate-after" "$fix.meta/stdout" || fail "final $gate did not reach expected gate"
    [[ ! -s $fix.meta/publish.log ]] || fail "final $gate published"
    assert_restored "$fix"
  done
}

test_published_inputs() {
  local fix=$BASE/published-inputs
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 build_release_fixture "$fix"
  python3 - "$fix" "$LIB" <<'PY'
import importlib.util, pathlib, subprocess, sys
root = pathlib.Path(sys.argv[1])
spec = importlib.util.spec_from_file_location('upstream', sys.argv[2])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
changed = lambda: module.published_changed(root, '3.31.0', '3.31.0')
assert not changed()
for name in ('README.md', 'Cargo.toml', 'Cargo.lock', 'LICENSE', 'rust-toolchain.toml',
             'crates/kalshi-api/Cargo.toml', 'crates/kalshi-api-openapi/README.md',
             'crates/kalshi-api/new.rs'):
    path = root / name
    old = path.read_bytes() if path.exists() else None
    path.write_bytes((old or b'') + b'\n// published input changed\n')
    assert changed(), name
    if old is None: path.unlink()
    else: path.write_bytes(old)
path = root / 'crates/kalshi-api-openapi/README.md'
old = path.read_bytes()
path.unlink()
assert changed(), 'deleted published input'
path.write_bytes(old)
for name in ('CHANGELOG.md', 'spec/asyncapi.yaml', 'tools/kalshi-sdk/pins.toml'):
    path = root / name
    path.write_bytes(path.read_bytes() + b'\n# note only\n')
assert not changed(), 'non-published notes selected release'
path = root / 'crates/kalshi-api/new.rs'
path.write_text('// already committed since published tag\n')
subprocess.run(['git', '-C', str(root), 'add', '-A'], check=True)
subprocess.run(['git', '-C', str(root), 'commit', '-qm', 'committed update'], check=True)
assert changed(), 'comparison used HEAD instead of published tag'
PY
}

test_pin_only_note() {
  local fix=$BASE/pin-note
  SPEC_VERSION=3.31.0 CRATE_VERSION=3.31.0 PY_SPEC=3.31.0 SEMVER_MODE_VALUE=patch build_release_fixture "$fix"
  printf '\n# conformance pin refresh\n' >> "$fix/tools/kalshi-sdk/pins.toml"
  rm "$fix.meta/token"
  AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"
  grep -q 'action=note' "$fix.meta/stdout" || fail "pin-only update was not committed"
  git -C "$fix" diff --exit-code v3.31.0 -- README.md Cargo.lock crates >/dev/null || fail "pin-only update changed published files"
  assert_eq "$(git -C "$fix" rev-list --count HEAD)" "2" "pin-only commit"
  AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/repeat"
  grep -q 'action=noop' "$fix.meta/repeat" || fail "pin-only repeat"
  [[ ! -s $fix.meta/publish.log ]] || fail "pin-only published"
}

test_real_drift_after_apply() {
  # Audit S4-A1 schedule: apply 4.0.0 on the 3.31.0 tree, then the real
  # generator drift must stay clean. Putting the old Package version line back
  # must make that same drift fail.
  [[ -n "${KALSHI_GENERATOR_JAR:-}" && -f ${KALSHI_GENERATOR_JAR} ]] || fail "set KALSHI_GENERATOR_JAR to the pinned generator jar"
  command -v java >/dev/null || fail "java is required for the real drift schedule"
  local slot saved dest
  slot=$(command -v cargo-slot) || fail "cargo-slot is required for the real drift schedule"
  saved=$PATH
  dest=$BASE/real-drift
  mkdir -p "$dest/slot-bin"
  tar -C "$ROOT" --exclude .git --exclude target -cf - . | tar -C "$dest" -xf -
  python3 "$dest/tools/kalshi-sdk/upstream_lib.py" apply --root "$dest" --version 4.0.0 --now 2026-09-28T15:04:00Z >/dev/null
  grep -qx -- '- Package version: 4.0.0' "$dest/crates/kalshi-api-openapi/README.md" || fail "apply left the generated README"
  cat > "$dest/slot-bin/cargo" << EOF
#!/bin/bash
export PATH="$saved"
exec "$slot" "\$@"
EOF
  chmod +x "$dest/slot-bin/cargo"
  if ! (
    export PATH="$dest/slot-bin:$saved"
    export JAVA="${JAVA:-java}"
    cd "$dest"
    bash tools/kalshi-sdk/drift.sh
  ) >"$dest.ok.log" 2>&1; then
    cat "$dest.ok.log" >&2 || true
    fail "real drift after 3.31.0 -> 4.0.0 apply"
  fi
  grep -q 'drift ok' "$dest.ok.log" || fail "real drift did not report ok"
  python3 - "$dest/crates/kalshi-api-openapi/README.md" << 'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
text = path.read_text().replace("- Package version: 4.0.0\n", "- Package version: 3.31.0\n", 1)
if text == path.read_text():
    raise SystemExit("package line was not restored")
path.write_text(text)
PY
  local rc
  set +e
  (
    export PATH="$dest/slot-bin:$saved"
    export JAVA="${JAVA:-java}"
    cd "$dest"
    bash tools/kalshi-sdk/drift.sh
  ) >"$dest.stale.log" 2>&1
  rc=$?
  set -e
  [[ "$rc" -ne 0 ]] || fail "stale Package version drifted clean"
  grep -q 'Package version' "$dest.stale.log" || { cat "$dest.stale.log" >&2; fail "stale drift diff did not mention Package version"; }
}

test_apply_rollback() {
  local fix=$BASE/apply-rollback
  SEMVER_MODE_VALUE=breaking build_release_fixture "$fix"
  sed -i '/^- Package version: /d' "$fix/crates/kalshi-api-openapi/README.md"
  (cd "$fix" && git ls-files -z | xargs -0 sha256sum) > "$fix.meta/before.sha256"
  git -C "$fix" diff > "$fix.meta/before.diff"
  git -C "$fix" status --porcelain > "$fix.meta/before.status"
  if AUTO_RELEASE=on run_release "$fix" --publish >"$fix.meta/stdout"; then fail "partial apply accepted"; fi
  grep -q 'gates=fail:version-apply' "$fix.meta/stdout" || fail "partial apply gate"
  [[ ! -s $fix.meta/publish.log ]] || fail "partial apply published"
  assert_restored "$fix"
}

run_case() {
  "$@"
  printf 'PASS %s\n' "$1"
}

start_telegram
run_case unit_decide
run_case unit_apply_real_tree
run_case unit_zip_slip
run_case test_pin_sources
run_case test_unchanged
run_case test_changed_launches
run_case test_version_only
run_case test_download_failure
run_case test_busy
run_case test_check_compatible
run_case test_publish_compatible
run_case test_publish_breaking
run_case test_initial
run_case test_pull_request
run_case test_hold
run_case test_noop
run_case test_note
run_case test_drift_red
run_case test_semver_error
run_case test_unfixable_break
run_case test_bad_token
run_case test_publish_false
run_case test_upstream_mismatch
run_case test_unexpected_path
run_case test_compatible_after_major
run_case test_async_only
run_case test_registry_refusals
run_case test_missing_tag
run_case test_final_gate_rollback
run_case test_published_inputs
run_case test_pin_only_note
run_case test_apply_rollback
run_case test_real_drift_after_apply
run_case test_phrase_and_urls
printf 's4 tests ok\n'
