#!/usr/bin/env python3
"""Drop generator files that must not be committed, and record that in the ignore file."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

DROPPED = ("git_push.sh", ".travis.yml")
IGNORE_LINES = ["git_push.sh", ".travis.yml"]


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("crate", type=Path)
    args = parser.parse_args(argv)
    ignore = args.crate / ".openapi-generator-ignore"
    text = ignore.read_text()
    extra = [line for line in IGNORE_LINES if line not in text.splitlines()]
    if extra:
        if not text.endswith("\n"):
            text += "\n"
        text += "\n".join(extra) + "\n"
        ignore.write_text(text)
    for name in DROPPED:
        path = args.crate / name
        if path.exists():
            path.unlink()
    print("generated extras trimmed")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
