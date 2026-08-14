# Architecture

The components a conforming implementation needs, by responsibility. Nothing here names a
language or dictates code structure; each component may be a module, a package, a class, or a
process, as the implementation language prefers.

## Entry and dispatch

One executable entry point resolves the invocation: global flags, verb selection, and the
usage-error surface (exit 2). The shipped verb set is derived from the build's own dispatch units
— files, registrations, or modules, whatever the language's idiom — never a hard-coded list, so
`help`, completion, and reality cannot disagree.

## Zone resolution

One implementation of the discovery walk (config file upward, `plan_dir` joined, the exit 1/2
split), used by every verb that reads the record. `init` bypasses it by design.

## The record library

The single reader of the record. It scans the config, the five lanes, the story and epic
documents' presence, the open questions, the fragments, and the journals, and answers the derived
questions every consumer shares: eligibility, the blocking edges of both kinds, story paths,
positions, topological order, the startable set, pending claims.

Two properties are load-bearing:

- One parser. Every verb and the checker read the record through this library, so a view and a
  gate can never disagree about what a file says.
- A caller-supplied defect sink. The library reports what it finds — a malformed line, a missing
  file — through a callback or equivalent; the caller decides what a defect means. `validate`
  turns defects into failures; `next` turns them into warnings and answers anyway; `ids` ignores
  them and lists what it could read. Parse once, judge per caller.

The canonical YAML subset is what makes this library implementable without a full YAML processor:
an implementation may use a real YAML parser restricted to the subset, or a purpose-built scanner
— either must reject what the subset rejects.

## The checker

The cross-file rule catalog, implemented against the record library, emitting `location: message`
diagnostics in the three classes. Internal — reached through `validate`, the writers' preflights,
and the scaffold's self-check, never a separately supported command.

## The writers

`fix`, `move`, `land`, `delete`, sharing one write-discipline layer: the zone lock, preflight
before first byte, block relocation without re-serialisation, temp-file-and-rename placement, and
the four guarantees. The rank repair is one implementation called by `fix` and by `move`.

## The render library

Renderers take numbers and strings, never records: a bar takes value and total; the ladder takes
an ordered node list and an edge list; the board takes rows. Every record-shaped question is
answered by the record library before rendering begins. This is what keeps the pipe-safety and
glyph rules testable in one place and lets `dashboard` compose verbs without computing.

## The probe set

One shared implementation of the dependency and environment probes, reported by `doctor`,
consulted at verb entry by anything with an optional dependency or environment requirement.

## Data shipped with the product

The schemas, the scaffold payload, and the worked example, under the product's data directory.
The worked example is a complete host project (config, documentation directory, populated zone —
including one permanently pending fragment so the pending gates demonstrably select something);
it is both documentation and a test fixture.

## What is deliberately absent

No daemon, no state outside the record and the disposable cache, no plugin system, no
configuration beyond `.wipctl.toml`. The product's extension point is the record's format, which
is specified, not the program's internals, which are not.
