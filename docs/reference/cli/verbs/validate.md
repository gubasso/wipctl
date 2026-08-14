# wipctl validate

Is the record coherent. The read-only gate every hook calls.

## Usage

```text
wipctl validate [<plan-dir>]
```

At most one positional (else exit 2). No flags.

## Contract

Two independent halves run:

1. The cross-file checker — REQUIRED. Every rule in [../../validation/checks.md](../../validation/checks.md). An installation missing its checker MUST exit 1 with `the record was not validated` — absence is a failure, never a skip.
2. Schema validation of the five lane files, every pending fragment, and `.wipctl.toml` — through an optional draft 2020-12 instance checker. Absent, the half MUST be skipped and named.

On success stdout carries the census and the schema report:

```text
ok: 41 entries, 3 pending, 5 epics, 2 open questions
schemas: ok
```

Counts MUST be singular/plural correct; entries are counted from the lane scan, pending from the fragment scan, epics from the documents, questions from the `##` sections. With no instance checker the second line reads `schemas: skipped — no instance checker on PATH`.

Diagnostics from either half go to stderr as `<location>: <message>` lines. Either half failing: exit 1, no census printed. A missing schema directory is a broken install: exit 1, not a skip.

The contract's principle: a check that did not run MUST always be reported as skipped, never silently green.

## Example, failing

```text
$ wipctl validate docs/plan
docs/plan/lanes/todo.yml:9: id 'rate-limit-the-search-endpoint-a7f3' already appears in doing.yml
docs/plan/stories/audit-the-headers-4c88.md: no entry and no fragment names this document
wipctl: 2 failures
$ echo $?
1
```
