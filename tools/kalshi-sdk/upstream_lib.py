#!/usr/bin/env python3
"""Hash and version checks, and the release version decision.

The shell entry points own downloads, gates, and publishing.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tomllib
import zipfile
from datetime import datetime, timezone
from pathlib import Path


PENDING_PREFIX = "- Pending official Python SDK conformance:"
CRATES = ("kalshi-api-openapi", "kalshi-api", "kalshi-api-pins")
PUBLISHED = ("kalshi-api-openapi", "kalshi-api")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_pins(path: Path) -> dict:
    return tomllib.loads(path.read_text())


def pins_of(root: Path) -> dict:
    return load_pins(root / "tools" / "kalshi-sdk" / "pins.toml")


def source_url(table: dict, label: str) -> str:
    value = str(table.get("source", "")).strip()
    if not (value.startswith("https://") or value.startswith("file://")):
        raise SystemExit(f"{label} source is missing")
    return value


def source_urls(pins: dict, include_sync: bool) -> list[str]:
    urls = [
        source_url(pins["openapi"], "openapi"),
        source_url(pins["asyncapi"], "asyncapi"),
        source_url(pins["python_sdk"], "python async"),
    ]
    if include_sync:
        urls.append(source_url(pins["python_sdk_sync"], "python sync"))
    return urls


def info_version(text: str) -> str:
    """Return the top-level info.version value from an OpenAPI or AsyncAPI document."""
    info_indent = None
    for line in text.splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        indent = len(line) - len(line.lstrip(" "))
        if info_indent is None:
            if line.startswith("info:"):
                info_indent = 0
            continue
        if indent <= info_indent:
            break
        stripped = line.strip()
        if stripped.startswith("version:"):
            value = stripped.split(":", 1)[1].strip().strip("'\"")
            if not value:
                break
            return value
    raise SystemExit("info.version is missing")


def parse_version(value: str) -> tuple[int, int, int]:
    parts = value.split(".")
    if len(parts) != 3:
        raise SystemExit(f"not a major.minor.patch version: {value}")
    numbers = []
    for part in parts:
        if not part.isdigit() or str(int(part)) != part:
            raise SystemExit(f"not a major.minor.patch version: {value}")
        numbers.append(int(part))
    return numbers[0], numbers[1], numbers[2]


def version_text(value: str) -> str:
    major, minor, patch = parse_version(value)
    return f"{major}.{minor}.{patch}"


def package_version(manifest: Path) -> str:
    current = None
    for line in manifest.read_text().splitlines():
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            current = stripped[1:-1].strip()
            continue
        if current == "package" and _key_is(stripped, "version"):
            return _quoted(stripped)
    raise SystemExit(f"package version is missing in {manifest}")


def _key_is(stripped: str, key: str) -> bool:
    if not stripped.startswith(key):
        return False
    rest = stripped[len(key) :]
    return rest[:1] in ("", " ", "=")


def _quoted(stripped: str) -> str:
    value = stripped.split("=", 1)[1].strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in "'\"":
        return value[1:-1]
    raise SystemExit(f"expected a quoted value in {stripped}")


def wheel_choice(document: dict) -> tuple[str, str, str]:
    version = str(document["info"]["version"])
    version_text(version)
    wheels = [item for item in document.get("urls", []) if item.get("packagetype") == "bdist_wheel"]
    preferred = [
        item
        for item in wheels
        if str(item.get("filename", "")).endswith("py3-none-any.whl")
        and version in str(item.get("filename", ""))
    ]
    chosen = preferred[0] if preferred else (wheels[0] if len(wheels) == 1 else None)
    if chosen is None:
        raise SystemExit("index document has no py3-none-any wheel")
    digest = str(chosen.get("digests", {}).get("sha256", ""))
    url = str(chosen.get("url", ""))
    if not digest or not url:
        raise SystemExit("index document is missing a wheel url or sha256")
    return version, url, digest


def _source(old_version: str, new_version: str, old_sha: str, new_sha: str) -> dict:
    return {
        "changed": old_version != new_version or old_sha != new_sha,
        "old_version": old_version,
        "new_version": new_version,
        "old_sha256": old_sha,
        "new_sha256": new_sha,
    }


def compare_upstream(pins: dict, openapi: Path, asyncapi: Path, index: dict, wheel: Path) -> dict:
    wheel_sha = sha256(wheel)
    py_version, _url, digest = wheel_choice(index)
    if wheel_sha != digest:
        raise SystemExit("wheel bytes do not match the index sha256")
    openapi_text = openapi.read_text()
    asyncapi_text = asyncapi.read_text()
    sources = {
        "openapi": _source(
            pins["openapi"]["info_version"],
            info_version(openapi_text),
            pins["openapi"]["sha256"],
            sha256(openapi),
        ),
        "asyncapi": _source(
            pins["asyncapi"]["info_version"],
            info_version(asyncapi_text),
            pins["asyncapi"]["sha256"],
            sha256(asyncapi),
        ),
        "python_async": _source(
            pins["python_sdk"]["version"],
            py_version,
            pins["python_sdk"]["wheel_sha256"],
            wheel_sha,
        ),
    }
    changed = [name for name, row in sources.items() if row["changed"]]
    return {"changed": bool(changed), "sources": sources}


def local_pin(root: Path) -> None:
    pins = pins_of(root)
    _match_document(root, pins, "openapi")
    _match_document(root, pins, "asyncapi")


def _match_document(root: Path, pins: dict, name: str) -> None:
    path = root / pins[name]["path"]
    if sha256(path) != pins[name]["sha256"]:
        raise SystemExit(f"{name} sha256 does not match pins")
    if info_version(path.read_text()) != pins[name]["info_version"]:
        raise SystemExit(f"{name} info.version does not match pins")


def upstream_pin(
    root: Path,
    openapi: Path,
    asyncapi: Path,
    async_index: dict,
    async_wheel: Path,
    sync_index: dict,
    sync_wheel: Path,
) -> None:
    local_pin(root)
    pins = pins_of(root)
    report = compare_upstream(pins, openapi, asyncapi, async_index, async_wheel)
    if report["changed"]:
        raise SystemExit("downloaded OpenAPI, AsyncAPI, or Python async pin does not match")
    if sha256(openapi) != sha256(root / pins["openapi"]["path"]):
        raise SystemExit("downloaded OpenAPI does not match the working tree")
    if sha256(asyncapi) != sha256(root / pins["asyncapi"]["path"]):
        raise SystemExit("downloaded AsyncAPI does not match the working tree")
    sync_version, _url, sync_digest = wheel_choice(sync_index)
    sync_sha = sha256(sync_wheel)
    if sync_sha != sync_digest:
        raise SystemExit("sync wheel bytes do not match the index sha256")
    pinned = pins["python_sdk_sync"]
    if sync_version != pinned["version"] or sync_sha != pinned["wheel_sha256"]:
        raise SystemExit("downloaded Python sync pin does not match")


def aligned(root: Path) -> str:
    versions = [package_version(root / "crates" / name / "Cargo.toml") for name in CRATES]
    if len(set(versions)) != 1:
        raise SystemExit(f"crate versions differ: {versions}")
    return versions[0]


def publishable(root: Path) -> None:
    for name in PUBLISHED:
        in_package = False
        for line in (root / "crates" / name / "Cargo.toml").read_text().splitlines():
            stripped = line.strip()
            if stripped.startswith("["):
                in_package = stripped == "[package]"
                continue
            if in_package and re.match(r"publish\s*=\s*false\s*$", stripped):
                raise SystemExit(f"{name} has publish = false")


def breaking_floor(current: str) -> str:
    major, minor, _patch = parse_version(current)
    if major == 0:
        return f"0.{minor + 1}.0"
    return f"{major + 1}.0.0"


def decide(current: str, kind: str, changed: bool) -> dict:
    current = version_text(current)
    if kind not in ("patch", "minor", "major", "initial"):
        raise SystemExit(f"unknown semver kind: {kind}")
    if not changed and kind != "initial":
        return {"version": current, "publish": False, "bump": "none"}
    major, minor, patch = parse_version(current)
    if kind == "initial":
        return {"version": current, "publish": True, "bump": "initial"}
    # --release-type is the assumed bump. Cargo treats a 0.y change of z as
    # minor and a change of y as major, so a minor result on 0.y increments z.
    if kind == "major":
        chosen = breaking_floor(current)
        bump = "minor" if major == 0 else "major"
    elif kind == "minor" and major > 0:
        chosen, bump = f"{major}.{minor + 1}.0", "minor"
    else:
        chosen, bump = f"{major}.{minor}.{patch + 1}", "patch"
    return {"version": chosen, "publish": True, "bump": bump}


def registry_version(document: dict, name: str) -> str:
    if document["crate"]["id"] != name:
        raise SystemExit("registry returned a different crate")
    # Use the latest stable release, including yanked versions. An unusable
    # baseline must refuse; it must never become permission to publish initially.
    versions = [row["num"] for row in document["versions"]
                if re.fullmatch(r"\d+\.\d+\.\d+", row["num"])]
    if not versions:
        raise SystemExit("registry has no stable baseline")
    return max(versions, key=parse_version)


def published_changed(root: Path, baseline: str, current: str) -> bool:
    version_text(baseline)
    if parse_version(current) < parse_version(baseline):
        raise SystemExit("worktree version is older than the registry baseline")
    tag = f"refs/tags/v{baseline}"
    def git(*args: str) -> bytes:
        return subprocess.check_output(["git", "-C", str(root), *args])
    git("rev-parse", "--verify", f"{tag}^{{commit}}")
    for name in PUBLISHED:
        manifest = tomllib.loads(git("show", f"{tag}:crates/{name}/Cargo.toml").decode())
        if manifest["package"]["version"] != baseline:
            raise SystemExit("published tag has inconsistent package versions")
    # The release inputs: both published crates (including their manifests and
    # READMEs) and shared manifest/lock/README/license/toolchain files. Pins,
    # specs, tooling and conformance-only CHANGELOG notes are not release inputs.
    paths = [*(f"crates/{name}" for name in PUBLISHED), "Cargo.toml",
             "Cargo.lock", "README.md", "LICENSE", "rust-toolchain.toml"]
    return bool(git("diff", "--name-only", tag, "--", *paths) or
                git("ls-files", "--others", "--exclude-standard", "--", *paths))


def pending_line(python_openapi: str, spec: str) -> str | None:
    if python_openapi == spec:
        return None
    return (
        f"{PENDING_PREFIX} Python OpenAPI {python_openapi}, spec {spec}."
    )


def note_needs_edit(changelog: str, line: str | None) -> bool:
    rows = [item for item in changelog.splitlines() if item.startswith(PENDING_PREFIX)]
    if line is None:
        return bool(rows)
    return rows != [line]


def shields_badge(spec: str, when: datetime) -> str:
    stamp = when.astimezone(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    message = f"OpenAPI {spec} | {stamp}"
    message = message.replace("-", "--").replace(" ", "%20").replace("|", "%7C").replace(":", "%3A")
    return f"![last synced](https://img.shields.io/badge/last%20synced-{message}-blue)"


def _set_section_key(text: str, section: str, key: str, rendered: str) -> str:
    current = None
    found = False
    out = []
    for line in text.splitlines(keepends=True):
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]") and not stripped.startswith("[["):
            current = stripped[1:-1].strip()
        if current == section and _key_is(stripped, key) and not found:
            newline = "\n" if line.endswith("\n") else ""
            out.append(f"{key} = {rendered}{newline}")
            found = True
            continue
        out.append(line)
    if not found:
        raise SystemExit(f"missing [{section}] {key}")
    return "".join(out)


def _set_package_version(text: str, version: str) -> str:
    return _set_section_key(text, "package", "version", f'"{version}"')


def _set_path_dep_version(text: str, dep: str, version: str) -> str:
    pattern = rf'({re.escape(dep)} = \{{[^}}]*version = ")[^"]+'
    new, count = re.subn(pattern, rf"\g<1>{version}", text, count=1)
    if count == 0:
        return text
    return new


def _set_lock(text: str, version: str) -> str:
    parts = text.split("[[package]]")
    found = set()
    rebuilt = [parts[0]]
    for part in parts[1:]:
        lines = part.splitlines(keepends=True)
        name = None
        replaced = False
        out = []
        for line in lines:
            if name is None and line.startswith("name = "):
                name = line.split("=", 1)[1].strip().strip('"')
            if name in CRATES and not replaced and line.startswith("version = "):
                out.append(f'version = "{version}"\n')
                replaced = True
                found.add(name)
                continue
            out.append(line)
        rebuilt.append("".join(out))
    missing = set(CRATES) - found
    if missing:
        raise SystemExit(f"Cargo.lock is missing {sorted(missing)}")
    return "[[package]]".join(rebuilt)


def _replace_generator_version(text: str, old: str, new: str) -> str:
    needle = f"packageVersion={old}"
    if needle not in text:
        raise SystemExit(f"missing {needle}")
    return text.replace(needle, f"packageVersion={new}")


def _changelog(text: str, version: str, day: str, bullets: list[str]) -> str:
    kept = [line for line in text.splitlines() if not line.startswith(PENDING_PREFIX)]
    heading = f"## [{version}]"
    section_lines = [heading + f" - {day}", ""] + bullets + [""]
    without = _drop_section(kept, heading)
    if not without or without[0].strip() != "# Changelog":
        raise SystemExit("CHANGELOG.md does not start with # Changelog")
    insert_at = 1
    if len(without) > 1 and without[1] == "":
        insert_at = 2
    updated = without[:insert_at] + section_lines + without[insert_at:]
    rendered = "\n".join(updated)
    if text.endswith("\n"):
        rendered += "\n"
    return rendered


def _section_heading(line: str, heading: str) -> bool:
    return line == heading or line.startswith(heading + " ")


def _drop_section(lines: list[str], heading: str) -> list[str]:
    out = []
    skipping = False
    for line in lines:
        if line.startswith("## ["):
            skipping = _section_heading(line, heading)
        if skipping:
            continue
        out.append(line)
    return out


def _badge_line(text: str, badge: str) -> str:
    pattern = re.compile(r"!\[last synced\]\(https://img\.shields\.io/badge/last%20synced-[^)]*\)")
    if not pattern.search(text):
        return text
    return pattern.sub(lambda _match: badge, text, count=1)


def apply_release(root: Path, version: str, now: datetime) -> None:
    version = version_text(version)
    pins = pins_of(root)
    local_pin(root)
    spec = pins["openapi"]["info_version"]
    old = aligned(root)
    day = now.astimezone(timezone.utc).strftime("%Y-%m-%d")
    bullets = [
        f"- OpenAPI {spec} sha256 {pins['openapi']['sha256']}.",
        f"- AsyncAPI {pins['asyncapi']['info_version']} sha256 {pins['asyncapi']['sha256']}.",
        f"- kalshi_python_async {pins['python_sdk']['version']} sha256 {pins['python_sdk']['wheel_sha256']}.",
    ]
    pending = pending_line(pins["python_sdk"]["openapi_info_version"], spec)
    if pending is not None:
        bullets.append(pending)
    for name in CRATES:
        path = root / "crates" / name / "Cargo.toml"
        text = _set_package_version(path.read_text(), version)
        text = _set_path_dep_version(text, "kalshi-api-openapi", version)
        path.write_text(text)
    lock = root / "Cargo.lock"
    lock.write_text(_set_lock(lock.read_text(), version))
    pins_path = root / "tools" / "kalshi-sdk" / "pins.toml"
    pins_text = _set_section_key(pins_path.read_text(), "generator", "package_version", f'"{version}"')
    if old != version:
        pins_text = _replace_generator_version(pins_text, old, version)
    pins_path.write_text(pins_text)
    generate = root / "tools" / "kalshi-sdk" / "generate.sh"
    generate_text = generate.read_text()
    if old != version:
        generate_text = _replace_generator_version(generate_text, old, version)
    generate.write_text(generate_text)
    # The pinned generator embeds packageVersion only in its manifest and this
    # README line. Re-render both, then verify with the real generator drift gate.
    generated_readme = root / "crates" / "kalshi-api-openapi" / "README.md"
    text, count = re.subn(r"(?m)^- Package version: " + re.escape(old) + r"$",
                          f"- Package version: {version}", generated_readme.read_text())
    if count != 1:
        raise SystemExit("generated README package version is missing or inconsistent")
    generated_readme.write_text(text)
    changelog = root / "CHANGELOG.md"
    changelog.write_text(_changelog(changelog.read_text(), version, day, bullets))
    readme = root / "README.md"
    text = _badge_line(readme.read_text(), shields_badge(spec, now))
    text, count = re.subn(
        r"(?m)^Generated from OpenAPI .+ and AsyncAPI .+\.$",
        f"Generated from OpenAPI {spec} and AsyncAPI {pins['asyncapi']['info_version']}.",
        text,
    )
    if count > 1:
        raise SystemExit("README generated-from line is repeated")
    text, count = re.subn(rf"(?m)^- Crate version: {re.escape(old)}$", f"- Crate version: {version}", text)
    if count > 1:
        raise SystemExit("README crate version line is repeated")
    api_version = info_version((root / "spec" / "openapi.yaml").read_text())
    api_lines = re.findall(r"(?m)^- API version: .+$", text)
    if len(api_lines) > 1:
        raise SystemExit("README API version line is repeated")
    if len(api_lines) == 1:
        text = re.sub(r"(?m)^- API version: .+$", f"- API version: {api_version}", text, count=1)
    text = re.sub(
        r"official OpenAPI document, info version [0-9.]+(?=\. API)",
        f"official OpenAPI document, info version {spec}",
        text,
    )
    readme.write_text(text)


def apply_note(root: Path) -> None:
    pins = pins_of(root)
    pending = pending_line(pins["python_sdk"]["openapi_info_version"], pins["openapi"]["info_version"])
    path = root / "CHANGELOG.md"
    text = "\n".join(line for line in path.read_text().splitlines()
                     if not line.startswith(PENDING_PREFIX)) + "\n"
    if pending is not None:
        if "## [Unreleased]" not in text:
            raise SystemExit("CHANGELOG.md is missing Unreleased")
        text = text.replace("## [Unreleased]", f"## [Unreleased]\n\n{pending}", 1)
    path.write_text(text)


def unpack_package(wheel: Path, dest: Path, name: str) -> Path:
    dest.mkdir(parents=True, exist_ok=True)
    root = dest.resolve()
    with zipfile.ZipFile(wheel) as archive:
        for info in archive.infolist():
            target = (root / info.filename).resolve()
            if target != root and root not in target.parents:
                raise SystemExit(f"wheel path escapes the destination: {info.filename}")
            if info.is_dir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            with archive.open(info) as source, target.open("wb") as sink:
                sink.write(source.read())
    package = root / name
    if not package.is_dir():
        raise SystemExit(f"{name} is missing from {wheel}")
    return package


def _state_key(report: dict) -> str:
    sources = report["sources"]
    return ":".join((
        sources["openapi"]["new_sha256"],
        sources["asyncapi"]["new_sha256"],
        sources["python_async"]["new_version"],
    ))


def _corrupt_state(path: Path) -> None:
    raise SystemExit(
        f"upstream state file {path} is corrupt and was left in place. "
        "Inspect the published crates and the git tags, then repair or delete this file. "
        "The check stops until that is done."
    )


def _load_state(path: Path) -> dict:
    if not path.is_file():
        return {"keys": {}}
    text = path.read_text()
    try:
        document = json.loads(text)
    except json.JSONDecodeError:
        _corrupt_state(path)
    keys = document.get("keys") if isinstance(document, dict) else None
    if not isinstance(keys, dict):
        _corrupt_state(path)
    return document


def _save_state(path: Path, document: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    with temporary.open("w", encoding="utf-8") as handle:
        handle.write(json.dumps(document, indent=2) + "\n")
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


def upstream_state(path: Path, report_path: Path, now: int, pid: int, event: str) -> str:
    """Remember one upstream key. begin returns launch, done, or failed."""
    del pid
    report = json.loads(report_path.read_text())
    key = _state_key(report)
    document = _load_state(path)
    row = document.setdefault("keys", {}).get(key, {})
    if event == "ok":
        document["keys"][key] = {"status": "done", "failures": int(row.get("failures", 0)), "alerted": int(row.get("alerted", 0))}
        _save_state(path, document)
        return "done"
    if event == "fail":
        document["keys"][key] = {
            "status": "failed",
            "failures": int(row.get("failures", 0)) + 1,
            "alerted": int(row.get("alerted", 0)),
        }
        _save_state(path, document)
        return "failed"
    if event != "begin":
        raise SystemExit(f"unknown upstream state event: {event}")
    if row.get("status") == "done":
        return "done"
    failures = int(row.get("failures", 0))
    if failures >= 3:
        alerted = int(row.get("alerted", 0))
        if now - alerted >= 24 * 60 * 60:
            row["alerted"] = now
            document["keys"][key] = row
            _save_state(path, document)
            return "failed"
        return "quiet"
    return "launch"


def _parse_now(value: str) -> datetime:
    try:
        parsed = datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError as exc:
        raise SystemExit("KALSHI_RELEASE_NOW must be YYYY-MM-DDTHH:MM:SSZ") from exc
    return parsed.replace(tzinfo=timezone.utc)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="cmd", required=True)

    spec = sub.add_parser("spec-version")
    spec.add_argument("--root", type=Path, required=True)

    manifest = sub.add_parser("package-version")
    manifest.add_argument("--manifest", type=Path, required=True)

    for name in ("aligned", "publishable", "local-pin"):
        item = sub.add_parser(name)
        item.add_argument("--root", type=Path, required=True)

    wheel = sub.add_parser("wheel-url")
    wheel.add_argument("--pypi", type=Path, required=True)

    sources = sub.add_parser("sources")
    sources.add_argument("--pins", type=Path, required=True)
    sources.add_argument("--sync", action="store_true")

    check = sub.add_parser("check")
    check.add_argument("--pins", type=Path, required=True)
    check.add_argument("--openapi", type=Path, required=True)
    check.add_argument("--asyncapi", type=Path, required=True)
    check.add_argument("--pypi", type=Path, required=True)
    check.add_argument("--wheel", type=Path, required=True)
    check.add_argument("--out", type=Path, required=True)

    upstream = sub.add_parser("upstream-pin")
    upstream.add_argument("--root", type=Path, required=True)
    upstream.add_argument("--openapi", type=Path, required=True)
    upstream.add_argument("--asyncapi", type=Path, required=True)
    upstream.add_argument("--async-pypi", type=Path, required=True)
    upstream.add_argument("--async-wheel", type=Path, required=True)
    upstream.add_argument("--sync-pypi", type=Path, required=True)
    upstream.add_argument("--sync-wheel", type=Path, required=True)

    decision = sub.add_parser("decide")
    decision.add_argument("--current", required=True)
    decision.add_argument("--kind", required=True)
    decision.add_argument("--changed", choices=("yes", "no"), required=True)

    registry = sub.add_parser("registry-version")
    registry.add_argument("--json", type=Path, required=True)
    registry.add_argument("--name", required=True)

    changed = sub.add_parser("published-changed")
    changed.add_argument("--root", type=Path, required=True)
    changed.add_argument("--baseline", required=True)
    changed.add_argument("--current", required=True)

    note_apply = sub.add_parser("apply-note")
    note_apply.add_argument("--root", type=Path, required=True)

    note = sub.add_parser("note-needed")
    note.add_argument("--root", type=Path, required=True)

    release = sub.add_parser("apply")
    release.add_argument("--root", type=Path, required=True)
    release.add_argument("--version", required=True)
    release.add_argument("--now", required=True)

    unpacked = sub.add_parser("unpack")
    unpacked.add_argument("--wheel", type=Path, required=True)
    unpacked.add_argument("--dest", type=Path, required=True)
    unpacked.add_argument("--name", required=True)

    badge = sub.add_parser("badge")
    badge.add_argument("--spec", required=True)
    badge.add_argument("--now", required=True)

    sub.add_parser("state-check").add_argument("--state", type=Path, required=True)

    state = sub.add_parser("upstream-state")
    state.add_argument("--state", type=Path, required=True)
    state.add_argument("--report", type=Path, required=True)
    state.add_argument("--now", type=int, required=True)
    state.add_argument("--pid", type=int, required=True)
    state.add_argument("--event", choices=("begin", "ok", "fail"), required=True)

    args = parser.parse_args(argv)
    if args.cmd == "spec-version":
        print(info_version((args.root / "spec" / "openapi.yaml").read_text()))
        return 0
    if args.cmd == "package-version":
        print(package_version(args.manifest))
        return 0
    if args.cmd == "aligned":
        print(aligned(args.root))
        return 0
    if args.cmd == "publishable":
        publishable(args.root)
        print("publishable ok")
        return 0
    if args.cmd == "local-pin":
        local_pin(args.root)
        print("local pin ok")
        return 0
    if args.cmd == "sources":
        for url in source_urls(load_pins(args.pins), args.sync):
            print(url)
        return 0
    if args.cmd == "wheel-url":
        _version, url, _digest = wheel_choice(json.loads(args.pypi.read_text()))
        print(url)
        return 0
    if args.cmd == "check":
        report = compare_upstream(
            load_pins(args.pins),
            args.openapi,
            args.asyncapi,
            json.loads(args.pypi.read_text()),
            args.wheel,
        )
        if report["changed"]:
            args.out.parent.mkdir(parents=True, exist_ok=True)
            args.out.write_text(json.dumps(report, indent=2) + "\n")
            return 10
        return 0
    if args.cmd == "upstream-pin":
        upstream_pin(
            args.root,
            args.openapi,
            args.asyncapi,
            json.loads(args.async_pypi.read_text()),
            args.async_wheel,
            json.loads(args.sync_pypi.read_text()),
            args.sync_wheel,
        )
        print("upstream pin ok")
        return 0
    if args.cmd == "decide":
        print(json.dumps(decide(args.current, args.kind, args.changed == "yes")))
        return 0
    if args.cmd == "registry-version":
        print(registry_version(json.loads(args.json.read_text()), args.name))
        return 0
    if args.cmd == "published-changed":
        print("yes" if published_changed(args.root, args.baseline, args.current) else "no")
        return 0
    if args.cmd == "apply-note":
        apply_note(args.root)
        return 0
    if args.cmd == "note-needed":
        root_pins = pins_of(args.root)
        line = pending_line(
            root_pins["python_sdk"]["openapi_info_version"],
            root_pins["openapi"]["info_version"],
        )
        needed = note_needs_edit((args.root / "CHANGELOG.md").read_text(), line)
        print("yes" if needed else "no")
        return 0
    if args.cmd == "apply":
        apply_release(args.root, args.version, _parse_now(args.now))
        print("apply ok")
        return 0
    if args.cmd == "unpack":
        print(unpack_package(args.wheel, args.dest, args.name))
        return 0
    if args.cmd == "badge":
        print(shields_badge(args.spec, _parse_now(args.now)))
        return 0
    if args.cmd == "state-check":
        _load_state(args.state)
        return 0
    if args.cmd == "upstream-state":
        print(upstream_state(args.state, args.report, args.now, args.pid, args.event))
        return 0
    raise SystemExit(f"unknown command {args.cmd}")


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
