# ADR-0016: An optional dependency never gates output

## Context and Problem Statement

Richer rendering and schema validation benefit from external tools, but the product's floor must
stay a bare environment.

## Considered Options

- Optional dependencies that improve output and never gate it, declared and loudly degrading
- Hard dependencies for rich views — the product's answers become hostage to an installation detail
- Silent degradation — the worst of both: the reader cannot tell a degraded answer from a full one

## Decision Outcome

Chosen option: optional, declared, loudly degrading — a dependency may improve output and may never
gate it. Each optional runtime dependency owes: a row in the dependency manifest naming its version
floor, the view it serves, and its declared fallback; a probe at verb entry, never mid-render; and
exactly one stderr line naming the degradation when the fallback is taken. A skipped check is
always named, never silently green. Environment requirements that are not tools — a UTF-8 locale,
colour capability — are probed the same way.

## Consequences

- Good: `doctor` exists as the aggregation of the shared probe set, and every view is legible in
  its fallback form.
- Bad: adding an optional tool is a three-part change (manifest, probe, fallback) by contract.

## Status

Accepted
