#!/usr/bin/env python3
"""Fail when a type:string enum in an OpenAPI document has a non-string YAML value.

YAML 1.1 treats bare yes/no/on/off/true/false as booleans. openapi-generator then
emits serde rename = "true"/"false" for a field whose wire values are "yes"/"no".
This gate reads the document with PyYAML compose and rejects any string-typed
enum whose members are not tagged as YAML strings.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

import yaml

STR_TAG = "tag:yaml.org,2002:str"


def _escape(key: str) -> str:
    return key.replace("~", "~0").replace("/", "~1")


def _items(node: yaml.Node) -> list[tuple[str, yaml.Node, yaml.Node]] | None:
    if not isinstance(node, yaml.MappingNode):
        return None
    out = []
    for key, value in node.value:
        if not isinstance(key, yaml.ScalarNode):
            raise SystemExit(
                f"non-scalar mapping key at line {key.start_mark.line + 1}"
            )
        out.append((str(key.value), key, value))
    return out


def find_bad_string_enums(root: yaml.Node) -> list[str]:
    bad: list[str] = []

    def walk(node: yaml.Node, pointer: str) -> None:
        items = _items(node)
        if items is None:
            if isinstance(node, yaml.SequenceNode):
                for index, child in enumerate(node.value):
                    walk(child, f"{pointer}/{index}")
            return
        keyed = {name: (key, value) for name, key, value in items}
        type_pair = keyed.get("type")
        enum_pair = keyed.get("enum")
        if (
            type_pair is not None
            and enum_pair is not None
            and "$ref" not in keyed
            and isinstance(type_pair[1], yaml.ScalarNode)
            and str(type_pair[1].value) == "string"
            and isinstance(enum_pair[1], yaml.SequenceNode)
        ):
            for index, child in enumerate(enum_pair[1].value):
                if not isinstance(child, yaml.ScalarNode):
                    bad.append(
                        f"{pointer}/enum/{index}: non-scalar enum member "
                        f"at line {getattr(child.start_mark, 'line', -1) + 1}"
                    )
                elif child.tag != STR_TAG:
                    bad.append(
                        f"{pointer}/enum/{index}: tag {child.tag} value "
                        f"{child.value!r} at line {child.start_mark.line + 1}"
                    )
        for name, _key, value in items:
            walk(value, f"{pointer}/{_escape(name)}")

    walk(root, "")
    return bad


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "document",
        type=Path,
        help="OpenAPI YAML to check (typically the effective overlay output)",
    )
    args = parser.parse_args(argv)
    text = args.document.read_text()
    root = yaml.compose(text)
    if root is None:
        raise SystemExit(f"empty document: {args.document}")
    bad = find_bad_string_enums(root)
    if bad:
        print("string enum gate FAILED:", file=sys.stderr)
        for line in bad:
            print(f"  {line}", file=sys.stderr)
        return 1
    print(f"string enum gate ok: {args.document}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
