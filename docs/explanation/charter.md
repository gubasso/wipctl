# wipctl — Charter

## What this is for

A planning method a project can adopt without adopting a documentation methodology, and the tooling that proves the project's record obeys the method. The record answers what to start, what is in flight, what was delivered, and what blocks what — without anyone opening five YAML files by hand, and without any state that is not a plain file in a plain git repository.

## Pillars

- The method and the tool are one deliverable. A rule the linter cannot check is stated as a review responsibility; a check the method does not explain is removed. Neither half changes alone.
- The host stays unassumed. An adopting project carries one committed file naming its identity; everything else — its layout, its document kinds, its conventions, where its plan is hosted and who can see it — is the project's own. The configuration has no defaults for anything a project must declare.
- A shipped artifact is proven by a gate, never asserted by prose. Every capability the documentation claims is exercised by a test or a hook; a skipped check is always named.
- The record is plain files first, and one live copy per project. It is readable before a project has a toolchain, diffable in review, and shared by every worktree, branch, and clone of the project on a machine — coordination state with exactly one current state to coordinate against, replicated between machines rather than copied per checkout.
- Every message teaches. A command's output tells its reader, human or agent, what happened, what it means for the record, and what to do next. A diagnostic that names a failure without naming its resolution is unfinished.
- The method the product teaches is the method its own project plans with. The product's own plan record passes the product's own linter, and that is a gate.

## No-gos

- No default for a path or a value a project must declare. A value guessed on a project's behalf is a value nobody wrote down.
- No second store of anything the record or the filesystem already holds. Repository history is explicitly excluded from the stores the method may lean on: a rebase rewrites it, a squash deletes it, a shallow clone never carries it. The plan trunk's own history is a replication log, not a store — no verb reads it back to answer a question the files answer.
- No promise past the build. The reference zone is the complete specification the product is built against, and conformance grows into it; a capability proposed after the product ships enters the plan record as a story before it enters the reference. Either way, a capability a given build lacks is named absent — in `help`, in the payload, everywhere — and never implied present.
- No rank a machine computes. The tool decides legality; a person decides priority. A repair may restore a legal order, never invent a preferred one.

## Iteration

Iterations are fixed-length windows anchored at a start date, both declared in the plan repository's `config.toml`. Each iteration closes stories whose points were accepted by review; velocity is the sum of those points, derived on every read and stored nowhere. Changing the cadence restarts the velocity series.
