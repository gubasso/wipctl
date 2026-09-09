# Concurrency

How many agents share one plan on one machine without waiting on each other and without losing each other's work. One record per project, and a lock measured in milliseconds. [replication.md](./replication.md) holds the other half, where two machines diverge and a person decides.

## The one idea

A lock protects a transaction, never a session.

An agent that is implementing a story is not editing the plan. It touched the plan for a few milliseconds when it took the work. It will touch the plan again for a few milliseconds when it hands the work to review. In between, the plan is free.

```text
the picture that needs a queue          the picture that does not

A ├LOCK──[ 12 min of work ]──UNLOCK┤    A ─[20ms]── 12 min, holds nothing ──[20ms]─
B      ├─ blocked 12 minutes ─────┤     B ──[20ms]──── 8 min, holds nothing ───[20ms]
C      ├─ blocked 12 minutes ──────     C ───[20ms]─── 30 min, holds nothing ────[20]
```

On the left the lock is scarce, so fairness matters, so you need a queue, priorities, liveness heartbeats, and something alive to reap dead waiters. On the right the lock is held for the length of a database write. Contention is measured in milliseconds and the deepest queue that ever forms is one.

The critical section is exactly four steps:

```text
┌─────────────────────────────────────────────────────────┐
│  LOCK                                                    │
│    1. read the record at current HEAD                    │
│    2. preflight — every check, before the first byte     │
│    3. write, repair the ranking, validate                │
│    4. commit                                             │
│  UNLOCK                                                  │
└─────────────────────────────────────────────────────────┘
    everything else — thinking, drafting, coding — is outside
```

## Where the lock lives

```text
   ~/src/payments-main/       worktree, branch trunk          ┐
   ~/src/payments-oauth/      worktree, branch agent/oauth    ├─ all resolve
   ~/src/payments-review/     separate clone                  ┘  the same id

        each carries .wipctl/project.toml:  project_id = "a1b2...e8f90"
                             │
                             ▼
   ~/.local/share/wipctl/projects/payments-acme/plan-repo/     slot name: payments-acme
   $XDG_RUNTIME_DIR/wipctl/payments-acme.lock                  lock named by the slot
```

One project, one plan, one lock, regardless of how many checkouts exist. The lock lives outside the record, so it is never committed, never cloned, and answers no question about the plan. Readers never take it. The preview, the board, the epic resolution, and validation all read the committed state and are never blocked.

A peer is read the same way, at the tree of its current commit. So this plan's reader never blocks that peer's own writer, and never sees a half-written transaction. That is one lock per plan, and this chapter covers one of them. What crosses machines lives in [replication.md](./replication.md), which now covers the peer case too.

## Simulation 1 — two agents, different entries

The ordinary case. Agent A starts one story while agent B starts another.

```text
time   agent A (pid 41291)                    agent B (pid 41337)
─────  ─────────────────────────────────────  ────────────────────────────────────
0.000  $ wipctl start
0.001  resolve id, find plan-repo             ← resolution is a read: no lock
0.002  ┌ LOCK ACQUIRED
0.003  │ HEAD is abc1234                      $ wipctl start
0.004  │ read lanes, journal, questions        resolve id, find plan-repo
0.005  │                                      ┌ LOCK BUSY
0.006  │ head of todo: rate-limit-search      │ stderr: waiting: held by pid 41291
0.009  │ eligible, preflight ok               │
0.013  │ todo.yml -1, doing.yml +1            │
0.015  │ journal/rate-limit-search.tsv +1     │
0.017  │ rank repair, validate                │
0.021  │ commit def5678                       │
0.022  └ LOCK RELEASED                        │
0.023  rate-limit-search  2pt                 └ LOCK ACQUIRED
0.024                                           HEAD is def5678   ← A's work is here
0.025                                           head of todo: audit-headers
0.028                                           preflight ok
0.041                                           commit 9ab0cde
0.042                                         └ LOCK RELEASED
0.043                                         audit-headers  1pt
```

Agent B waited 19 milliseconds and got a different entry. Two facts carry the whole design:

- Line 0.024. B reads inside the lock. Whatever B read a minute ago is advisory. The state B checks against is the state B writes into.
- Line 0.025. Because A already removed `rate-limit-search` from `todo`, the head B sees is the next one. The agents fan out without negotiating.

## Simulation 2 — two agents, the same entry

An agent that takes an entry by id can still lose a race. It loses safely.

