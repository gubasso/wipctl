# A machine format is verb-local and owes a schema

## Context and Problem Statement

Consumers want structured output, and a blanket `--json` on every verb is the obvious ask. Most verbs have nothing structured to say, and every invented shape is a contract.

## Considered Options

- Verb-local machine formats, each owing a schema and a validation case
- A global `--json` — forces a JSON shape onto verbs with nothing structured to say, and each invented shape ships ungated
- No machine formats at all — pushes consumers to scrape terminal output, the least stable surface

## Decision Outcome

Chosen option: verb-local `--json`, schema-backed — a machine format exists only where output has structure to lose, and never ungated. A verb whose output has structure to lose may offer a verb-local `--json`; offering one obliges shipping a schema for the shape and a case validating real output against it. Verbs already emitting one parseable token or line (`next`, `ids`) offer none; drawings offer none.

## Consequences

- Good: exactly three output schemas exist (single-epic, rollup, listing), each gated.
- Bad: adding a machine format is a deliberate, tested act, never a free flag.

## Status

Accepted
