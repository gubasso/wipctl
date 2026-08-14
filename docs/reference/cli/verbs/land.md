# wipctl land

The drain: reconciles pending fragments into the lane files, in an order every clone derives
identically, with every conflict reported rather than silently merged.

## Usage

```text
wipctl land [--report] [<plan-dir>]
```

## Ordering rule

Fragments MUST land ascending by the instant each capture stated (`captured`), ties broken on the
uid — the id's trailing four hex characters, compared bytewise; this is the one stated exemption
from id opacity ([../../record/ids.md](../../record/ids.md)) — and a residual tie on the full id,
so the order is total and two runs — on two machines, on a shallow clone, after a rebase — agree.
The verb MUST NOT read repository history, a clock, a commit, or a reflog.

## `--report` — the read-only half

Prints the resolved landing order, one line per fragment:

```text
backlog  rate-limit-the-search-endpoint-a7f3  bottom
todo     audit-the-headers-4c88               after profile-composition-e01a
wipctl: drift: audit-the-headers-4c88 needs secure-session-storage-9c2e, which closed
2 pending, 1 drift
```

Position is `bottom` or `after <id>`. Drift lines follow (see
[../../record/pending-fragment.md](../../record/pending-fragment.md) for the vocabulary); a
fragment whose position target is gone, moved, or closed MUST land at the bottom of its claimed
lane and the report MUST say so. An empty or absent `pending/` succeeds and says
`nothing to land`. The zone MUST be byte-for-byte unchanged, fragments included.

## Writing mode

Everything the report shows, then the writes: each resolved entry is appended to its claimed lane
at its claimed position, each landed fragment is removed, and the summary reads
`2 landed, 2 fragments removed`. A second run prints `nothing to land`, exit 0.

- All or nothing: any content-check failure or a cyclic landed result MUST refuse and write
  nothing. No partial landing under any flag.
- The drain's own preflight: a fragment whose entry lacks a field a lane entry requires (`id`,
  `type`, `points`, `summary`) MUST be refused, naming the fragment and the field — capture
  writes `points` and `summary` absent for the operator to fill, and the drain MUST NOT write a
  lane entry the lane schema rejects.
- The shared writer discipline MUST apply without exception: deterministic, byte-identical for
  everything untouched, idempotent on a re-run, never canonicalising an order somebody chose.
- Landing is a creation, not a transition: a journal event MUST NOT be written, and a landed
  entry's journal begins at its first `move`.
- The drain MUST NOT repair ranking afterwards. A ranking failure in the landed record MUST be
  reported, and `fix` remains the separate, explicit act.
- The verb MUST NOT land into a work lane and MUST NOT invoke version control, even to remove a
  fragment. A hook MUST NOT invoke it. It MUST hold the zone lock.
