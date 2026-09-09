# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- breaking, `config`: Identify projects and plans with minted uids, name slots locally, and rename `plan_uid` and peer `uid` to `plan_id`. Adopters must mint both identities, update peer rows, and reattach each plan so its machine can mint a slot name.

## [0.3.0](https://github.com/gubasso/wipctl/compare/v0.2.0...v0.3.0) - 2026-09-09

### Added

- breaking, `config`: Name each configuration section for what it decides ([#35](https://github.com/gubasso/wipctl/pull/35))
- breaking, `config`: Declare peers and sources in the plan configuration ([#34](https://github.com/gubasso/wipctl/pull/34))
- breaking, `config`: Move each configuration file into a wipctl directory ([#32](https://github.com/gubasso/wipctl/pull/32))

## [0.2.0](https://github.com/gubasso/wipctl/compare/v0.1.10...v0.2.0) - 2026-09-08

### Other

- breaking, `changelog`: Record the break that 0.1.10 shipped undeclared ([#29](https://github.com/gubasso/wipctl/pull/29))

## [0.1.10](https://github.com/gubasso/wipctl/compare/v0.1.9...v0.1.10) - 2026-09-08

### Added

- `cli`: Report the peers and sync the closure on request ([#25](https://github.com/gubasso/wipctl/pull/25))

### Correction

This entry was generated before [#27](https://github.com/gubasso/wipctl/pull/27) landed, and the tag sits on a commit that contains it. So 0.1.10 also carries a breaking change to the command surface: the rule forbidding a project flag is replaced, and a caller who wants another plan names it with `--plan <alias>` instead of changing directory. Under 0.y the minor position is the breaking one, so that change owed a minor bump and shipped as a patch. A published entry is never rewritten, so the correction is recorded here and the version line moves at the next release.

## [0.1.9](https://github.com/gubasso/wipctl/compare/v0.1.8...v0.1.9) - 2026-09-08

### Added

- `validation`: Check peers, and prove the graph over the attached closure ([#23](https://github.com/gubasso/wipctl/pull/23))

## [0.1.8](https://github.com/gubasso/wipctl/compare/v0.1.7...v0.1.8) - 2026-09-08

### Added

- `record`: Let a needs id name an entry in an attached peer ([#21](https://github.com/gubasso/wipctl/pull/21))

## [0.1.7](https://github.com/gubasso/wipctl/compare/v0.1.6...v0.1.7) - 2026-09-08

### Added

- `record`: Let a plan name its peers, write them, and attach them ([#19](https://github.com/gubasso/wipctl/pull/19))

## [0.1.6](https://github.com/gubasso/wipctl/compare/v0.1.5...v0.1.6) - 2026-09-08

### Added

- `record`: Give a plan a global identity ([#17](https://github.com/gubasso/wipctl/pull/17))

## [0.1.5](https://github.com/gubasso/wipctl/compare/v0.1.4...v0.1.5) - 2026-09-08

### Other

- `explanation`: Split the concurrency chapter and gate the line cap

## [0.1.4](https://github.com/gubasso/wipctl/compare/v0.1.3...v0.1.4) - 2026-09-08

### Other

- `repo`: State the contribution grant and correct the published file set

## [0.1.3](https://github.com/gubasso/wipctl/compare/v0.1.2...v0.1.3) - 2026-09-08

### Other

- `docs`: Adopt the spec-driven-docs convention ([#7](https://github.com/gubasso/wipctl/pull/7))

## [0.1.2](https://github.com/gubasso/wipctl/compare/v0.1.1...v0.1.2) - 2026-09-06

### Fixed

- `ci`: Gate the merge on every job, not one ([#3](https://github.com/gubasso/wipctl/pull/3))

## [0.1.1](https://github.com/gubasso/wipctl/compare/v0.1.0...v0.1.1) - 2026-09-06

### Fixed

- `repo`: Generate a changelog the prose gates accept ([#2](https://github.com/gubasso/wipctl/pull/2))

## [0.1.0](https://github.com/gubasso/wipctl/releases/tag/v0.1.0) - 2026-09-06

### Added

- breaking, `docs`: refound the plan record as a per-project repository
- `docs/agnosticism`: enforce the host-agnosticism doctrine

### Other

- `repo`: adopt the release-kit release convention
- `toolchain`: bootstrap the repository toolchain
- pin rust toolchain to 1.97.1
- land the greenfield product specification
- first commit
