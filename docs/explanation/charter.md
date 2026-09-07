# wipctl charter

## What this is for

A planning method a project can adopt without adopting a documentation methodology, and the tooling that proves the project's record obeys the method. The record answers what to start, what is in flight, what was delivered, and what blocks what. It answers all four without anyone opening five YAML files by hand. It holds no state that is not a plain file in a plain git repository.

## Pillars

- The method and the tool are one deliverable. A rule the linter cannot check is stated as a review responsibility. A check the method does not explain is removed. Neither half changes alone.
- The host stays unassumed. An adopting project carries one committed file naming its identity. Everything else is the project's own: its layout, its document kinds, its conventions, where its plan is hosted, and who can see it. The configuration has no defaults for anything a project must declare.
- A shipped artifact is proven by a gate, never asserted by prose. Every capability the documentation claims is exercised by a test or a hook. A skipped check is always named.
- The record is plain files first, and one live copy per project. It is readable before a project has a toolchain and diffable in review. It is shared by every worktree, branch, and clone of the project on a machine. It is coordination state with exactly one current state to coordinate against, replicated between machines rather than copied per checkout.
- Every message teaches. A command's output tells its reader, human or agent, what happened, what it means for the record, and what to do next. A diagnostic that names a failure without naming its resolution is unfinished.
- The method the product teaches is the method its own project plans with. The product's own plan record passes the product's own linter, and that is a gate.

## No-gos

- No default for a path or a value a project must declare. A value guessed on a project's behalf is a value nobody wrote down.
- No second store of anything the record or the filesystem already holds. Repository history is explicitly excluded from the stores the method leans on. A rebase rewrites it, a squash deletes it, and a shallow clone never carries it. The plan trunk's own history is a replication log and not a store. No verb reads it back to answer a question the files answer.
- No promise past the build. The spec zone is the complete specification the product is built against, and conformance grows into it. A capability proposed after the product ships enters the plan record as a story before it enters a spec. Either way, a capability a given build lacks is named absent, in the usage output, in the payload, and everywhere else. It is never implied present.
- No rank a machine computes. The tool decides legality, and a person decides priority. A repair restores a legal order and never invents a preferred one.

## Iteration

Iterations are fixed-length windows anchored at a start date. Both values are declared in the plan repository's configuration. Each iteration closes stories whose points were accepted by review. Velocity is the sum of those points, derived on every read and stored nowhere. Changing the cadence restarts the velocity series.
