#!/usr/bin/env bash
# Only release entry. Every run re-checks the gates and refuses when one is red.
# AUTO_RELEASE=on commits, pushes main, and pushes the version tag.
# AUTO_RELEASE=off opens a pull request. Crates are uploaded by the tag workflow.
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: release.sh --check | --publish" >&2
  exit 2
fi
case "$1" in
  --check) MODE=check ;;
  --publish) MODE=publish ;;
  *)
    echo "usage: release.sh --check | --publish" >&2
    exit 2
    ;;
esac

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
cd "$ROOT"
LIB="$ROOT/tools/kalshi-sdk/upstream_lib.py"
LOGDIR=$(mktemp -d)
LOG=${KALSHI_RELEASE_LOG:-$ROOT/target/kalshi-sdk-release.log}
mkdir -p "$(dirname "$LOG")"
BACKUP=$LOGDIR/backup
RESTORE=no
NOW=${KALSHI_RELEASE_NOW:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}
REPO=wangschien/kalshi-api-rust
if [[ -z "${KALSHI_OPENAPI_URL:-}" || -z "${KALSHI_ASYNCAPI_URL:-}" || -z "${KALSHI_PYPI_JSON_URL:-}" || -z "${KALSHI_PYPI_SYNC_JSON_URL:-}" ]]; then
  PIN_URLS=$(python3 "$LIB" sources --pins "$ROOT/tools/kalshi-sdk/pins.toml" --sync)
  mapfile -t PIN_URLS <<< "$PIN_URLS"
fi
OPENAPI_URL=${KALSHI_OPENAPI_URL:-${PIN_URLS[0]}}
ASYNCAPI_URL=${KALSHI_ASYNCAPI_URL:-${PIN_URLS[1]}}
PYPI_URL=${KALSHI_PYPI_JSON_URL:-${PIN_URLS[2]}}
SYNC_URL=${KALSHI_PYPI_SYNC_JSON_URL:-${PIN_URLS[3]}}
unset CARGO_REGISTRY_TOKEN || true

cleanup() {
  if [[ "$RESTORE" == yes ]]; then restore_release_files; fi
  rm -rf "$LOGDIR"
}
trap cleanup EXIT

SPEC=$(python3 "$LIB" spec-version --root "$ROOT" 2>/dev/null || echo unknown)

notify() {
  printf '%s\n' "$1" >> "$LOG"
  printf '%s\n' "$1"
  local url=${KALSHI_TELEGRAM_URL:-}
  if [[ -z "$url" ]]; then
    if [[ -z "${TELEGRAM_BOT_TOKEN:-}" || -z "${TELEGRAM_CHAT_ID:-}" ]]; then
      return 0
    fi
    url="https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/sendMessage"
  fi
  if [[ -z "${TELEGRAM_CHAT_ID:-}" ]]; then
    return 0
  fi
  curl --fail --silent --show-error --max-time 30 \
    -X POST "$url" \
    --data-urlencode "chat_id=${TELEGRAM_CHAT_ID}" \
    --data-urlencode "text=$1" \
    >/dev/null || echo "telegram post failed" >&2
}

open_issue() {
  local gate=$1
  {
    echo "Gate: ${gate}"
    echo
    echo "The release script refused."
    echo
    if [[ -f "$LOGDIR/$gate.log" ]]; then
      echo '```'
      tail -n 200 "$LOGDIR/$gate.log"
      echo '```'
    fi
  } > "$LOGDIR/issue.md"
  if ! gh issue create --repo "$REPO" \
    --title "release refused: ${gate}" \
    --body-file "$LOGDIR/issue.md" >"$LOGDIR/gh-issue.log" 2>&1; then
    echo "issue open failed" >&2
  fi
}

fail_gate() {
  local gate=$1
  notify "spec=${SPEC} gates=fail:${gate} action=refuse version=-"
  open_issue "$gate"
  exit 1
}

run_gate() {
  local name=$1
  shift
  if ! "$@" >"$LOGDIR/$name.log" 2>&1; then
    fail_gate "$name"
  fi
}

fetch() {
  curl --fail --silent --show-error --location --max-time 60 --output "$2" "$1"
}

