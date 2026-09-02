# Architecture

The components a conforming implementation needs, by responsibility. Nothing here names a language or dictates code structure; each component may be a module, a package, a class, or a process, as the implementation language prefers.

## The two-repository shape

Every project the tool touches spans two repositories: the host, which carries one identity file and nothing else of the tool's, and the plan repository, which carries the record, its config, and its own hooks, at a machine-level slot every checkout of the project resolves to. The components below split along that line — resolution and the identity live at the boundary; everything that reads or writes the record lives on the plan side; nothing lives on the host side, by design.

## Entry and dispatch

One executable entry point resolves the invocation: global flags, verb selection, and the usage-error surface (exit 2). The shipped verb set is derived from the build's own dispatch units — files, registrations, or modules, whatever the language's idiom — never a hard-coded list, so `help`, completion, and reality cannot disagree.

## Project resolution

One implementation of the two-step resolution — the upward walk to `.wipctl.toml`, the attachment registry lookup, the identity agreement, the exit 1/2 split — used by every verb that touches a record. `init` bypasses it by design.

## The record library

The single reader of the record. It scans the config, the five lanes, the story, epic, and initiative documents' presence, the open questions, the fragments, and the journals, and answers the derived questions every consumer shares: eligibility, the blocking edges of both kinds, story paths, positions, topological order, the startable set, pending claims, membership at both tiers.

Two properties are load-bearing:

- One parser. Every verb and the checker read the record through this library, so a view and a gate can never disagree about what a file says.
- A caller-supplied defect sink. The library reports what it finds — a malformed line, a missing file — through a callback or equivalent; the caller decides what a defect means. `validate` turns defects into failures; `next` turns them into warnings and answers anyway; `ids` ignores them and lists what it could read. Parse once, judge per caller.

The canonical YAML subset is what makes this library implementable without a full YAML processor: an implementation may use a real YAML parser restricted to the subset, or a purpose-built scanner — either must reject what the subset rejects.

## The checker

The cross-file rule catalog, implemented against the record library, emitting `location: message` diagnostics in the three classes. Internal — reached through `validate`, the writers' preflights, the reconciliation, and the scaffold's self-check, never a separately supported command.

## The writers and the transaction layer

`new`, `fix`, `move`, `land`, `delete`, `start`, `rename`, `sync`, and `resolve`, sharing one write-discipline layer: the transaction lock, preflight before first byte, block relocation without re-serialisation, temp-file-and-rename placement, the four guarantees, and the closing plan trunk commit in the fixed grammar. The rank repair is one implementation called by `fix` and by the transition verbs.

## The plan repository manager

The one component that speaks git: creating the plan repository, installing its hook set, committing a transaction's staged paths, and the sync cycle — fetch, semantic reconcile, fast-forward push. Reconciliation is part of this component and works entry-wise through the record library, never line-wise through a textual merge; a conflict it cannot decide is a report, and `resolve` records the person's decision.

## The render library

Renderers take numbers and strings, never records: a bar takes value and total; the ladder takes an ordered node list and an edge list; the board takes rows. Every record-shaped question is answered by the record library before rendering begins. This is what keeps the pipe-safety and glyph rules testable in one place and lets `dashboard` compose verbs without computing.

## The probe set

One shared implementation of the dependency and environment probes, reported by `doctor`, consulted at verb entry by anything with an optional dependency or environment requirement. It also answers the plan-machinery questions `doctor` reports: the lock directory in use, the attachment and identity state, the hook installation.

## Data shipped with the product

The schemas, the scaffold payload, and the worked example, under the product's data directory. The worked example is a complete host project with its plan repository (identity file, populated zone — including one permanently pending fragment so the pending gates demonstrably select something); it is both documentation and a test fixture.

## What is deliberately absent

No daemon, no queue, no plugin system, no state beyond the four stated locations — the plan repository (the record), the state directory (sync bookkeeping), the disposable cache, and the transaction-scoped lock. The product's extension point is the record's format, which is specified, not the program's internals, which are not.
