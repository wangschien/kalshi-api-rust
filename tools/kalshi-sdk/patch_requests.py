#!/usr/bin/env python3
"""Preserve the Python SDK's media headers and absent optional bodies in Rust."""

import argparse
import re
from pathlib import Path

import yaml


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("spec", type=Path)
    parser.add_argument("crate", type=Path)
    args = parser.parse_args()
    spec = yaml.safe_load(args.spec.read_text())

    def resolve(node):
        while "$ref" in node:
            value = spec
            for key in node["$ref"].removeprefix("#/").split("/"):
                value = value[key]
            node = value
        return node

    operations = {value["operationId"].casefold(): value for path in spec["paths"].values() for method, value in path.items() if method in {"get", "post", "put", "delete", "patch"}}
    count = 0
    for path in sorted((args.crate / "src/apis").glob("*_api.rs")):
        parts = re.split(r"(?=pub async fn )", path.read_text())
        for index, part in enumerate(parts):
            name = re.match(r"pub async fn (\w+)", part)
            if not name:
                continue
            operation = operations[name[1].replace("_", "").casefold()]
            part = re.sub(r'    req_builder = req_builder.header\(reqwest::header::(?:ACCEPT|CONTENT_TYPE), "application/json"\);\n', '', part)
            headers = []
            if any("application/json" in resolve(value).get("content", {}) for value in operation["responses"].values()):
                headers.append('    req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");')
            body = resolve(operation.get("requestBody", {}))
            if "application/json" in body.get("content", {}):
                headers.append('    req_builder = req_builder.header(reqwest::header::CONTENT_TYPE, "application/json");')
                if not body.get("required"):
                    part, replaced = re.subn(r"    req_builder = req_builder.json\(&params\.(\w+)\);", r"    if let Some(ref body) = params.\1 {\n        req_builder = req_builder.json(body);\n    }", part)
                    assert replaced == 1 or "if let Some(ref body) = params." in part, name[1]
            marker = "    if let Some(ref user_agent) = configuration.user_agent {"
            assert part.count(marker) == 1, name[1]
            part = re.sub(r"\n{3,}" + re.escape(marker), "\n\n" + marker, part)
            parts[index] = part.replace(marker, "\n".join(headers) + "\n\n" + marker)
            count += 1
        path.write_text("".join(parts))
    assert count == len(operations), (count, len(operations))
    print(f"request serialization patched for {count} operations")


if __name__ == "__main__":
    main()