download_upstream() {
  local dest=$LOGDIR/upstream
  mkdir -p "$dest"
  fetch "$OPENAPI_URL" "$dest/openapi.yaml"
  fetch "$ASYNCAPI_URL" "$dest/asyncapi.yaml"
  fetch "$PYPI_URL" "$dest/async.json"
  fetch "$(python3 "$LIB" wheel-url --pypi "$dest/async.json")" "$dest/async.whl"
  fetch "$SYNC_URL" "$dest/sync.json"
  fetch "$(python3 "$LIB" wheel-url --pypi "$dest/sync.json")" "$dest/sync.whl"
}

gate_upstream() {
  download_upstream
  python3 "$LIB" upstream-pin \
    --root "$ROOT" \
    --openapi "$LOGDIR/upstream/openapi.yaml" \
    --asyncapi "$LOGDIR/upstream/asyncapi.yaml" \
    --async-pypi "$LOGDIR/upstream/async.json" \
    --async-wheel "$LOGDIR/upstream/async.whl" \
    --sync-pypi "$LOGDIR/upstream/sync.json" \
    --sync-wheel "$LOGDIR/upstream/sync.whl"
}

gate_conformance() {
  local async_root
  async_root=$(python3 "$LIB" unpack --wheel "$LOGDIR/upstream/async.whl" \
    --dest "$LOGDIR/async-pkg" --name kalshi_python_async)
  python3 "$ROOT/tools/kalshi-sdk/conformance.py" \
    --python "$async_root" \
    --rust "$ROOT/crates/kalshi-api-openapi/src/models" \
    --rust-api "$ROOT/crates/kalshi-api-openapi/src/apis" \
    --expected "$ROOT/tools/kalshi-sdk/expected_py334.json" \
    --api-expected "$ROOT/tools/kalshi-sdk/expected_api_async.json"
}

registry_package() {
  local pkg=$1 status
  if ! status=$(curl --silent --show-error --max-time 60 \
    --user-agent 'kalshi-api-rust-release/1.0' \
    --output "$LOGDIR/registry-${pkg}.json" --write-out '%{http_code}' \
    "https://crates.io/api/v1/crates/${pkg}"); then
    return 1
  fi
  case "$status" in
    404) printf 'initial' ;;
    200) python3 "$LIB" registry-version --name "$pkg" --json "$LOGDIR/registry-${pkg}.json" ;;
    *) echo "registry returned HTTP $status for $pkg" >&2; return 1 ;;
  esac
}

classify_package() {
  local pkg=$1 rt rc
  for rt in patch minor major; do
    if cargo semver-checks check-release --package "$pkg" \
      --baseline-version "$BASELINE" --release-type "$rt" >"$LOGDIR/semver-${pkg}-${rt}.log" 2>&1; then
      printf '%s' "$rt"
      return
    else
      rc=$?
    fi
    if [[ "$rc" -ne 100 ]]; then
      printf 'error'
      return
    fi
  done
  printf 'error'
}

combine_kinds() {
  local left=$1 right=$2 kind
  for kind in error major minor patch; do
    if [[ "$left" == "$kind" || "$right" == "$kind" ]]; then
      printf '%s' "$kind"
      return
    fi
  done
  printf 'error'
}

final_semver() {
  local pkg state
  for pkg in kalshi-api-openapi kalshi-api; do
    state=$(registry_package "$pkg") || return 1
    [[ "$state" == "$BASELINE" ]] || return 1
    if [[ "$BASELINE" != initial ]]; then
      cargo semver-checks check-release --package "$pkg" \
        --baseline-version "$BASELINE" --release-type "$KIND" || return 1
    fi
  done
}

snapshot_release_files() {
  local path
  mkdir -p "$BACKUP"
  for path in \
    crates/kalshi-api/Cargo.toml \
    crates/kalshi-api-openapi/Cargo.toml \
    crates/kalshi-api-openapi/README.md \
    crates/kalshi-api-pins/Cargo.toml \
    Cargo.lock \
    tools/kalshi-sdk/pins.toml \
    tools/kalshi-sdk/generate.sh \
    CHANGELOG.md \
    README.md
  do
    mkdir -p "$BACKUP/$(dirname "$path")"
    cp -a "$ROOT/$path" "$BACKUP/$path"
  done
}

