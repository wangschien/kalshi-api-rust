#!/usr/bin/env python3
"""Fail if a generator log contains a warning outside the known name classes."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

PATTERNS = (
    re.compile(r"^ApplicationJson cannot be used as a model name\. Renamed to ApplicationSlashJson$"),
    re.compile(r"^type cannot be used as a field/variable name\. Renamed to r#type$"),
    re.compile(r"^type cannot be used as a parameter name\. Renamed to r#type$"),
    re.compile(r"^serde_json::value cannot be used as a field/variable name\. Renamed to serde_json_colon_colon_value$"),
    re.compile(r"^.+ cannot be used as a enum variant name\. Renamed to .+$"),
    re.compile(r"^Failed to get the schema name: null$"),
)
WARN_RE = re.compile(r"\bWARN\b\s+\S+\s+-\s+(.*)$")
ERROR_RE = re.compile(r"\bERROR\b")


def check_lines(lines: list[str]) -> list[str]:
    problems = []
    for number, line in enumerate(lines, start=1):
        if ERROR_RE.search(line):
            problems.append(f"{number}: error: {line.rstrip()}")
            continue
        match = WARN_RE.search(line)
        if not match:
            continue
        message = match.group(1).strip()
        if not any(pattern.fullmatch(message) for pattern in PATTERNS):
            problems.append(f"{number}: new warning class: {message}")
    return problems


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path, nargs="?")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args(argv)
    if args.self_test:
        accepted = check_lines(
            [
                "[main] WARN  o.o.c.languages.AbstractRustCodegen - type cannot be used as a field/variable name. Renamed to r#type",
                "[main] WARN  o.o.c.languages.AbstractRustCodegen - Self cannot be used as a enum variant name. Renamed to VariantSelf",
                "[main] INFO  o.o.codegen.DefaultGenerator - Generator 'rust' is considered stable.",
            ]
        )
        rejected = check_lines(
            [
                "[main] WARN  o.o.codegen.utils.ModelUtils - something new",
                "[main] ERROR o.o.codegen.DefaultGenerator - boom",
            ]
        )
        if accepted or len(rejected) != 2:
            raise SystemExit(f"self-test failed: {accepted!r} {rejected!r}")
        print("warning whitelist self-test ok")
        return 0
    if args.log is None:
        raise SystemExit("log path is required")
    problems = check_lines(args.log.read_text(errors="replace").splitlines())
    if problems:
        print("\n".join(problems))
        return 1
    print("warning whitelist ok")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
