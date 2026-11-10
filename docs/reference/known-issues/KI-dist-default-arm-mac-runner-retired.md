---
upstream: https://github.com/axodotdev/cargo-dist/issues
affects: the default GitHub runner of cargo-dist 0.32.0 for aarch64-apple-darwin
state: masked
filing: deferred
workaround: dist-workspace.toml names macos-15 for aarch64-apple-darwin in [dist.github-custom-runners]
retire_when: the cargo-dist version that dist-workspace.toml pins builds aarch64-apple-darwin on a label that GitHub serves to public repositories, so the custom runner entry can go
checked: 2026-11-10
---

# cargo-dist 0.32.0 builds aarch64-apple-darwin on a runner that GitHub retired

## Symptom

The Release workflow of a tag ends as cancelled. The job `build-local-artifacts (aarch64-apple-darwin)` stops after about two seconds, and the jobs `build-global-artifacts`, `host`, and `announce` are skipped. The crate is on crates.io, but the GitHub release of the tag has no binaries and no installers. This happened to v0.6.7 (run 42210571427, 2026-11-06) and v0.6.8 (run 42840974253, 2026-11-10).

## How it works

1. release-plz publishes the crate and pushes the tag, for example `v0.6.8`.
2. The tag push starts `.github/workflows/release.yml`. Its `plan` job runs `dist host --steps=create --tag=v0.6.8` on the tagged commit. dist reads `dist-workspace.toml` of that commit and writes the build matrix, one runner label per target.
3. With no entry for the target in `[dist.github-custom-runners]`, cargo-dist 0.32.0 gives aarch64-apple-darwin its default label, `macos-14`.
4. GitHub no longer serves `macos-14` to public repositories. The job gets the annotation "The configuration 'macos-14-arm64-us-default' is not supported", and GitHub cancels it.
5. The `host` job needs every build job, so it is skipped, and dist never creates the GitHub release.

Cause: the default runner label of the pinned cargo-dist names a runner image that GitHub retired. A first reading blames the workflow file, but `release.yml` holds no runner label. It takes the labels from `dist plan` at run time, so `dist generate` cannot fix it. The smallest fix is a custom runner for the one target.

A rerun of the Release workflow for an old tag fails the same way, because it reads `dist-workspace.toml` of the tagged commit. v0.6.7 and v0.6.8 stay crate-only, and the next release carries binaries again.

## Signal

A cancelled `build-local-artifacts (aarch64-apple-darwin)` job whose annotation names a runner configuration that "is not supported". `dist plan --output-format=json` lists a runner label for a target that GitHub's runner reference no longer lists.

## Workaround

`dist-workspace.toml` sets `aarch64-apple-darwin = "macos-15"` under `[dist.github-custom-runners]`, with a comment that cites this case. `dist plan` then builds every macOS target on a label that GitHub lists: `macos-15` for arm64 and `macos-15-intel` for x86_64.
