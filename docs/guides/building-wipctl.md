# Building wipctl

A milestone sequence from zero to the full surface, for a team implementing the product against [the spec zone](../README.md). Each milestone ends usable and gated, and none depends on a later one. An implementation reshapes its internals freely. The milestones exist because their order is forced by what depends on what. The components named here are the responsibilities in [explanation/architecture.md](../explanation/architecture.md). The gates each milestone registers are in [specs/SPEC-quality-gates.md](../specs/SPEC-quality-gates.md).

## 1 — The record and its gate

Build the data model first:

- the configuration reader for both configuration files
- the record library over the canonical subset, with the caller-supplied sink
- the checker, with the identity, lane-parse, story, epic, initiative, graph, and open-questions rules
- the record-wide order computation, with close date followed by the validated key chain from the plan configuration

Then ship validation, with its census and its named schema half. Ship the preview verb as a head read that never gates, with its explain flag naming the key that reached the head. Ship the schemas with their metaschema gate. A hand-fabricated plan repository is the fixture, because no verb creates one yet.

Acceptance: a hand-written zone validates. Every diagnostic in [the catalog](../specs/SPEC-validation.md) fires on a deliberate defect. The preview verb answers on a broken record, with warnings.

## 2 — Identity, resolution, and adoption

Build the project resolution: the upward walk, the attachment registry, and the identity agreement. Then build the scaffold with [the payload](../specs/SPEC-scaffold.md):

- the mint, with its postfix escalation
- the plan repository's creation, the hook installation, and the self-check
- the hook-printing flag
- the attach verb, for the second machine
- the usage and version verbs
- the travelling method document

Acceptance: the journey test's first half passes. Scaffold from an installation, confirm the host gained exactly one file, fill the charter, add a story, and gate it. An attach of the created plan repository resolves the same record from a second checkout.

## 3 — Writers and transactions

Build the supported repairs and the slot report under the writer guarantees. Lane order stays in the record reader built in milestone 1, and no writer stores or repairs it.

This is also where the transaction layer lands whole: the lock, the preflight before the first byte, and the plan trunk commit. A local-state repair is the simplest writer to prove it on.

Acceptance: the guarantee suite passes for identity, determinism, idempotence, and refusal on cycles. Every mutation is one commit in the trunk grammar.

## 4 — Transitions

Build the journal rules in the checker. Then build the move verb with its full preflight, its write set, its report, and reopening. Then build the take verb and the delete verb, with tombstones and burned ids. All of them run under the transaction layer.

Acceptance: lane and journal agreement fails when hand-edited. A reopened entry counts as open. Two concurrent moves both survive and commit. Two concurrent takes claim two different entries. A refused move leaves the bytes identical and names its resolution.

## 5 — Capture, drain, and rename

Build the mint's under-lock check against the live record and every tombstone. Then build the capture verb with its explicit-id flag. Add the pending schema and its rules to the checker and the census. Then build the id stream and the completion wiring.

Then build the drain: its report mode and its writing mode, writing lane-file bytes by the stated instant with the full-id residual tie, the dependency-drift report, and all-or-nothing. The record reader computes rank after landing. Then build the rename verb, whose write set exercises everything above.

Acceptance: a repeat capture is refused naming the holder. The drain lands identically on two machines. Dependency drift reports. An ordinary rename moves every reference, burns the old id, and is one commit. A rename resolving an id collision burns nothing.

## 6 — Replication between machines

Build the plan repository manager's replication cycle. That is the replication verb, covering fetch, semantic reconcile, validate, fast-forward push, and the report with its schema. Then build the resolution verb. This lands after the writers, because reconciliation is defined over their transactions.

Acceptance: disjoint work on two clones reconciles by construction. A same-entry conflict and a same-slug collision each report both sides and stop the push. The resolution verb records the decision, and the next replication pushes it. No path ever force-pushes.

## 7 — Views

Build the render library and the probe set first, covering pipe safety, glyph tiers, and the plain flag. Then build the views in dependency order:

1. the epic rollup and the epic resolution, with their machine formats, the cache flag, and their schemas
2. the initiative decomposition and its rollup, with theirs
3. the board, the dependency drawing, velocity, flow, and the stale view
4. the diagnostic report
5. the composite view, as pure composition

Acceptance: every view succeeds on an empty record, survives a pipe, and degrades by naming what it lacks. The three tiers' arithmetic is one implementation.

## Throughout

Dogfood from milestone 1. The implementation project's own plan record is attached and validated by its own build, as a hook. Prove every new gate fails once. Keep every refusal message naming its resolution, from the milestone that introduces it. Retrofitting the diagnostics contract is the same work done twice. Keep the usage output honest: verbs not yet built appear as planned work, which this sequence makes true at every point.
