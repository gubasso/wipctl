# A project is identified by a minted slug and resolved through an attachment registry

## Context and Problem Statement

With the record outside the working tree, a verb must map an invocation to a plan repository. The key must be a committed fact — a durable record keyed by checkout path makes every worktree a different project — and it should be legible to the people who type it.

## Considered Options

- A human-friendly slug derived from the project's name, disambiguated by postfix escalation, resolved through the machine's attachment registry
- An opaque minted token — immune to renames, and unreadable everywhere a person meets it: paths, messages, registries
- The repository path or a hash of it — a location, not an identity; every checkout would be its own project
- The forge URL — not every project has one, and a remote can move

## Decision Outcome

Chosen option: the minted slug with an attachment registry. `project_id` is the project's name as a slug, made unique against this machine's attachment registry at mint time by appending postfixes in priority order — parent scope, operator account, forge name, a sequential number last. A step whose segment is already present adds nothing and is skipped. Every verb resolves in two steps: walk upward to `.wipctl.toml` for the id, then look the id up in the registry; the id is restated inside the plan repository, and the two are checked against each other at every resolution.

## Consequences

- Good: identity is committed, human-legible, and shared by every checkout; an attach to the wrong repository surfaces as a disagreement between two committed facts.
- Bad: the derivation is a mint-time convenience, not a live binding — renaming the repository does not rename the id, and renaming the id is its own recorded operation, because a filesystem path and a registry slot are built on it.

## Status

Superseded

Supersedes ADR-the-plan-directory-is-discovered.

Superseded by [ADR-an-identity-is-a-minted-uid-and-a-name-is-derived.md](./ADR-an-identity-is-a-minted-uid-and-a-name-is-derived.md) — identities are minted uids, names are local or derived, and no identity names a path.
