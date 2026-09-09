# A source alias is declared in its own file

## Context and Problem Statement

A source reference is `<alias>#<key>`, so the alias needs a home that binds it to an address and a way to list the source's open items. The plan repository already has a configuration file, and adding a table to it is the obvious move. It is also the wrong one.

## Considered Options

- `sources.toml at the zone root, with its own schema` — chosen.
- `a table inside config.toml` — rejected: the configuration is required, its every key is required, and its schema rejects unknown keys; a table that is legally absent for most projects does not belong in a file whose contract is that it is complete.
- `an alias declared inline at each reference` — rejected: the address would be restated at every reference, which is the second store the charter refuses.

## Decision Outcome

Chosen option: `sources.toml at the zone root, with its own schema` — a declaration that most projects never write belongs in a file most projects never have. The file is earned: its absence is legal and means the project declares no external source, exactly as an absent initiative directory means the project uses two tiers. It carries one table per alias, each with a url template and a list command, and both keys are required because no value is guessed on a project's behalf. The alias is local to the plan that declares it, so two plans can use one alias for different systems and neither is wrong. The configuration keeps its closed key list and its meaning: identity and cadence, complete and required.

Enforced by `external-sources:the-sources-section-is-earned` and `external-sources:an-alias-is-unique-and-slugged`.

## Consequences

- Good: the plan zone gains one optional file with one purpose, and the configuration's contract is untouched.
- Bad: a reader now looks in two files to learn what a plan repository declares about itself.

## Status

Superseded

Superseded by [ADR-a-plan-declares-everything-in-one-configuration-file.md](./ADR-a-plan-declares-everything-in-one-configuration-file.md) — the declaration is a section of the plan configuration. It is still earned, and its absence still means the project declares no source.
