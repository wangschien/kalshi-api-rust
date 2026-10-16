#!/usr/bin/env python3
"""Keep JSON numbers as numbers, including integers the Python SDK preserves."""

import argparse
import json
import re
from pathlib import Path



def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("crate", type=Path)
    parser.add_argument("--defaults", type=Path, required=True)
    args = parser.parse_args()
    schemas = {name.casefold(): values for name, values in json.loads(args.defaults.read_text()).items()}
    changed = 0
    for path in (args.crate / "src/models").glob("*.rs"):
        text = path.read_text()
        updated, count = re.subn(r"\bf64\b", "serde_json::Number", text)
        structure = re.search(r"pub struct (\w+) \{(.*?)\n\}", updated, re.S)
        if structure:
            name, body = structure.groups()
            fields = re.findall(r"pub (r#\w+|\w+): ([^\n]+),", body)
            defaults_by_field = {}
            for field, value in schemas.get(name.casefold(), {}).items():
                assert isinstance(value, (bool, int)), (name, field, value)
                literal = str(value).lower()
                defaults_by_field[field] = f"Some({literal})"
                ty = dict(fields)[field]
                helper = f"default_{field}"
                if f"fn {helper}()" not in updated:
                    needle = rf'(    #\[serde\([^\]]*rename = "{field}"[^\]]*)\)\](\s*pub {field}:)'
                    updated, replacements = re.subn(needle, rf'\1, default = "{helper}")]\2', updated)
                    assert replacements == 1, (name, field)
                    updated += f"\nfn {helper}() -> {ty} {{\n    Some({literal})\n}}\n"
                    updated = updated.replace(f"            {field}: None,", f"            {field}: {helper}(),")
            needs_default = any(ty == "serde_json::Number" for _, ty in fields) or defaults_by_field
            if needs_default and f"impl Default for {name}" not in updated:
                updated = updated.replace("Clone, Default, Debug", "Clone, Debug")
                defaults = "\n".join(f"            {field}: " + defaults_by_field.get(field, "0.into()" if ty == "serde_json::Number" else "Default::default()") + "," for field, ty in fields)
                updated += f"\nimpl Default for {name} {{\n    fn default() -> Self {{\n        Self {{\n{defaults}\n        }}\n    }}\n}}\n"
        if updated != text:
            path.write_text(updated)
            changed += count
    manifest = args.crate / "Cargo.toml"
    text, count = re.subn(r'^serde_json = .+$', 'serde_json = { version = "^1.0", features = ["arbitrary_precision"] }', manifest.read_text(), flags=re.M)
    assert count == 1
    manifest.write_text(text)
    print(f"JSON number types preserved at {changed} model sites")


if __name__ == "__main__":
    main()
