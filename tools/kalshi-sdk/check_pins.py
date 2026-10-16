#!/usr/bin/env python3
"""Check pinned input hashes and the overlay output hash."""

from __future__ import annotations

import argparse
import hashlib
import sys
import tomllib
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require(path: Path, expected: str, label: str) -> None:
    actual = sha256(path)
    if actual != expected:
        raise SystemExit(f"{label} sha256 mismatch for {path}: {actual}")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--jar", type=Path)
    parser.add_argument("--effective", type=Path)
    args = parser.parse_args(argv)
    pins = tomllib.loads((args.root / "tools/kalshi-sdk/pins.toml").read_text())
    require(args.root / pins["openapi"]["path"], pins["openapi"]["sha256"], "openapi")
    require(args.root / pins["asyncapi"]["path"], pins["asyncapi"]["sha256"], "asyncapi")
    if "defaults_path" in pins["python_sdk"]:
        require(args.root / pins["python_sdk"]["defaults_path"], pins["python_sdk"]["defaults_sha256"], "Python model defaults")
    if args.jar is not None:
        require(args.jar, pins["generator"]["jar_sha256"], "generator jar")
    if args.effective is not None:
        require(args.effective, pins["overlay"]["effective_openapi_sha256"], "effective openapi")
    print("pins ok")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
