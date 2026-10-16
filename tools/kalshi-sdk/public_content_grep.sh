#!/usr/bin/env bash
# Fail when tracked-tree text contains a local absolute path, or a term from the
# optional denylist file named by PUBLIC_CONTENT_DENYLIST (one fixed string per
# line, kept outside the repository).
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
cd "$ROOT"

found=0
if grep -RInE --exclude=.git --exclude-dir=.git --exclude-dir=target '/(home|data)/' .; then
  found=1
fi
if [[ -n "${PUBLIC_CONTENT_DENYLIST:-}" ]]; then
  [[ -r "$PUBLIC_CONTENT_DENYLIST" ]] || { echo "denylist not readable" >&2; exit 2; }
  if grep -RInwiF --exclude=.git --exclude-dir=.git --exclude-dir=target -f "$PUBLIC_CONTENT_DENYLIST" .; then
    found=1
  fi
fi
if (( found )); then
  echo "public-content grep found forbidden text" >&2
  exit 1
fi
echo "public-content grep ok"
