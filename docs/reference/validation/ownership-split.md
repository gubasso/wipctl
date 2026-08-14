# Validation ownership

Every check has exactly one owner. The split is architectural, not incidental: it decides where a
rule is written, which tool reports it, and what a hook wires.

## The schema half

A JSON Schema owns one file's shape: field names, types, enums, bounds, and single-file
conditionals (the closed-lane requirement, the `reshaped`/`succeeded_by` pairing). Three instance
schemas gate what a project writes — the lane files, the pending fragments, the config — and
three output schemas gate what verbs emit. See [../schemas/README.md](../schemas/README.md).

The schema half runs through any draft 2020-12 instance checker, which is the product's one
optional dependency: `validate` MUST name the half as run or skipped, every time, because a check
that did not run reported as green is the one failure mode a gate must not have.

## The checker half

The cross-file checker owns every fact spanning two files: id uniqueness across lanes and
documents, entry-to-document agreement, dependency existence and acyclicity, lane-gate
eligibility, journal agreement, pending claims, ranking. The full catalog is
[checks.md](./checks.md). The checker is REQUIRED — its absence is a failure of `validate`, not a
skip — and it is internal: it MUST be reached only through `validate`, `fix`, `move`'s preflight,
`land`, `delete`, and `init`'s self-check, and MUST NOT be a separately supported command.

## Three diagnostic classes

1. failure — the record states something incorrectly and a tool MUST NOT decide the fix.
   Non-zero exit.
2. ranking failure — an order contradicting the dependency graph. Non-zero exit; kept as a
   separate class because it is the one class the repair may resolve.
3. warning — a fact the reader decides about: one the repair resolves on explicit request (a
   close date out of sequence) or one the drain handles as stated (drift). A warning MUST reach
   the reader and MUST NOT reach the exit code.

Every diagnostic the two halves emit MUST be one `location: message` line on stderr —
`<file>:<line>` where a line is known, a bare file or directory otherwise — using real filesystem
paths so a reader's tooling can jump to them. There is no diagnostic code namespace; the message
is the diagnostic. A verb relaying a record fact in its own voice — `next` warning past a defect,
the drain reporting drift — prefixes it per the command conventions
([../cli/conventions.md](../cli/conventions.md)).

## Why the gate never repairs

The repair exists (`fix`, and the repair phase `move` runs) but a hook MUST NOT invoke a writer: a gate that
rewrites the thing it gates cannot fail, and a gate that cannot fail proves nothing. Validation
MUST stay read-only and falsifiable; repair stays a person's explicit act.
