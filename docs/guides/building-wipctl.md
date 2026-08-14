# Building wipctl

A milestone sequence from zero to the full surface, for a team implementing the product against [the reference zone](../README.md). Each milestone ends usable and gated; none depends on a later one. An implementation may reshape the internals freely — the milestones exist because their order is forced by what depends on what. The components named here are the responsibilities in [explanation/architecture.md](../explanation/architecture.md); the gates each milestone registers are [reference/quality-gates.md](../reference/quality-gates.md).

## 1 — The record and its gate

Data model first: the config reader, the record library over the canonical subset with the caller-supplied sink, and the checker with the config, lane-parse, story, epic, graph, and open-questions rules. Ship `validate` (census, named schema half), `next` (head read, never gates), and the schemas with their metaschema gate.

Acceptance: a hand-written zone validates; every diagnostic in [the catalog](../reference/validation/checks.md) fires on a deliberate defect; `next` answers on a broken record with warnings.

## 2 — Adoption

`init` with [the payload](../reference/scaffold-payload.md), the substitution set, create-only preflight, the self-check, `--print-hooks`, and `--headings-gate`; `help` and `version`; the travelling `AGENTS.md`.

Acceptance: the journey test's first half — scaffold from an installation, fill the charter, add a story, gate it.

## 3 — Ranking legality as a verb

The repair: `fix` and `fix --slots` under the four guarantees, plus the R1/R2/closed-order rules in the checker (they can ship in milestone 1 as report-only; the writer needs this milestone).

Acceptance: the guarantee suite — identity, idempotence, non-canonicality, refusal on cycles.

## 4 — Transitions

The journal rules in the checker, `move` with its full preflight, the write set, the report, the opt-in commit, reopening, then `delete` with tombstones and burned ids, and the zone lock over every writer shipped so far.

Acceptance: lane/journal agreement fails when hand-edited; a reopened entry counts open; two concurrent moves both survive; a refused move leaves bytes identical.

## 5 — Identity and capture

The mint, `new`, the pending schema and rules in the checker and census, `ids`, completion wiring, then the drain: `land --report` and `land`, ordering by the stated instant, drift vocabulary, all-or-nothing, under the lock.

Acceptance: two-session capture merges clean and lands identically on two clones; every drift class reports; a repeat capture collides with nothing.

## 6 — Views

The render library and probe set first (pipe safety, glyph tiers, `--plain`), then the views in dependency order: `epics` and `epic` (with `--json`, `--write`, and their schemas), `board`, `graph`, `velocity`, `flow`, `aging`, `doctor`, and finally `dashboard` as pure composition.

Acceptance: every view exits 0 on an empty record, survives a pipe, and degrades by naming what it lacks.

## Throughout

Dogfood from milestone 1 — the implementation project's own plan record, validated by its own build, as a hook. Register record gates at commit and push from milestone 2. Prove every new gate fails once. Keep `help` honest: verbs not yet built appear under `not in this build`, which this sequence makes true at every point.
