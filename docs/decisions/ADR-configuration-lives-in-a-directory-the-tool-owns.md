# Configuration lives in a directory the tool owns

## Context and Problem Statement

wipctl reads configuration in two places, and each place carried a bare file with a different name. A bare file has to carry two facts at once, the product's name and the file's role, so each root gains one entry per need instead of one entry.

## Considered Options

- `.wipctl/project.toml` in the host and `.wipctl/plan.toml` in the zone — chosen.
- One bare file per place — rejected: each root gains an entry per need, and the name has to carry the product and the role together.
- `.wipctl.toml` in the host and `wipctl.toml` in the zone — rejected: the two names differ by one character and read as a typo.
- `.wipctl/identity.toml` beside `.wipctl/config.toml` — rejected: one name states content and the other states role, so neither says which domain it serves.

## Decision Outcome

Chosen option: a `.wipctl` directory in each place, holding one file named for its domain. The convention is then one sentence, and the root marker is a directory, which is what git, Cargo, and every editor toolchain already claim at a root. The product has two domains that own committed state, the project and the plan, and each file is one of them. `plan.toml` carrying `project_id` is a back-reference and not a domain mix.

Enforced by `configuration:the-project-file-marks-the-root` and `configuration:the-configuration-directory-holds-one-file`.

## Consequences

- Good: one sentence states where configuration lives, in both places.
- Good: a reader opening the plan zone meets the record, not the tool's files.
- Bad: the upward walk tests a path two segments deep, at one more filesystem call per level.
- Bad: three rule ids naming the old noun were renamed, so a citation of an old id no longer resolves. The ids were cited by prose alone, and no code cites one yet.

## Status

Accepted

Supersedes [ADR-the-config-is-toml-at-the-project-root.md](./ADR-the-config-is-toml-at-the-project-root.md). Two of that record's decisions survive: TOML is still the format, for the reasons it gave, and one marker still cannot disagree with itself.
