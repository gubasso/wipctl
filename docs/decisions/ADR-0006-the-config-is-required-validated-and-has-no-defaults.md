# ADR-0006: The config is required, validated, and has no defaults

## Context and Problem Statement

The tool needs parameters it cannot infer from the record: where the zone sits, and the iteration
window. Tools commonly guess such values to feel friendly.

## Considered Options

- A required, schema-validated config with no defaults
- Inferring `docs/plan` when present — the first default becomes the assumed layout, and the single
  host assumption quietly grows
- Parsing the charter's prose for the cadence — prose is for readers; a tool parsing it turns style
  edits into breakage
- Warning instead of failing on absence — views would run against a half-configured project and
  disagree with the gate

## Decision Outcome

Chosen option: required, validated, no defaults — a value guessed on a project's behalf is a value
nobody wrote down. `.wipctl.toml` is required: its absence is a validation failure, not a warning,
and nothing infers an anchor in its place. It is schema-validated with unknown keys rejected at
every level. No key has a default. The checker owns presence of the file and its keys; the schema
owns their types. The one accepted duplication is the cadence stated in both the config and the
charter, each for its audience.

## Consequences

- Good: every verb can trust the config exists once validation passes, and adoption asks one real
  question (where does the zone live) instead of silently answering it.
- Bad: no verb answers until the adopting project writes the config.

## Status

Accepted

Amended by ADR-0037 — the no-defaults rule binds verbs reading the config; the scaffold generates
a complete file the operator owns.
