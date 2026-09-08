# Replication

One record and two machines, with no lock between them. What a capture writes so that two histories combine, what the sync verb resolves, and what a person decides. [concurrency.md](./concurrency.md) holds the same-machine half, where one lock does the work.

## Capture, and why it is a fragment

`wipctl new` writes a story document and a pending fragment, and touches no lane file. The fragment states a lane and a position, and nothing the lane files already hold. Two rules bound it: a fragment claims a planning lane only, and its position names a landed entry only. [../specs/SPEC-pending-fragment.md](../specs/SPEC-pending-fragment.md) holds both.

The drain later reconciles every fragment into the ranked record, in a derived order every machine agrees on. It reports every drift and repairs none. A drift is a need that closed, or a position target that moved. Rank is a claim, and only a person makes one.

On one machine the lock already serialises captures, so the fragment is not needed for safety there. It is needed for the next section. A capture is a disjoint delta: one new file, named by an id nobody else was minting. Disjoint deltas are what make reconciliation between machines cheap. That choice was made so captures combine rather than conflict, and it pays exactly where no lock can reach.

## A simulation — two machines

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

## A note on ids

Every id in this chapter and in [concurrency.md](./concurrency.md) is a title's slug, such as `rate-limit-search`, and never a random suffix. That is a consequence of everything above. A random suffix is what a mint draws when it cannot see the record it is writing into. That was true when every clone held its own copy. With one live record behind one lock, the mint reads before it writes. A taken id is therefore refused at capture time. The answer is to rephrase the title, or to keep it and choose a qualifying postfix. The grammar and the checks are in [../specs/SPEC-ids.md](../specs/SPEC-ids.md).
