# ADR-0052: A title and its id always match; renaming is an operation

## Context and Problem Statement

With the slug as the whole id (ADR-0051), an id is derived from its title — and a title edited after capture would silently disagree with it. A drift a reader has to tolerate, or an invariant a tool holds: the record must pick one.

## Considered Options

- The invariant: `id == slugify(title)` always, with `rename` as the supported operation that changes both together
- Tolerate the drift — the title line decays into a repetition nothing can check, which is the state the old grammar lived in
- Forbid title changes — agreements change after work begins; a record that cannot say so drives edits underground

## Decision Outcome

Chosen option: the invariant, held by one function and one verb. Slugification is specified once and used by the mint, by `rename`, and by the gate — three consumers computing one string three ways is how a title and an id come to disagree in a system that claims they cannot. The title line `# <id> — <short title>` becomes checkable: the checker verifies the title slugifies to the id. `rename` is a writer in one transaction whose write set is everything that names the id — documents, journal, fragment, every `needs`, `succeeded_by`, `Blocks:`, `epic`, and `Initiative` reference. The new id is minted through the same checks as a fresh one, and the old id is burned by a tombstone whose final event destination is `renamed`, so a stale reference fails loudly rather than resolving to nothing.

## Consequences

- Good: a repetition became an invariant, and changing a title is a recorded, reviewable act instead of a drift.
- Bad: a heavily renamed entry leaves a chain of tombstones, each holding a title nobody can reuse.

## Status

Accepted

Amends ADR-0024.