restore_release_files() {
  local path
  for path in \
    crates/kalshi-api/Cargo.toml \
    crates/kalshi-api-openapi/Cargo.toml \
    crates/kalshi-api-openapi/README.md \
    crates/kalshi-api-pins/Cargo.toml \
    Cargo.lock \
    tools/kalshi-sdk/pins.toml \
    tools/kalshi-sdk/generate.sh \
    CHANGELOG.md \
    README.md
  do
    cp -a "$BACKUP/$path" "$ROOT/$path"
  done
}

allowed_path() {
  case "$1" in
    spec/*|tools/kalshi-sdk/*|crates/*|Cargo.toml|Cargo.lock|CHANGELOG.md|README.md|rust-toolchain.toml|LICENSE)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

commit_release() {
  local version=$1 spec=$2 path
  git add -A
  git reset -q -- tools/kalshi-sdk/upstream_change.json 2>/dev/null || true
  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if ! allowed_path "$path"; then
      git reset -q
      restore_release_files
      fail_gate unexpected-path
    fi
  done < <(git diff --cached --name-only)
  if git diff --cached --quiet; then
    restore_release_files
    fail_gate empty-commit
  fi
  echo "Release ${version} for OpenAPI ${spec}." > "$LOGDIR/commit.txt"
  GIT_AUTHOR_NAME=wangschien \
    GIT_AUTHOR_EMAIL=schienwang@gmail.com \
    GIT_COMMITTER_NAME=wangschien \
    GIT_COMMITTER_EMAIL=schienwang@gmail.com \
    git -c commit.gpgsign=false commit -F "$LOGDIR/commit.txt" >"$LOGDIR/commit.log"
}

all_gates() {
  local suffix=$1
  run_gate "local-pin${suffix}" python3 "$LIB" local-pin --root "$ROOT"
  run_gate "aligned${suffix}" python3 "$LIB" aligned --root "$ROOT"
  run_gate "publishable${suffix}" python3 "$LIB" publishable --root "$ROOT"
  run_gate "public-content${suffix}" bash "$ROOT/tools/kalshi-sdk/public_content_grep.sh"
  run_gate "fmt${suffix}" cargo fmt --all --check
  run_gate "fixed-point${suffix}" python3 "$ROOT/tools/kalshi-sdk/fixed_point_fields.py" --check
  run_gate "subaccount${suffix}" python3 "$ROOT/tools/kalshi-sdk/subaccount_semantics.py" --check
  run_gate "field-list${suffix}" python3 "$ROOT/tools/kalshi-sdk/ws_fields.py" --check
  run_gate "clippy${suffix}" cargo clippy --workspace --all-targets --locked -- -D warnings
  run_gate "build${suffix}" cargo build --workspace --locked --all-targets
  run_gate "test${suffix}" cargo test --workspace --locked --all-targets
  run_gate "upstream-pin${suffix}" gate_upstream
  run_gate "conformance${suffix}" gate_conformance
  run_gate "drift${suffix}" bash "$ROOT/tools/kalshi-sdk/drift.sh"
}

all_gates ""
CURRENT=$(python3 "$LIB" package-version --manifest "$ROOT/crates/kalshi-api/Cargo.toml")
if ! LEFT=$(registry_package kalshi-api-openapi) || ! RIGHT=$(registry_package kalshi-api); then
  fail_gate registry
fi
if [[ "$LEFT" != "$RIGHT" ]]; then
  echo "crate registry baselines differ" > "$LOGDIR/registry.log"
  fail_gate registry
fi
BASELINE=$LEFT
if [[ "$BASELINE" == initial ]]; then
  KIND=initial
  CHANGED=yes
else
  LEFT=$(classify_package kalshi-api-openapi)
  RIGHT=$(classify_package kalshi-api)
  KIND=$(combine_kinds "$LEFT" "$RIGHT")
  if [[ "$KIND" == error ]]; then
    cat "$LOGDIR"/semver-*.log > "$LOGDIR/semver-checks.log"
    fail_gate semver-checks
  fi
  if ! CHANGED=$(python3 "$LIB" published-changed --root "$ROOT" --baseline "$BASELINE" --current "$CURRENT" 2>"$LOGDIR/published-tree.log"); then
    fail_gate published-tree
  fi
fi
DECISION=$(python3 "$LIB" decide --current "$CURRENT" --kind "$KIND" --changed "$CHANGED")
VERSION=$(python3 -c 'import json,sys; print(json.load(sys.stdin)["version"])' <<<"$DECISION")
BUMP=$(python3 -c 'import json,sys; print(json.load(sys.stdin)["bump"])' <<<"$DECISION")
WILL=$(python3 -c 'import json,sys; print("yes" if json.load(sys.stdin)["publish"] else "no")' <<<"$DECISION")
NOTE=$(python3 "$LIB" note-needed --root "$ROOT")
DIRTY=$(git status --porcelain --untracked-files=all -- . ':!tools/kalshi-sdk/upstream_change.json')

if [[ "$MODE" == publish ]]; then
  if [[ "$WILL" == no && "$NOTE" == no && -z "$DIRTY" ]]; then
    notify "spec=${SPEC} gates=ok action=noop version=${VERSION} kind=${KIND} bump=${BUMP}"
    exit 0
  fi
  if [[ "${AUTO_RELEASE:-}" != on && "${AUTO_RELEASE:-}" != off ]]; then
    notify "spec=${SPEC} gates=ok action=hold version=${VERSION} kind=${KIND} bump=${BUMP}"
    exit 2
  fi
fi

snapshot_release_files
RESTORE=yes
if [[ "$WILL" == yes ]]; then
  run_gate version-apply python3 "$LIB" apply --root "$ROOT" --version "$VERSION" --now "$NOW"
else
  run_gate note-apply python3 "$LIB" apply-note --root "$ROOT"
fi
# Check the exact candidate, including the re-rendered generated README. --check
# restores it on exit; a failed gate in either mode restores every changed file.
all_gates -after
run_gate semver-checks-after final_semver
if [[ "$MODE" == check ]]; then
  notify "spec=${SPEC} gates=ok action=check version=${VERSION} kind=${KIND} bump=${BUMP} publish=${WILL}"
  exit 0
fi

BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "${AUTO_RELEASE}" == "off" && "$BRANCH" == "main" ]]; then
  git checkout -b "sync/openapi-${SPEC}" >"$LOGDIR/branch.log" 2>&1
  BRANCH=$(git rev-parse --abbrev-ref HEAD)
fi

commit_release "$VERSION" "$SPEC"
RESTORE=no

if [[ "${AUTO_RELEASE}" == "off" ]]; then
  if ! git push origin "HEAD:${BRANCH}" >"$LOGDIR/git-push.log" 2>&1; then
    fail_gate git-push
  fi
  {
    echo "OpenAPI ${SPEC}"
    echo
    echo "Gates passed. AUTO_RELEASE=off, so this pull request does not publish."
    echo "Selected version: ${VERSION} (${BUMP})."
  } > "$LOGDIR/pr.md"
  if ! gh pr create --repo "$REPO" --base main --head "$BRANCH" \
    --title "Sync OpenAPI ${SPEC}" --body-file "$LOGDIR/pr.md" >"$LOGDIR/gh-pr.log" 2>&1; then
    echo "pull request open failed" > "$LOGDIR/pull-request.log"
    fail_gate pull-request
  fi
  notify "spec=${SPEC} gates=ok action=pr version=${VERSION} kind=${KIND} bump=${BUMP}"
  exit 0
fi

if [[ "$BRANCH" != "main" ]]; then
  if ! git checkout main >"$LOGDIR/checkout-main.log" 2>&1; then
    fail_gate git-merge
  fi
  if ! git merge --ff-only "$BRANCH" >"$LOGDIR/git-merge.log" 2>&1; then
    fail_gate git-merge
  fi
fi
if ! git push origin HEAD:main >"$LOGDIR/git-push.log" 2>&1; then
  fail_gate git-push
fi

if [[ "$WILL" == "yes" ]]; then
  if ! git tag "v${VERSION}" >"$LOGDIR/git-tag.log" 2>&1; then
    fail_gate git-tag
  fi
  if ! git push origin "v${VERSION}" >"$LOGDIR/git-push-tag.log" 2>&1; then
    fail_gate git-push
  fi
  notify "spec=${SPEC} gates=ok action=tag version=${VERSION} kind=${KIND} bump=${BUMP}"
else
  notify "spec=${SPEC} gates=ok action=note version=${VERSION} kind=${KIND} bump=${BUMP}"
fi
exit 0
