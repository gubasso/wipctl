# Concurrent capture

Two sessions — two people, two agents, two clones — each add work without waiting on the other.
A later act reconciles everything into one ranked record, with every conflict reported rather
than silently merged.

## A capture is a delta

`wipctl new` writes a story document and a pending fragment, and touches no lane file. The
fragment states where the entry wants to land — a lane, a position — and nothing the lane files
already hold. Because captures touch disjoint files (each fragment is one file named by
an id no other session is minting), parallel captures merge by ordinary version-control
arithmetic, with no conflict to resolve. The rare random collision is the exception, and it
surfaces as the conflict itself: both captures claim the same story and fragment paths, so the
merge stops as an add/add conflict. Resolve it whole — keep one capture's document and fragment
together, re-mint the other capture entirely, merge again. Taking each path from a different side
splices two captures into one id, and no gate can see the splice; the id-uniqueness gate names
claimants only where both coexist in one record.

The id makes this possible: the mint reads nothing and waits for nothing (see
[../reference/record/ids.md](../reference/record/ids.md)), so no session ever asks the record what
the next number is.

## Two rules bound the capture

1. A fragment claims a planning lane only — `backlog` or `todo`. Landing into a work lane would
   be a transition, and a transition owes a journal event; a capture is a creation.
2. A fragment's position names only a landed entry. The ordinary two-session case — the second
   session capturing beneath what the first just captured — is deliberately refused: resolving it
   would give the drain a two-phase order and double every drift class. The second session
   captures with `after: null` and ranks the entry after the drain, or drains first. The cost is
   an extra drain on the common path; the alternative cost is a reconciliation nobody can
   predict.

## The drain

`wipctl land` orders fragments by the instant each capture stated, ties broken on the uid, so
every clone derives the same order — no filesystem timestamps, no commit history, no counter.
It reports before it writes: the landing order, then every drift (a need that closed, a position
target that moved), then lands all or nothing. Drift is reported, never repaired, because rank is
a claim and only a person makes one.

Landing is a creation, not a transition: no journal event is written, and a landed entry's
journal begins at its first move. The journal's completeness claim is per-entry from first
transition, so an entry the journal never saw arrive is the normal case, not an exception.

## What this is not

Not automatic conflict resolution — there is none, anywhere. Not a second copy of the record —
a fragment carries only its delta. Not a workflow requirement — a project that never captures
concurrently simply never has a `pending/` directory.
