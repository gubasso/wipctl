# Verb mechanics

The ordered steps and the write sets of the writing verbs: the move preflight, the peer walk, the reconciliation cases, the batch run, the rename write set, and the slot repair. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## The move preflight, in order

1. The record already validates.
2. The destination is a real lane.
3. The id is in some lane.
4. The entry is not already in the destination.
5. The closing flags are consistent. An outcome is required by and only by a move to the closed lane. A successor is required by and only by a reshaped outcome.
6. An entry entering a work lane has every dependency closed and no blocking question. A dependency that names a peer is read in that peer's record, and step 6 needed no widening to reach it: a peer's entry is an entry and its lane is a lane.
7. A supplied session reference names a work lane as its destination. A move to a planning or closed lane carries none, which holds a session claim to the discipline step 5 holds a closing flag to.

Rules that bind this section: `transitions:the-preflight-runs-entire-and-in-order`, `transitions:a-closing-flag-is-required-by-its-case`, `ranking:a-work-lane-entry-is-unblocked`, `transitions:a-session-claim-changes-with-the-move`.

## Filling the slots

```text
wipctl attach --peers
```

The verb resolves this project's plan first, as every verb does, then reads its table.

For each row absent from this machine, the verb clones from the first url that answers and verifies the plan identity.

```text
1. clone the url into a temporary directory outside every slot
2. read the clone's .wipctl/plan.toml and verify plan_id and project_id
3. take this plan's writer lock, and re-read the peers section under it
4. promote the temporary directory into the slot, create-only: a slot
   that appeared meanwhile is a lost race, and the loser removes its
   own temporary clone and reports the winner
5. release the lock; the temporary directory is removed on every path,
   including every failure path
```

The walk is transitive and generous. After a peer is attached, its own table is read and every row of it is attached the same way. The walk ends when every plan in the declared set sits in a slot.

Rules that bind this section: `peer-attachment:the-network-read-is-outside-the-lock`, `peer-attachment:the-walk-fills-the-declared-set`.

## The five reconciliation cases

| Case                                        | Outcome                                                                      |
| ------------------------------------------- | ---------------------------------------------------------------------------- |
| a capture on one side                       | merges by construction; the files are named by an id nobody else was minting |
| different entries transitioned on each side | combined; different entry blocks and different journal files                 |
| the same entry transitioned on both sides   | a semantic conflict, decided by a person                                     |
| one slug minted on both sides               | an id collision, recovered by renaming the losing capture                    |
| one plan, two minted global identities      | a semantic conflict, decided by a person, because no fact orders the mints   |

Rules that bind this section: `sync:reconciliation-is-semantic-never-textual`, `sync:a-person-decides-a-conflict`, `sync:two-minted-identities-are-reported`.

## The batch run

```text
wipctl sync [--all] [--json]
```

Without `--all` the verb is unchanged: this plan's trunk, fetched, reconciled, validated, pushed fast-forward. `sync.schema.json` is unchanged with it.

With `--all` the run is not that contract repeated, and the [replication](../explanation/replication.md) explanation gives the reason.

```text
discover    resolve this plan, walk the declared set, and deduplicate by
            plan_id, so a plan two rows reach is visited once. The
            output of this pass is the visit list, in a stated order:
            this plan first, then each plan in the order it was first
            declared.

reconcile   for each slot in turn: take that slot's writer lock, fetch,
            fast-forward or reconcile, commit, release. No validation
            here, and no push. One lock at a time. Then re-run discover
            and repeat, until the visit list stops growing.

verify      with every slot reconciled and no lock held, validate the
            closure once, recording the revision each slot was read at.
            This is the answer the run reports, and it is an answer
            about the state on disk after the reconcile pass.

publish     for each slot that gained a commit, in turn: take that
            slot's lock, confirm the trunk still stands at the recorded
            revision, push fast-forward, release. A slot that failed
            verification is not pushed, and the summary says so.
```

Each slot is independent in the reconcile and publish passes. A failure in one slot stops that slot and nothing else, and the run continues so the summary names every slot's outcome. A slot whose lock is still held when the bounded wait expires is reported as locked.

The verb writes into a peer's slot only what replication writes into any slot: a fetch, a fast-forward, a reconciliation commit when both sides moved, and a push. It never edits a peer's record and never settles a peer's semantic conflict, which stays with that plan's owners and their own resolution verb.

```text
$ wipctl sync --all
payments-acme      remote advanced; fetched 2 commits; nothing to push
platform-acme      up to date
profile-acme       reconciled 1 disjoint capture; pushed the plan trunk
verified the closure: 63 entries, 3 plans, ok
```

`--json` under `--all` conforms to `sync-all.schema.json` and not to `sync.schema.json`.

Rules that bind this section: `sync:the-batch-runs-in-four-passes`, `sync:the-reconciliation-runs-under-the-lock`, `cli-conventions:a-machine-format-is-verb-local`, `sync:a-batch-reports-each-slot-in-a-stated-order`, `sync:the-batch-publishes-only-what-it-verified`, `sync:the-batch-holds-one-lock-at-a-time`, `sync:the-aggregate-exit-code-has-a-precedence`, `peer-attachment:the-walk-writes-into-no-peer`, `sync:a-person-decides-a-conflict`.

## The rename write set

```text
the lane entry's id, when an entry is renamed
the story document and its title line
the story's sibling directory, when one exists
the epic or initiative document, when one is renamed
the question's heading, when a question is renamed
the journal, history travelling with the entry
the pending fragment, when the entry is still a fragment
every dependency entry naming the old id, in every lane
every successor field naming the old id
every blocking line naming the old id
every epic field naming the old id, when an epic is renamed
every initiative section naming the old id, when an initiative is renamed
```

A question has no lane entry, no journal, and no fragment, so its rename touches its heading and its inbound references alone.

Rules that bind this section: `rename:the-write-set-covers-every-reference`.

## The slot repair

```text
wipctl fix --registry [<name>]
```

Without a name, the repair derives the project slug and escalates it until free. With a name, it uses that name and refuses an occupied destination.

The repair takes the locks for the current and new slot names in lexical order. It then moves three filesystem parts in order:

```text
1. move the slot
2. move the state directory
3. remove the old cache directory
```

Each boundary is resumable. A later run finds the moved slot, then completes any state move and cache removal left under the old name.

The repair writes no project file because a peer slot has no host repository. It commits and replicates nothing.

```text
wipctl: slot renamed: payments -> payments-ben
wipctl: state moved:  <state dir>/projects/payments-ben
wipctl: cache removed: <cache dir>/projects/payments
```

Rules that bind this section: `slot-naming:a-taken-name-escalates`, `repair:an-explicit-slot-name-is-free`, `repair:a-slot-repair-locks-both-names`, `repair:a-slot-repair-moves-local-state-in-order`, `repair:the-repair-is-idempotent`, `repair:a-slot-repair-is-machine-local`.