```text
  time   agent A                                agent B
  ─────  ─────────────────────────────────────  ────────────────────────────────────
 -60.0   $ wipctl next                          $ wipctl next
         rate-limit-search  2pt                 rate-limit-search  2pt
         ↑ no lock, no gate — both legitimately see the same head

  0.000  $ wipctl move rate-limit-search --to doing
  0.002  ┌ LOCK ACQUIRED
  0.006  │ preflight: it is in todo, ok         $ wipctl move rate-limit-search --to doing
  0.021  │ commit def5678                       ┌ LOCK BUSY (waiting)
  0.022  └ LOCK RELEASED
  0.023                                         └ LOCK ACQUIRED
  0.025                                           HEAD is def5678
  0.026                                           preflight check 4:
                                                  rate-limit-search is already in doing
  0.027                                           exit 2, nothing written
  0.028                                         └ LOCK RELEASED

                                                wipctl: rate-limit-search is already
                                                        in doing
                                                wipctl: another writer took it; run
                                                        'wipctl start' to take the
                                                        next startable entry
```

No corruption, no lost update, no double claim. The move is the claim. There is no separate claim record to keep in agreement with the lanes.

The exit code is deliberate. A usage error means the invocation was wrong, so a caller that retries the identical command retries a mistake. The correct response is to ask again what to start and take a different entry. The take verb does that in one step.

```text
without start (read, then race)          with start (read and take, atomically)

A: next  -> rate-limit-search            A: start -> rate-limit-search       taken
B: next  -> rate-limit-search   same     B: start -> audit-headers           head moved
C: next  -> rate-limit-search   same     C: start -> profile-composition     head moved
A: move rate-limit-search --to doing  ok
B: move rate-limit-search --to doing  exit 2
C: move rate-limit-search --to doing  exit 2
```

## Simulation 3 — an agent that works for half an hour

The case the whole design is shaped around.

```text
  09:00:00.000  agent takes work
                ┌ LOCK   read, preflight, todo -> doing, commit
                └ UNLOCK                                        held 21ms

  09:00:00.021  ── agent reads the story, reads its Reads documents,
                   writes code, runs tests, rewrites, runs tests ──
                   the plan is completely free this whole time
                   other agents take work, close work, capture work

  09:34:12.400  agent hands off
                ┌ LOCK   read at a HEAD 40 commits later, preflight,
                │        doing -> review, commit
                └ UNLOCK                                        held 18ms
```

Two lock holdings totalling 39 milliseconds across 34 minutes of work. The second holding reads a record that changed a great deal while the agent was busy. It preflights against that record rather than against the one it remembers.

## Why there is no queue

A queue answers a question this design does not have: how to fairly schedule a scarce resource. At twenty milliseconds the resource is not scarce. And a queue that is correct must answer all of this:

```text
   an agent enters the queue and its process is killed
       who removes it? every waiter now needs a heartbeat, and something
       alive must reap the dead — a daemon

   a capture of a typo fix is ahead of a close that unblocks four stories
       strict order inverts priority, so the queue needs priorities,
       and now it is a scheduler

   an agent's turn arrives after the agent exited
       a process that ended cannot be woken; either everyone blocks —
       which is the lock again — or a supervisor dispatches, which is a
       second workflow system beside this one

   is the queue per machine or shared
       shared queue state needs coordination to replicate, which is the
       problem the queue was supposed to solve
```

What is promised instead is four things. A waiting writer says who holds the lock. The wait is bounded. Expiry names the holder rather than blocking without end. A dead holder's lock is released rather than inherited.

## What this asks of an agent

```text
read freely                 next, board, epic, validate — no lock, never blocked
take atomically             start — one transaction, id chosen inside the lock
hold nothing while working  the plan is free during implementation
re-check at the write       every precondition is checked against current HEAD
accept a refusal            exit 2 means somebody got there first: ask again
sync when leaving a machine fetch, reconcile, push; never force
```

## A note on messages

Every message in this chapter and in [replication.md](./replication.md) names its resolution, and that is a rule rather than a courtesy ([../specs/SPEC-cli-conventions.md](../specs/SPEC-cli-conventions.md)). A reader who is told only what failed has to guess the next step. A coding agent that guesses takes a wrong action rather than asking a question. So a refusal names the verb or the edit that unblocks it. A wait names who is holding and whether they are alive. A conflict names both sides and the choice between them.
