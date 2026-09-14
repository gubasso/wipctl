# Working in parallel sessions

Several agents, several worktrees, and one live record, plus a second machine when you leave the first. Nobody waits on anybody, and the record stays coherent. [explanation/concurrency.md](../explanation/concurrency.md) holds the model behind the steps, and [explanation/replication.md](../explanation/replication.md) holds the second-machine half. This page is the sequence.

## 1 — Fan the agents out

Inputs: none.

1. Give each agent its own checkout. Worktrees or clones both work. Every checkout carries the same committed `.wipctl/project.toml`, so every agent resolves the same plan repository.
2. Declare a session adapter, once per plan, if the agents run in a session system another agent can reach:

   ```toml
   [session_adapters.sessions]
   inspect = ["<the command that reads one session>", "{key}"]
   contact = ["<the command that contacts one session>", "{key}"]
   resume = ["<the command that resumes one session>", "{key}"]
   ```

   Check: `wipctl validate` passes. A declared alias with no operation is refused, and an unknown operation key is refused with it. [reference/session-adapters.md](../reference/session-adapters.md) carries the setup profile for each supported session system. Skip this step where no session can be resolved: a claim is optional, and its absence honestly says the record makes none.

3. Each agent takes work atomically rather than reading and racing, naming its own session as it goes:

   ```text
   agent A $ wipctl start --session sessions#4f19a2
   rate-limit-the-search-endpoint  Rate limit the search endpoint  2pt  ...

   agent B $ wipctl start --session sessions#7d20be
   audit-the-response-headers  Audit the response headers  1pt  ...
   ```

   Two agents starting at the same instant get two different entries. The id is chosen inside the lock. The second reads a `todo` whose head the first already removed. An agent that takes an entry by id can lose that race, and loses safely. The move verb refuses with a usage error and names the take verb.

Outputs of this phase:

```text
<TAKEN_IDS> — one entry per agent, each in doing, each taken in its own transaction
```

## 2 — Work, holding nothing

Inputs: `<TAKEN_IDS>` (§1).

1. Each agent implements its story in its own checkout. The plan is completely free the whole time: the lock protects a transaction, never a session, so an agent holds nothing while it thinks, drafts, or tests. The session claim is a locator and never a lock, and it blocks nobody.
2. New work discovered mid-session is captured without waiting:

   ```text
   $ wipctl new story "Rate limit the admin endpoint too" --points 2
   ```

   The mint checks the live record and every tombstone under the lock. A taken or burned id is refused on the spot, with the rephrase suggested. Fill the fragment's `summary`, its `points` when the capture omitted them, and its `touches`. The drain refuses an incomplete fragment.

3. Hand off through the verb, never an edit:

   ```text
   $ wipctl move <id> --to review
   ```

   The preflight runs against the record as it is now, not as the agent remembers it. A record that moved on while the agent worked is re-checked at the write. A move between work lanes keeps the claim, and a move out of them strips it in the same transaction.

Outputs of this phase:

```text
<PENDING_FRAGMENTS> — the captures under pending/, waiting for the drain
```

## 3 — Reach another agent

Inputs: `<TAKEN_IDS>` (§1).

1. Read the claims under the epic you share:

   ```text
   $ wipctl epic session-hardening --json
   ```

   Check: each claimed entry carries its available operations and its reading. A reading of `null` means the adapter declares no inspect command, so nothing was attempted. A reading carrying `active`, `ended`, or `unknown` means one was, and `observed_at` says when. An `unknown` state is the reader failing to look, never proof that a session ended. The path union and the overlaps are predictions, so treat an overlap as a reason to talk rather than as a fact about the tree.

2. Contact the peer whose claim overlaps yours, where its row lists `contact`:

   ```text
   $ printf '%s\n' "<the message>" | <the declared contact command> <the key>
   ```

   Check: exit 0. That means the native system accepted or queued the message, and nothing more. It does not mean the peer read it, agreed, or acted. A host that already authorizes the command needs no second wipctl confirmation. Where the row does not list `contact`, that session has no inbox, and the coordination goes through a person instead.

3. Resume a session only as a deliberate act, and only where its row lists `resume`. Nothing infers a resume from a reading, an age, or an overlap. Two clients attached to one session can interleave work, so confirm that the first one has stopped before you attach.

Outputs: none. The record is unchanged, because a reading gates nothing and moves nothing.

## 4 — One session drains

Inputs: `<PENDING_FRAGMENTS>` (§2).

1. Read the report first, then land:

   ```text
   $ wipctl land --report
   backlog  rate-limit-the-admin-endpoint-too
   1 pending, 0 drifts
   $ wipctl land
   ...
   1 landed, 1 fragment removed
   ```

   The drain writes fragments by their stated capture instants, with a final tie-break on the full id, so the resulting bytes are identical on every machine. A dependency that closed meanwhile is reported as drift. After landing, the ranking procedure computes each entry's rank from the record facts.

2. Validate the landed record:

   ```text
   $ wipctl validate
   ```

   The check succeeds with no ranking repair because file sequence stores membership alone. The next preview or board read applies the total ranking procedure.

Outputs: none. The fragments are consumed, and the lane files hold the entries.

## 5 — A second machine

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
- A claim is a locator. It gates nothing, and an entry whose session ended keeps it until a person resolves it.
- The drain reads no clock, no history, and no filesystem timestamp. Two machines agree by construction.
- Never force-push the plan trunk. The replication verb never does, and neither does a hand.
