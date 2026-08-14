# Working in parallel sessions

Two sessions add work at the same time — two people, or two agents, in one clone or two. Neither
waits; the record stays coherent. The model behind the sequence is
[explanation/concurrent-capture.md](../explanation/concurrent-capture.md).

## 1 — Session A captures

Inputs: none.

1. Capture without touching a lane file:

   ```text
   $ wipctl new story "Rate limit the search endpoint" --epic session-hardening-c4d1 --points 2
   docs/plan/stories/rate-limit-the-search-endpoint-a7f3.md
   docs/plan/pending/rate-limit-the-search-endpoint-a7f3.yml
   ```

2. Write the goal, example, and acceptance into the document, and fill the fragment's entry
   `summary` — capture writes it absent, and the drain refuses an incomplete fragment. A's diff
   is two new files whose shared name no other session is minting.

Outputs of this phase:

```text
<PENDING_FRAGMENTS> — the fragment files under pending/; the drain's whole input, grown by each capture
```

## 2 — Session B captures, concurrently

Inputs: none.

1. Capture in the other session, then complete the fragment's entry — its `summary`, and its
   `points`, which this capture omitted:

   ```text
   $ wipctl new chore "Audit the response headers"
   docs/plan/stories/audit-the-response-headers-4c88.md
   docs/plan/pending/audit-the-response-headers-4c88.yml
   ```

2. Rank it if a landed anchor exists: B wants the entry beneath one it can see is landed, so it
   edits the fragment's `after:` to that id. A position may name a landed entry only — naming A's
   still-pending capture is drift the drain reports, landing the entry at the bottom of its
   claimed lane instead, so when B wants to rank beneath A's capture, B leaves `after: null` and
   ranks after the drain.

Outputs of this phase:

```text
<PENDING_FRAGMENTS> — grown by B's capture; disjoint from every other session's files
```

## 3 — The branches merge

Inputs: `<PENDING_FRAGMENTS>` (§1, §2).

1. Merge in version control. Outside a uid collision the captures are disjoint files and the
   merge has no conflict; the rare collision surfaces as an add/add conflict on the same paths —
   the one reliable signal, so resolve it whole: keep one side's document and fragment together
   as a pair, re-capture the other side under a fresh mint, and merge again. Taking each path
   from a different side splices two captures into one id, and no gate can see the splice.
2. Confirm the merged record:

   ```text
   $ wipctl validate
   ```

   It passes: each document is claimed by its fragment, and the census reports `2 pending`.

Outputs: none — the merged worktree is the state the drain reads.

## 4 — One session drains

Inputs: `<PENDING_FRAGMENTS>` (§1, §2).

1. Read the report first, then land:

   ```text
   $ wipctl land --report
   backlog  rate-limit-the-search-endpoint-a7f3  bottom
   backlog  audit-the-response-headers-4c88      after harden-the-proxy-defaults-90ce
   2 pending, 0 drifts
   $ wipctl land
   ...
   2 landed, 2 fragments removed
   ```

   The order is the captures' own stated instants, ties on the uid — identical on every machine.
   Any drift (a need that closed meanwhile, a moved position target) is reported, and a fragment
   with an unresolvable position lands at the bottom of its claimed lane, said out loud.

2. Rank as a person's act:

   ```text
   $ wipctl fix --slots
   ```

   shows the legal positions; an edit or `wipctl fix` settles them; `wipctl validate` closes the
   loop.

Outputs: none — the fragments are consumed and the lane files hold the entries.

## Rules that keep this safe

- A fragment claims `backlog` or `todo` only; nothing lands into flight.
- The drain reads no clock, no history, no filesystem timestamp — two clones agree by
  construction.
- Writers hold the zone lock, so a concurrent `move` and `land` in one working tree cannot lose
  an update; the waiter says who it waits for and fails bounded (exit 3) rather than forever.
