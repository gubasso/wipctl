# Replication Specification

<!--TOC-->

- [Purpose](#purpose)
- [The four reconciliation cases](#the-four-reconciliation-cases)
- [The batch run](#the-batch-run)
- [When the transport cannot finish](#when-the-transport-cannot-finish)
- [Requirements](#requirements)
  - [`sync:the-batch-runs-in-four-passes` — The batch runs in four passes](#syncthe-batch-runs-in-four-passes--the-batch-runs-in-four-passes)
  - [`sync:the-batch-publishes-only-what-it-verified` — The batch publishes only what it verified](#syncthe-batch-publishes-only-what-it-verified--the-batch-publishes-only-what-it-verified)
  - [`sync:the-batch-holds-one-lock-at-a-time` — The batch holds one lock at a time](#syncthe-batch-holds-one-lock-at-a-time--the-batch-holds-one-lock-at-a-time)
  - [`sync:the-aggregate-exit-code-has-a-precedence` — The aggregate exit code has a precedence](#syncthe-aggregate-exit-code-has-a-precedence--the-aggregate-exit-code-has-a-precedence)
  - [`sync:reconciliation-is-semantic-never-textual` — Reconciliation is semantic, never textual](#syncreconciliation-is-semantic-never-textual--reconciliation-is-semantic-never-textual)
  - [`sync:two-minted-identities-are-reported` — Two minted identities are reported, never merged](#synctwo-minted-identities-are-reported--two-minted-identities-are-reported-never-merged)
  - [`sync:the-reconciliation-runs-under-the-lock` — The reconciliation runs under the lock](#syncthe-reconciliation-runs-under-the-lock--the-reconciliation-runs-under-the-lock)
  - [`sync:a-conflict-stops-the-push` — A conflict stops the push](#synca-conflict-stops-the-push--a-conflict-stops-the-push)
  - [`sync:a-conflict-names-both-sides` — A conflict names both sides](#synca-conflict-names-both-sides--a-conflict-names-both-sides)
  - [`sync:the-push-is-never-forced` — The push is never forced](#syncthe-push-is-never-forced--the-push-is-never-forced)
  - [`sync:a-rejected-push-loses-nothing` — A rejected push loses nothing](#synca-rejected-push-loses-nothing--a-rejected-push-loses-nothing)
  - [`sync:nothing-to-do-is-said-and-succeeds` — Nothing to do is said and succeeds](#syncnothing-to-do-is-said-and-succeeds--nothing-to-do-is-said-and-succeeds)
  - [`sync:a-person-decides-a-conflict` — A person decides a conflict](#synca-person-decides-a-conflict--a-person-decides-a-conflict)
  - [`sync:the-discarded-event-never-enters-the-journal` — The discarded event never enters the journal](#syncthe-discarded-event-never-enters-the-journal--the-discarded-event-never-enters-the-journal)
  - [`sync:a-resolution-names-an-open-conflict` — A resolution names an open conflict](#synca-resolution-names-an-open-conflict--a-resolution-names-an-open-conflict)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

Replication between machines, and the decision a person makes when two machines disagree. No lock spans machines and none is pretended. The boundary runs at the decision. This domain owns the fetch, the reconciliation, the push, and the recorded resolution. The ids domain owns what a collision recovery does to a name.

## The four reconciliation cases

| Case                                        | Outcome                                                                      |
| ------------------------------------------- | ---------------------------------------------------------------------------- |
| a capture on one side                       | merges by construction; the files are named by an id nobody else was minting |
| different entries transitioned on each side | combined; different entry blocks and different journal files                 |
| the same entry transitioned on both sides   | a semantic conflict, decided by a person                                     |
| one slug minted on both sides               | an id collision, recovered by renaming the losing capture                    |
| one plan, two minted global identities      | a semantic conflict, decided by a person, because no fact orders the mints   |

## The batch run

```text
wipctl sync [--all] [--json]
```

Without `--all` the verb is unchanged: this plan's trunk, fetched, reconciled, validated, pushed fast-forward. `sync.schema.json` is unchanged with it.

With `--all` the run is not that contract repeated, and the reason comes before the passes. The single-plan contract validates before it pushes. Running it plan by plan validates the first plan against peers this same run has not fetched yet. That plan then fails against state the run is about to replace, and nothing revisits it. Peer graphs have cycles, so no ordering of slots fixes that. The batch is four named passes instead.

```text
discover    resolve this plan, walk the declared set, and deduplicate by
            plan_uid, so a plan two rows reach is visited once. The
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

The reconcile pass loops because reconciling a peer can bring in a row this run never saw. That plan joins the declared set. Without the loop, a plan already in a slot is verified at a revision the run never fetched. The loop ends because the closure is finite and the visit list only grows.

Publishing checks the recorded revision because verification releases every lock before it runs. A local writer can commit into a slot in that window. A slot whose trunk moved is reported and not pushed, so the run publishes no commit that verification never read.

Each slot is independent in the reconcile and publish passes. A failure in one slot stops that slot and nothing else, and the run continues so the summary names every slot's outcome. A slot whose lock is still held when the bounded wait expires is reported as locked.

A slot's outcome is one of five: up to date, advanced, reconciled, failed, or locked. The single-plan contract holds one lock from the fetch to the push, and the batch cannot, because its verify pass reads every slot at once. So the batch states what replaces that hold: one lock at a time, and a push only of the revision verification read.

The verb writes into a peer's slot only what replication writes into any slot: a fetch, a fast-forward, a reconciliation commit when both sides moved, and a push. It never edits a peer's record and never settles a peer's semantic conflict, which stays with that plan's owners and their own resolution verb.

```text
$ wipctl sync --all
payments-acme      remote advanced; fetched 2 commits; nothing to push
platform-acme      up to date
profile-acme       reconciled 1 disjoint capture; pushed the plan trunk
verified the closure: 63 entries, 3 plans, ok
```

`--json` under `--all` conforms to `sync-all.schema.json` and not to `sync.schema.json`. One is an object about one plan and the other is a run over many. One schema for both makes every field of the smaller one optional.

## When the transport cannot finish

Reconciliation here is semantic, and its conflicts have their own diagnostics below. The other class is the transport itself. It cannot complete a fetch, a fast-forward, or a push, or the slot is not as the tool left it. The messages domain owns that class, because relaying another tool's output is a rule about what a message says and not about replication.

## Requirements

### `sync:the-batch-runs-in-four-passes` — The batch runs in four passes

Under the batch flag, replication MUST reconcile until the declared set stops growing, and MUST verify the closure once after that.

#### Scenario: Reconciling one peer brings in a plan the run never visited

- GIVEN a peer whose table gains a row for a plan already in a slot
- WHEN the reconcile pass ends
- THEN discovery runs again and that slot is reconciled too. A plan verified at a revision the run never fetched is the failure these passes exist to prevent

Verify: `cargo nextest run --test sync`

### `sync:the-batch-publishes-only-what-it-verified` — The batch publishes only what it verified

Under the batch flag, replication MUST record the revision it verified each slot at, and MUST NOT push a slot that moved since.

#### Scenario: A local writer commits between verification and publication

- GIVEN a slot that gained a commit after the verify pass released its lock
- WHEN its publish turn arrives
- THEN it is reported and not pushed. Verification runs with no lock held, and that window otherwise allows a push of work nothing verified

Verify: `cargo nextest run --test sync`

### `sync:the-batch-holds-one-lock-at-a-time` — The batch holds one lock at a time

The batch MUST hold one writer lock at a time, and MUST report a slot whose bounded wait expires as locked while the run continues.

#### Scenario: Another agent is writing one plan of the closure

- GIVEN a slot whose lock is held throughout the run
- WHEN the reconcile pass reaches it
- THEN that slot alone is reported and the rest of the run proceeds, and no ordering of slots can deadlock against that agent

Verify: `cargo nextest run --test writer_guarantees`

### `sync:the-aggregate-exit-code-has-a-precedence` — The aggregate exit code has a precedence

The batch MUST exit 1 where any slot failed, 3 where a lock timeout was the only fault, and 0 where every slot succeeded.

#### Scenario: One slot fails a check and another times out

- GIVEN a run with both outcomes
- WHEN it ends
- THEN it exits 1, because a lock timeout is a retry and a failed check is a decision

Verify: `cargo nextest run --test sync`

### `sync:reconciliation-is-semantic-never-textual` — Reconciliation is semantic, never textual

The implementation MUST reconcile the local and remote histories semantically, and MUST NOT resolve a difference by whichever line won a textual merge.

#### Scenario: Two machines transitioned different entries

- GIVEN two disjoint sets of changes
- WHEN reconciliation runs
- THEN both are combined, because the record's shape says they cannot conflict and a line-based merge cannot see that

Verify: `cargo nextest run --test sync`

### `sync:two-minted-identities-are-reported` — Two minted identities are reported, never merged

Where two replicas of one plan each minted a `plan_uid`, replication MUST report both values and MUST name the edit that resolves it.

#### Scenario: Two machines upgrade one record before either replicates

- GIVEN two replicas that each ran the upgrade form
- WHEN the next replication reads both
- THEN both values are named and neither is chosen. A uid carries no time and no order, and history is not a store this method reads, so nothing on disk orders the two mints

The resolution is an operator's edit of `config.toml` on the losing side, committed by hand. No verb performs it, because no verb changes a uid.

That settles the two replicas and nothing beyond them. A uid another plan already wrote into its own peer table stays written there, and no verb changes a row's uid. So this is cheap while the plan is young and expensive once other plans name it. The message says which case the operator is in.

Verify: `cargo nextest run --test sync`

### `sync:the-reconciliation-runs-under-the-lock` — The reconciliation runs under the lock

Without the batch flag, replication MUST fetch, reconcile, validate, commit, and push, with the reconciliation inside one lock holding.

#### Scenario: A local capture runs during a sync

- GIVEN a concurrent local mutation
- WHEN reconciliation holds the lock
- THEN the two cannot interleave, because a reconciliation reading a record another writer is changing reconciles a state that never existed. The batch cannot hold one lock across its passes, so it names its own guarantees instead: one lock at a time, and a push only of what verification read

Verify: `cargo nextest run --test sync`

### `sync:a-conflict-stops-the-push` — A conflict stops the push

A conflict MUST stop the push and MUST leave the local trunk unmodified.

#### Scenario: One entry moved on both machines

- GIVEN a semantic conflict
- WHEN reconciliation finds it
- THEN nothing is pushed and everything already committed locally stays committed, because a push over an undecided conflict publishes a guess

Verify: `cargo nextest run --test sync`

### `sync:a-conflict-names-both-sides` — A conflict names both sides

A reported conflict MUST name both sides and the resolution.

#### Scenario: An entry closed on one machine and started on the other

- GIVEN both transitions with their instants
- WHEN the conflict is reported
- THEN both are shown with the decision to make, because one of them happened and the other did not

Verify: `cargo nextest run --test sync`

### `sync:the-push-is-never-forced` — The push is never forced

The implementation MUST push fast-forward only and MUST NOT force-push.

#### Scenario: The remote has advanced

- GIVEN a remote ahead of the local trunk
- WHEN the push is attempted
- THEN it is rejected rather than forced, because plan history is append-only and another replica's work is not this machine's to discard

Verify: `cargo nextest run --test sync`

### `sync:a-rejected-push-loses-nothing` — A rejected push loses nothing

Where a push is rejected, the implementation MUST report that the local work is committed and MUST name fetching and reconciling again as the resolution.

#### Scenario: Another replica advanced mid-sync

- GIVEN a push rejected by the remote
- WHEN the message prints
- THEN it says the work is not lost and names the re-run, because the reader's first fear is that it was

Verify: `cargo nextest run --test sync`

### `sync:nothing-to-do-is-said-and-succeeds` — Nothing to do is said and succeeds

With no remote changes and nothing local to push, the implementation MUST say so and succeed.

#### Scenario: A sync runs twice

- GIVEN a replica already in step with the remote
- WHEN the second run completes
- THEN it says there is nothing to do, because a silent success is indistinguishable from a hang

Verify: `cargo nextest run --test sync`

### `sync:a-person-decides-a-conflict` — A person decides a conflict

A semantic conflict MUST be decided by a person and recorded by the resolution verb, inside one lock holding.

#### Scenario: A conflict is resolved and pushed

- GIVEN a decision naming the side to keep
- WHEN the resolution commits
- THEN the next replication pushes the result, because a machine cannot know which of two transitions happened

Verify: `cargo nextest run --test sync`

### `sync:the-discarded-event-never-enters-the-journal` — The discarded event never enters the journal

The resolution MUST discard the other side's transition and MUST NOT write its event to the journal.

#### Scenario: The kept side moved the entry elsewhere

- GIVEN two competing transitions for one entry
- WHEN one is kept
- THEN only its event is in the journal, because a journal is one unbroken path and only one of the two paths happened

Verify: `cargo nextest run --test sync`

### `sync:a-resolution-names-an-open-conflict` — A resolution names an open conflict

The resolution verb MUST require an id naming an open conflict, and MUST treat any other id as a usage error naming the open conflicts.

#### Scenario: An id with no conflict is passed

- GIVEN an entry nothing reported
- WHEN the resolution runs
- THEN it is a usage error listing what is open, because there is nothing to decide

Verify: `cargo nextest run --test verb_contracts`

## Unenforced rules

| Rule                               | Why no command decides it                                                                          |
| ---------------------------------- | -------------------------------------------------------------------------------------------------- |
| `sync:a-person-decides-a-conflict` | Whether the kept side is the one that actually happened is the judgment the verb exists to record. |

A collision recovery renames the losing capture under a rephrased title. That rename moves the capture's document and fragment together, which keeps the pair intact. It is the one rename that leaves no tombstone.
