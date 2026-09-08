# Concurrency

How many agents share one plan without waiting on each other and without losing each other's work. One record per project, a lock measured in milliseconds, and reconciliation between machines that a person can always overrule.

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

        each carries .wipctl.toml:  project_id = "payments-acme"
                             │
                             ▼
   ~/.local/share/wipctl/projects/payments-acme/plan-repo/     the one live record
   $XDG_RUNTIME_DIR/wipctl/payments-acme.lock                  the one writer lock
```

One project, one plan, one lock, regardless of how many checkouts exist. The lock lives outside the record, so it is never committed, never cloned, and answers no question about the plan. Readers never take it. The preview, the board, the epic resolution, and validation all read the committed state and are never blocked.

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

  09:00:00.021  ── agent reads the story, reads its Reads sources,
                   writes code, runs tests, rewrites, runs tests ──
                   the plan is completely free this whole time
                   other agents take work, close work, capture work

  09:34:12.400  agent hands off
                ┌ LOCK   read at a HEAD 40 commits later, preflight,
                │        doing -> review, commit
                └ UNLOCK                                        held 18ms
```

Two lock holdings totalling 39 milliseconds across 34 minutes of work. The second holding reads a record that changed a great deal while the agent was busy. It preflights against that record rather than against the one it remembers.

## Capture, and why it is a fragment

`wipctl new` writes a story document and a pending fragment, and touches no lane file. The fragment states a lane and a position, and nothing the lane files already hold. Two rules bound it: a fragment claims a planning lane only, and its position names a landed entry only. [../specs/SPEC-pending-fragment.md](../specs/SPEC-pending-fragment.md) holds both.

The drain later reconciles every fragment into the ranked record, in a derived order every machine agrees on. It reports every drift and repairs none. A drift is a need that closed, or a position target that moved. Rank is a claim, and only a person makes one.

On one machine the lock already serialises captures, so the fragment is not needed for safety there. It is needed for the next section. A capture is a disjoint delta: one new file, named by an id nobody else was minting. Disjoint deltas are what make reconciliation between machines cheap. That choice was made so captures combine rather than conflict, and it pays exactly where no lock can reach.

## Simulation 4 — two machines

A lock cannot span machines, and none is pretended. The laptop and the desktop each hold a clone of the plan repository with its own lock, and they can genuinely diverge.

```text
laptop    ... abc1234 ── def5678 ── 11aa22b   capture retry-on-timeout
desktop   ... abc1234 ── def5678 ── 77ff88e   move audit-headers to review
                                    ↑ two children of def5678
```

`wipctl sync` resolves it: fetch, reconcile, validate, fast-forward push. What reconciliation has to do depends on what actually happened:

```text
   a capture on one side, anything on the other
       nothing to resolve. The capture wrote two files named by an id nobody
       else was minting, so the histories combine by construction.

   different entries transitioned on each side
       different entry blocks, different journal files. Combine.

   the same entry transitioned on both sides
       a semantic conflict, reported naming both sides, decided by a person
       through resolve. Never by whichever line won a textual merge.
```

The third case is rare and the first is overwhelmingly common. That is why capture writes a disjoint fragment instead of editing a shared ranked lane file. One case is new under the slug-only grammar. Two machines can mint the same slug, and that too surfaces at replication as a conflict naming both sides. The recovery is to rephrase one title, which a person usually wants to do anyway.

## The two layers, side by side

```text
               same machine                    different machines
───────────    ─────────────────────────────   ──────────────────────────────
mechanism      exclusive lock, about 20 ms     fetch, reconcile, push
prevents       a lost update at the read        divergent trunks
               that decides it
conflict is    exit 2, a refused preflight,     a semantic conflict report
               every file byte-identical
frequency      milliseconds of contention       whenever two machines worked
```

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

## A note on ids

Every id on this page is a title's slug, such as `rate-limit-search`, and never a random suffix. That is a consequence of everything above. A random suffix is what a mint draws when it cannot see the record it is writing into. That was true when every clone held its own copy. With one live record behind one lock, the mint reads before it writes. A taken id is therefore refused at capture time. The answer is to rephrase the title, or to keep it and choose a qualifying postfix. The grammar and the checks are in [../specs/SPEC-ids.md](../specs/SPEC-ids.md).

## A note on messages

Every message on this page names its resolution, and that is a rule rather than a courtesy ([../specs/SPEC-cli-conventions.md](../specs/SPEC-cli-conventions.md)). A reader who is told only what failed has to guess the next step. A coding agent that guesses takes a wrong action rather than asking a question. So a refusal names the verb or the edit that unblocks it. A wait names who is holding and whether they are alive. A conflict names both sides and the choice between them.
