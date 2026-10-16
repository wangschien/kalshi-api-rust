#!/usr/bin/env python3
"""Insert crate-level clippy allows into generated src/lib.rs.

openapi-generator 7.17.0 already emits two crate allows. Rust 1.93 clippy
-D warnings also reports needless_return, derivable_impls, empty_docs, and
into_iter_on_ref on the generated templates. Those allows are inserted here,
next to the generator's own allow attributes, so individual generated files
stay untouched. The step is idempotent.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ANCHOR = "#![allow(clippy::too_many_arguments)]\n"
ALLOWS = (
    "#![allow(clippy::needless_return)]\n",
    "#![allow(clippy::derivable_impls)]\n",
    "#![allow(clippy::empty_docs)]\n",
    "#![allow(clippy::into_iter_on_ref)]\n",
)


def apply(text: str) -> str:
    if ANCHOR not in text:
        raise SystemExit("generated src/lib.rs is missing the clippy::too_many_arguments allow")
    missing = "".join(line for line in ALLOWS if line not in text)
    if not missing:
        return text
    return text.replace(ANCHOR, ANCHOR + missing, 1)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lib_rs", type=Path)
    args = parser.parse_args(argv)
    original = args.lib_rs.read_text()
    updated = apply(original)
    if updated != original:
        args.lib_rs.write_text(updated)
    if apply(updated) != updated:
        raise SystemExit("clippy allow insertion is not idempotent")
    print("clippy allows ok")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
