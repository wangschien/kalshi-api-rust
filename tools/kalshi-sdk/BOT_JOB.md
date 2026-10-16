# Release job

You are the release worker for this repository. Do this job and then stop.

The scheduler runs `tools/kalshi-sdk/check_upstream.sh` every 10 minutes. That script reads the `source` fields in `tools/kalshi-sdk/pins.toml` (official OpenAPI, official AsyncAPI, and the Python async package index), downloads those three sources, and compares hashes and versions with the pins. It does not call a model. When something differs it writes `tools/kalshi-sdk/upstream_change.json` and starts the program the maintainer configured, with this file as the argument. A lock stops a second run from overlapping. When nothing differs it exits 0.

You may change only this repository. Do not use venue credentials. Do not call trading endpoints. Do not touch any other repository. Do not schedule timers. Do not publish crates yourself. `tools/kalshi-sdk/release.sh` is the only publish entry.

## Path A — mechanical update

Use this path when regeneration and the existing tools accept the new documents.

1. Create branch `sync/openapi-<new OpenAPI info.version>`.
2. Replace `spec/openapi.yaml` and `spec/asyncapi.yaml` with the official documents. Update `tools/kalshi-sdk/pins.toml` so the hashes, info versions, and the Python async pin match `upstream_change.json`. Leave the `source` fields on the official URLs. Recompute the integer overlay: every integer that omits `format` becomes `int64`, and record the new effective hash and the listed entries. Run `tools/kalshi-sdk/regen.sh`.
3. Run `tools/kalshi-sdk/release.sh --check`. Crate versions are independent of spec versions: the script compares the published crates and shared manifest/lock/README/license/toolchain inputs against the latest stable registry version's `v<version>` tag, which must be available locally. It tries `cargo semver-checks` with patch, minor, then major and selects the first passing level for both crates (a breaking change in 0.x bumps minor). It applies the candidate version, re-renders the generated version metadata, then re-runs every gate on that final tree: drift, build, test, clippy, fmt, conformance, derived fields, public content, and semver. `--check` restores the original files on exit. Registry errors, missing tags and semver tool errors refuse; initial publication requires crates.io HTTP 404 for both crate names.
4. The first public release waits until the maintainer has reviewed the README. Until that review, do not publish.
5. When `--check` exits 0 and `AUTO_RELEASE=on`, run `tools/kalshi-sdk/release.sh --publish`. That script runs every gate again. All green commits as wangschien <schienwang@gmail.com>, pushes main, tags, and publishes `kalshi-api-openapi` then `kalshi-api`, with the changelog and the last-synced badge. Any red gate refuses.
6. When `AUTO_RELEASE=off`, run `tools/kalshi-sdk/release.sh --publish`. The script opens a pull request and does not publish.
7. When `AUTO_RELEASE` is unset, stop after `--check` and report that. Do not publish.
8. Report the one-line result.

## Path B — code, overlay, or tooling must change

Use this path when a gate fails because the hand-written client, the overlay, or the tools need a change. A red `release.sh` already opens a GitHub issue and sends the alert.

1. Develop the fix on the sync branch.
2. Open a pull request with the fix and wait for maintainer review.
3. Do not publish before the review approves. After approval, run `tools/kalshi-sdk/release.sh --publish`.

The README promise stays "usually within 1 hour". That is not a guarantee.

## Python SDK lag

When the latest `kalshi_python_async` package targets an older OpenAPI version, conformance still runs against that package. New fields are listed as pending official Python SDK conformance in `CHANGELOG.md`. A later Python release that conforms removes that note and commits the pins/notes without changing the published inputs or publishing a new crate. AsyncAPI-only changes that alter the published inputs do release. The OpenAPI and AsyncAPI versions are recorded in the release CHANGELOG, README generated-from line and pins. A failed re-run is Path B.
