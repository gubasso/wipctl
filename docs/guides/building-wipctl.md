# Building wipctl

A milestone sequence from zero to the full surface, for a team implementing the product against [the reference zone](../README.md). Each milestone ends usable and gated; none depends on a later one. An implementation may reshape the internals freely — the milestones exist because their order is forced by what depends on what. The components named here are the responsibilities in [explanation/architecture.md](../explanation/architecture.md); the gates each milestone registers are [reference/quality-gates.md](../reference/quality-gates.md).

## 1 — The record and its gate

Data model first: the config reader for both identity files, the record library over the canonical subset with the caller-supplied sink, and the checker with the identity, lane-parse, story, epic, initiative, graph, and open-questions rules. Ship `validate` (census, named schema half), `next` (head read, never gates), and the schemas with their metaschema gate — a hand-fabricated plan repository is the fixture, since no verb can create one yet.

Acceptance: a hand-written zone validates; every diagnostic in [the catalog](../reference/validation/checks.md) fires on a deliberate defect; `next` answers on a broken record with warnings.

## 2 — Identity, resolution, and adoption

The project resolution — the upward walk, the attachment registry, the identity agreement — then `init` with [the payload](../reference/scaffold-payload.md): the mint with its postfix escalation, the plan repository's creation, the hook installation, the self-check, and `--print-hooks`; `attach` for the second machine; `help` and `version`; the travelling `AGENTS.md`.

Acceptance: the journey test's first half — scaffold from an installation, confirm the host gained exactly one file, fill the charter, add a story, gate it; an `attach` of the created plan repository resolves the same record from a second checkout.

## 3 — Ranking legality as a verb

The repair: `fix` and `fix --slots` under the four guarantees, plus the R1/R2/closed-order rules in the checker (they can ship in milestone 1 as report-only; the writer needs this milestone). This is also where the transaction layer lands whole — the lock, preflight-before-first-byte, and the plan trunk commit — because `fix` is the simplest writer to prove it on.

Acceptance: the guarantee suite — identity, idempotence, non-canonicality, refusal on cycles; every mutation is one commit in the trunk grammar.

## 4 — Transitions

The journal rules in the checker, `move` with its full preflight, the write set, the report, reopening, then `start` — the atomic take — and `delete` with tombstones and burned ids, all under the transaction layer.

Acceptance: lane/journal agreement fails when hand-edited; a reopened entry counts open; two concurrent moves both survive and commit; two concurrent `start` invocations take two different entries; a refused move leaves bytes identical and names its resolution.

## 5 — Capture, drain, and rename

The mint's under-lock check against the live record and every tombstone, `new` with `--id`, the pending schema and rules in the checker and census, `ids`, completion wiring, then the drain: `land --report` and `land`, ordering by the stated instant with the full-id residual tie, drift vocabulary, all-or-nothing. Then `rename`, whose write set exercises everything above.

Acceptance: a repeat capture is refused naming the holder; the drain lands identically on two machines; every drift class reports; an ordinary rename moves every reference, burns the old id, and is one commit, while a rename resolving an id collision burns nothing.

## 6 — Sync between machines

The plan repository manager's replication cycle: `sync` (fetch, semantic reconcile, validate, fast-forward push, the report and its schema) and `resolve`. This lands after the writers because reconciliation is defined over their transactions.

Acceptance: disjoint work on two clones reconciles by construction; a same-entry conflict and a same-slug collision each report both sides and stop the push; `resolve` records the decision and the next `sync` pushes it; no path ever force-pushes.

## 7 — Views

The render library and probe set first (pipe safety, glyph tiers, `--plain`), then the views in dependency order: `epics` and `epic` (with `--json`, `--write`, and their schemas), `initiative` and `initiatives` (with theirs), `board`, `graph`, `velocity`, `flow`, `aging`, `doctor`, and finally `dashboard` as pure composition.

Acceptance: every view exits 0 on an empty record, survives a pipe, and degrades by naming what it lacks; the three tiers' arithmetic is one implementation.

## Throughout

Dogfood from milestone 1 — the implementation project's own plan record, attached and validated by its own build, as a hook. Prove every new gate fails once. Keep every refusal message naming its resolution from the milestone that introduces it — retrofitting the diagnostics contract is the same work done twice. Keep `help` honest: verbs not yet built appear under `not in this build`, which this sequence makes true at every point.
