# A plan mints its own global identity

## Context and Problem Statement

One plan repository has no name another plan repository can write down. `project_id` is unique against one machine's registry, so two operators mint the same slug without ever meeting. A reference outward needs a name that survives a rename, a rehost, and a move between forges, and that no server allocates.

## Considered Options

- A 128-bit value minted on the machine that creates the plan, written as 32 lowercase hexadecimal characters in the plan repository's own `config.toml`
- The project slug — rejected: its uniqueness is decided against one machine, which is the property the reference needs and the slug does not have
- The repository location — rejected: a rehost renames the plan, and the specification names no forge
- A value derived from the repository's history — rejected: the charter excludes history as a store the method leans on, and a rebase, a squash, or a shallow clone each break the derivation
- A central registry that allocates — rejected: an allocating service is a value guessed on a project's behalf, which the no-defaults rule forbids

## Decision Outcome

Chosen option: the minted value. 128 bits of randomness makes a collision between independently minted plans a risk no operator has to think about, so nothing is left for a coordinating service to do. The value is opaque: nothing parses it, and it carries no time, no machine, and no order. It stands beside `project_id` and replaces nothing, because `project_id` is still what resolves and the uid is read inside the slot that resolution reached. A record that predates the key acquires one through a single explicit verb form, which mints once and refuses a second time.

Enforced by `configuration:the-plan-declares-a-global-identity`, `attachment:a-plan-uid-is-minted-only-at-creation`, `attachment:one-plan-occupies-one-slot`, and `sync:two-minted-identities-are-reported`.

## Consequences

- Good: a plan can be named from another repository, and the name outlives every rename and rehost.
- Bad: a plan carries two identities, and a repository copied by any other means carries its origin's uid, which no verb yet replaces.

## Status

Accepted
