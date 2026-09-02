# Working in parallel sessions

Several agents, several worktrees, one live record — and a second machine when you leave the first. Nobody waits on anybody; the record stays coherent. The model behind the sequence is [explanation/concurrency.md](../explanation/concurrency.md); this page is the sequence.

## 1 — Fan the agents out

Inputs: none.

1. Give each agent its own checkout — worktrees or clones, it does not matter. Every checkout carries the same committed `.wipctl.toml`, so every agent resolves the same plan repository.
2. Each agent takes work atomically rather than reading and racing:

   ```text
   agent A $ wipctl start
   rate-limit-the-search-endpoint  Rate limit the search endpoint  2pt  ...

   agent B $ wipctl start
   audit-the-response-headers  Audit the response headers  1pt  ...
   ```

   Two agents starting at the same instant get two different entries: the id is chosen inside the lock, and the second reads a `todo` whose head the first already removed. An agent that takes an entry by id instead can lose that race, and loses safely — `move` refuses with exit 2 and names `start` as the correction.

Outputs of this phase:

```text
<TAKEN_IDS> — one entry per agent, each in doing, each taken in its own transaction
```

## 2 — Work, holding nothing

Inputs: `<TAKEN_IDS>` (§1).

1. Each agent implements its story in its own checkout. The plan is completely free the whole time: the lock protects a transaction, never a session, so an agent holds nothing while it thinks, drafts, or tests.
2. New work discovered mid-session is captured without waiting:

   ```text
   $ wipctl new story "Rate limit the admin endpoint too" --points 2
   ```

   The mint checks the live record and every tombstone under the lock, so a taken or burned id is refused on the spot with the rephrase suggested. Fill the fragment's `summary` (and `points`, when the capture omitted it); the drain refuses an incomplete fragment.

3. Hand off through the verb, never an edit:

   ```text
   $ wipctl move <id> --to review
   ```

   The preflight runs against the record as it is now — not as the agent remembers it — so a record that moved on while the agent worked is re-checked at the write.

Outputs of this phase:

```text
<PENDING_FRAGMENTS> — the captures under pending/, waiting for the drain
```

## 3 — One session drains

Inputs: `<PENDING_FRAGMENTS>` (§2).

1. Read the report first, then land:

   ```text
   $ wipctl land --report
   backlog  rate-limit-the-admin-endpoint-too  bottom
   1 pending, 0 drifts
   $ wipctl land
   ...
   1 landed, 1 fragment removed
   ```

   The order is the captures' own stated instants, with a residual tie on the full id — identical on every machine. Any drift (a need that closed meanwhile, a moved position target) is reported, and a fragment with an unresolvable position lands at the bottom of its claimed lane, said out loud.

2. Rank as a person's act:

   ```text
   $ wipctl fix --slots
   ```

   shows the legal positions; an edit or `wipctl fix` settles them; `wipctl validate` closes the loop.

Outputs: none — the fragments are consumed and the lane files hold the entries.

## 4 — A second machine

Inputs: none.

1. Leaving a machine, replicate:

   ```text
   $ wipctl sync
   pushed the plan trunk
   ```

2. Arriving at another, replicate again before taking work. Disjoint work — captures, transitions of different entries — reconciles by construction. The two genuine conflicts are reported, never merged: the same entry transitioned on both sides, decided by a person through `wipctl resolve <id> --keep here|remote`; and one slug minted on both sides, recovered with `wipctl rename` on the losing capture under a rephrased title, which moves its document and fragment together and leaves the id to the other.

Outputs: none — both machines hold the same trunk.

## Rules that keep this safe

- Take atomically (`start`); hold nothing while working; accept a refusal — exit 2 means somebody got there first, so ask again.
- A fragment claims `backlog` or `todo` only; nothing lands into flight.
- The drain reads no clock, no history, no filesystem timestamp — two machines agree by construction.
- Never force-push the plan trunk; `sync` never does, and neither should a hand.
