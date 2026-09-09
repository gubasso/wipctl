# Architecture

The components a conforming implementation needs, by responsibility. Nothing here names a language or dictates code structure. Each component is a module, a package, a class, or a process, as the implementation language prefers.

## The two-repository shape

Every project the tool touches spans two repositories. The host carries one project file and nothing else of the tool's. The plan repository carries the record, its configuration, and its own hooks, at a machine-level slot every checkout of the project resolves to.

The components below split along that line. Resolution and the identity live at the boundary. Everything that reads or writes the record lives on the plan side. Nothing lives on the host side, by design.

## Entry and dispatch

One executable entry point resolves the invocation: global flags, verb selection, and the usage-error surface. The shipped verb set is derived from the build's own dispatch units. Those are files, registrations, or modules, whatever the language's idiom. It is never a hard-coded list, so the usage output, completion, and reality cannot disagree.

## Project resolution

One implementation of the two-step resolution, used by every verb that touches a record. It covers the upward walk to the project file and the attachment registry lookup. It also covers the identity agreement and the split between a usage error and a failed check. The scaffold bypasses it by design.

It owns the third path too. Where an invocation names a peer, the component resolves this plan first and reads its peer table. It then finds the slot whose configuration declares the `plan_id` that alias binds. A slot holds one plan, whether it arrived from this project or as a peer, so nothing downstream distinguishes the two.

## The record library

The single reader of the record. It scans the configuration, the five lanes, the presence of the story, epic, and initiative documents, the open questions, the fragments, and the journals. It scans the peer table too. It reads an attached peer through itself, at the tree of that peer's current commit. A peer is a plan, and there is no second reader for one. It answers the derived questions every consumer shares: eligibility, the blocking edges of both kinds, story paths, positions, topological order, the startable set, pending claims, and membership at both tiers.

Two properties are load-bearing:

- One parser. Every verb and the checker read the record through this library, so a view and a gate can never disagree about what a file says.
- A caller-supplied defect sink. The library reports what it finds, such as a malformed line or a missing file, through a callback or an equivalent. The caller decides what a defect means. Validation turns defects into failures. The preview verb turns them into warnings and answers anyway. The id stream ignores them and lists what it can read. Parse once, judge per caller.

The canonical YAML subset is what makes this library implementable without a full YAML processor. An implementation uses either a real YAML parser restricted to the subset, or a purpose-built scanner. Either one must reject what the subset rejects.

## The checker

The cross-file rule catalog, implemented against the record library, emitting location-and-message diagnostics in the three classes. It is internal. It is reached through validation, the writers' preflights, the reconciliation, and the scaffold's self-check, and never as a separately supported command.

## The writers and the transaction layer

The ten writing verbs share one write-discipline layer. That layer holds the transaction lock, the preflight before the first byte, and block relocation without re-serialisation. It also holds temporary-file-and-rename placement, the four guarantees, and the closing plan trunk commit in the fixed grammar. The rank repair is one implementation, called by the repair verb and by the transition verbs.

## The plan repository manager

The one component that speaks git. It creates the plan repository, installs its hook set, and commits a transaction's staged paths. It also runs the replication cycle: fetch, semantic reconcile, and fast-forward push.

Reconciliation is part of this component. It works entry-wise through the record library, never line-wise through a textual merge. A conflict it cannot decide is a report, and the resolution verb records the person's decision.

## The render library

Renderers take numbers and strings, never records. A bar takes a value and a total. The ladder takes an ordered node list and an edge list. The board takes rows. Every record-shaped question is answered by the record library before rendering begins. This is what keeps the pipe-safety and glyph rules testable in one place, and it lets the composite view compose verbs without computing.

## The probe set

One shared implementation of the dependency and environment probes. The diagnostic verb reports them, and anything with an optional dependency or an environment requirement consults them at verb entry. It also answers the plan-machinery questions that verb reports: the lock directory in use, the attachment and identity state, and the hook installation.

## Data shipped with the product

The schemas, the scaffold payload, and the worked example, under the product's data directory. The worked example is a complete host project with its plan repository: a project file and a populated zone. The zone carries one permanently pending fragment, so the pending gates demonstrably select something. The example is both documentation and a test fixture.

## What is deliberately absent

There is no daemon, no queue, and no plugin system. There is no state beyond the four stated locations: the plan repository holding the record, the state directory holding replication bookkeeping, the disposable cache, and the transaction-scoped lock. The product's extension point is the record's format, which is specified. It is not the program's internals, which are not.
