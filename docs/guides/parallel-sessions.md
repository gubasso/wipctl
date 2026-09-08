# Working in parallel sessions

Several agents, several worktrees, and one live record, plus a second machine when you leave the first. Nobody waits on anybody, and the record stays coherent. [explanation/concurrency.md](../explanation/concurrency.md) holds the model behind the steps. This page is the sequence.

## 1 — Fan the agents out

Inputs: none.

1. Give each agent its own checkout. Worktrees or clones both work. Every checkout carries the same committed `.wipctl.toml`, so every agent resolves the same plan repository.
2. Each agent takes work atomically rather than reading and racing:

   ```text
   agent A $ wipctl start
   rate-limit-the-search-endpoint  Rate limit the search endpoint  2pt  ...

   agent B $ wipctl start
   audit-the-response-headers  Audit the response headers  1pt  ...
   ```

   Two agents starting at the same instant get two different entries. The id is chosen inside the lock. The second reads a `todo` whose head the first already removed. An agent that takes an entry by id can lose that race, and loses safely. The move verb refuses with a usage error and names the take verb.

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

   The mint checks the live record and every tombstone under the lock. A taken or burned id is refused on the spot, with the rephrase suggested. Fill the fragment's `summary`, and its `points` when the capture omitted them. The drain refuses an incomplete fragment.

3. Hand off through the verb, never an edit:

   ```text
   $ wipctl move <id> --to review
   ```

   The preflight runs against the record as it is now, not as the agent remembers it. A record that moved on while the agent worked is re-checked at the write.

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

   The order is the captures' own stated instants, with a residual tie on the full id. It is identical on every machine. Any drift is reported, such as a need that closed meanwhile or a moved position target. A fragment with an unresolvable position lands at the bottom of its claimed lane, and the report says so.

2. Rank as a person's act:

   ```text
   $ wipctl fix --slots
   ```

   shows the legal positions. An edit or `wipctl fix` settles them, and `wipctl validate` closes the loop.

Outputs: none. The fragments are consumed, and the lane files hold the entries.

## 4 — A second machine

Inputs: none.

1. Leaving a machine, replicate:

   ```text
   $ wipctl sync
   pushed the plan trunk
   ```

2. At another machine, replicate again before taking work. Disjoint work reconciles by construction, which covers captures and transitions of different entries. Two genuine conflicts are reported and never merged:
   - the same entry transitioned on both sides. A person decides it through `wipctl resolve <id> --keep here|remote`.
   - one slug minted on both sides. Recover with `wipctl rename` on the losing capture, under a rephrased title. That moves its document and fragment together and leaves the id to the other.

Outputs: none. Both machines hold the same trunk.

## Rules that keep this safe

- Take atomically with the take verb. Hold nothing while working. Accept a refusal: a usage error means somebody got there first, so ask again.
- A fragment claims `backlog` or `todo` only. Nothing lands into flight.
- The drain reads no clock, no history, and no filesystem timestamp. Two machines agree by construction.
- Never force-push the plan trunk. The replication verb never does, and neither does a hand.
