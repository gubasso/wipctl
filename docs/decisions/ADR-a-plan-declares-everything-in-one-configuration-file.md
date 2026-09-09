# A plan declares everything in one configuration file

## Context and Problem Statement

A plan repository declared itself in three committed files: the configuration, the peer table, and the external sources. Two records argued each split, and both rested on the same reason: the configuration is required and complete, so an optional table does not belong in it.

## Considered Options

- One configuration file holding a required core and two optional sections — chosen.
- Three committed files, one per subject — rejected: none of them matches a reason the wider ecosystem splits a file off.
- A fourth file for a later optional subject — rejected: it is the same move again, one addition at a time.

## Decision Outcome

Chosen option: one file. Ecosystems split a file off for four reasons: a tool writes it, it is not committed, another program reads it, or its content does not fit the manifest's format. Both merged files match none of the four. Each was committed, each was written by a person or by a wipctl verb, each was read by wipctl alone, and each was TOML. Optionality is not one of the four reasons, and rarity is not either. `package.json` carries `eslintConfig`, `prettier`, and `jest`, and most files carry none of them. `pyproject.toml` absorbed four separate files on purpose.

The configuration domain gains the file and gains no semantics. A reader who wants to know what a url can contain still reads the peers page.

Enforced by `configuration:an-optional-section-is-legally-absent` and `configuration:an-unknown-key-is-rejected`.

## Consequences

- Good: a plan declares itself in one place, and one schema decides it.
- Good: the closed key list now catches a misspelled section rather than ignoring it.
- Bad: a person who opened the peer table alone now opens a file that also carries the cadence.

## Status

Accepted

Supersedes [ADR-a-source-alias-is-declared-in-its-own-file.md](./ADR-a-source-alias-is-declared-in-its-own-file.md). Amends [ADR-a-plan-names-its-peers-locally-and-attaches-them.md](./ADR-a-plan-names-its-peers-locally-and-attaches-them.md), whose core decision survives: a local alias, an immutable uid, and a replaceable url are still three separate facts, and the walk still fills the declared set. Only the placement changes.
