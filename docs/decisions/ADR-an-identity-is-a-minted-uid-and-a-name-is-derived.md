# An identity is a minted uid and a name is derived

## Context and Problem Statement

The project identity was both a committed name and a local path. A replicated rename could collide with another machine's registry, while an opaque path would be hard to read.

## Considered Options

- Mint both identities as opaque uids and keep names local or derived — chosen.
- Keep a committed slug as the project identity and path — rejected because local path availability cannot replicate safely.
- Use one uid for the project and plan — rejected because settling two plan mints would force a host commit in every clone.

## Decision Outcome

Chosen option: `project_id` and `plan_id` are separate 128-bit random values. The host commits the project identity, and the plan commits both. A machine mints a readable slot name for paths. Readers derive a project slug for messages. Neither name is an identity, and neither identity is a path.

Enforced by `configuration:the-project-file-carries-one-key`, `configuration:the-plan-declares-a-global-identity`, `attachment:resolution-is-two-steps`, and `slot-naming:a-derived-slug-decides-only-at-mint-time`.

## Consequences

- Good: identity survives every rename without making a replicated name select a local path.
- Bad: an identity alone is unreadable, so paths and messages must restore the name from local facts.

## Status

Accepted

Supersedes [ADR-a-project-is-identified-by-a-minted-slug.md](./ADR-a-project-is-identified-by-a-minted-slug.md).
