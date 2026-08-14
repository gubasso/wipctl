# Publishing a release

How a version of `wipctl` reaches its package registry and its binary artifacts. The routine path is automated and the only human decision is merging a release pull request; the manual paths below exist for a first publish and for the days the automation is unavailable.

The gates every release must already pass are [reference/quality-gates.md](../reference/quality-gates.md). This guide covers only the distribution step that follows them.

## The routine path

1. Merge feature work into the default branch with commit subjects that follow Conventional Commits, since the version bump and the changelog are derived from them.
2. The release automation opens or refreshes a release pull request carrying the version bump, `CHANGELOG.md`, `Cargo.toml`, and `Cargo.lock`.
3. Review that pull request and merge it.
4. Merging tags the release, publishes the crate, and builds the binary artifacts.

The tag is never created by hand. Creating one manually desynchronizes the tag from the version the automation believes it published.

## Registry authentication

Publishing from CI authenticates by short-lived OIDC token, minted by the release job itself from its `id-token: write` permission. No long-lived registry token is stored as a repository secret.

The first publish cannot use that path, because a trusted publisher can only be registered against a crate that already exists. Once, and only once:

1. Mint a registry token scoped to the `publish-new` endpoint, scoped to this crate alone, with the shortest available expiry.
2. Run `./scripts/publish-dry` to confirm the package builds and ships the intended files.
3. Authenticate with `cargo login`, then run `./scripts/publish`.
4. Register the trusted publisher against this repository and the release workflow's filename. The registration matches on the filename, so it must name the publishing workflow and not the binary-artifact workflow.
5. Revoke the token from step 1.

The auth check lives inside `scripts/publish` and nowhere else. It confirms that authentication is configured, never that a credential is valid, and it never reads, echoes, or logs a credential.

## Readiness checks

`./scripts/publish-dry` runs the registry dry run and prints the exact file list the package would ship. Neither needs authentication, so it is safe to run at any point.

Read that file list. The package should carry build inputs plus `README.md`, the license files, and the changelog — nothing else. The `exclude` denylist in `Cargo.toml` keeps documentation, CI, task running, and development tooling out of it; a denylist is used rather than an allowlist because an allowlist silently drops future source files, and because under an SPDX license expression the readme and license files are not included automatically. Re-read the file list whenever a new top-level artifact appears in the repository.

## Local release helpers

`./scripts/release` drives a release by hand. None of its subcommands publish.

- `release-plz-update` — apply the version bump and changelog locally.
- `release-plz-pr` — open or refresh the release pull request.
- `cargo-release-dry <level>` — dry-run a bump at the given level.
- `semver-check` — check public API compatibility.

`wipctl` ships a binary and exposes no library API, so the API compatibility check has nothing to inspect. The command surface is the compatibility surface instead, and [the reference zone](../README.md) is what defines it; a breaking change to a verb, a flag, or an exit code is a breaking release regardless of what the API check reports.

## Binary artifacts

Prebuilt binaries and their installers are built by a separate, tag-triggered workflow configured in `dist-workspace.toml`. That workflow is generated, not written: regenerate it after editing the configuration rather than editing it directly. It publishes nothing to the registry, and it must never be the workflow registered as the trusted publisher.

Installing from a checkout instead is [the README's install section](../../README.md).

## Withdrawing a release

A published version is immutable. It cannot be replaced or deleted, only yanked, which stops new dependents from selecting it while leaving existing ones building.

```bash
cargo yank --version <VERSION>
cargo yank --version <VERSION> --undo
```

The repair for a bad release is a new release. Before `1.0.0` the minor position is the breaking position, so a break under `0.x` bumps the minor.
