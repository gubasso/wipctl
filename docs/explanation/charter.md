# wipctl — Charter

## What this is for

A planning method a project can adopt without adopting a documentation methodology, and the
tooling that proves the project's record obeys the method. The record answers what to start, what
is in flight, what was delivered, and what blocks what — without anyone opening five YAML files by
hand, and without any state that is not a plain file in version control.

## Pillars

- The method and the tool are one deliverable. A rule the linter cannot check is stated as a
  review responsibility; a check the method does not explain is removed. Neither half changes
  alone.
- The host stays unassumed. An adopting project has a documentation directory; everything else —
  its layout, its document kinds, its conventions — is the project's own. The configuration has no
  defaults for anything a project must declare.
- A shipped artifact is proven by a gate, never asserted by prose. Every capability the
  documentation claims is exercised by a test or a hook; a skipped check is always named.
- The record is plain files first. It is readable before a project has a toolchain, diffable in
  review, and mergeable by ordinary version-control arithmetic.
- The method the product teaches is the method its own project plans with. The product's own plan
  record passes the product's own linter, and that is a gate.

## No-gos

- No default for a path a project must declare. A value guessed on a project's behalf is a value
  nobody wrote down.
- No second store of anything the record or the filesystem already holds. Repository history is
  explicitly excluded from the stores the method may lean on: a rebase rewrites it, a squash
  deletes it, a shallow clone never carries it.
- No capability documented as reference before it exists. An unbuilt capability is a story in the
  plan record, never a page in the reference.
- No rank a machine computes. The tool decides legality; a person decides priority. A repair may
  restore a legal order, never invent a preferred one.

## Iteration

Iterations are fixed-length windows anchored at a start date, both declared in `.wipctl.toml`.
Each iteration closes stories whose points were accepted by review; velocity is the sum of those
points, derived on every read and stored nowhere. Changing the cadence restarts the velocity
series.
