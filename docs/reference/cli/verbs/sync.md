# wipctl sync

Replication between machines: fetch, reconcile, validate, fast-forward push. No lock spans machines and none is pretended; the model is [../../explanation/concurrency.md](../../../explanation/concurrency.md), and this page is the contract.

## Usage

```text
wipctl sync [--json]
```

## Contract

- The verb fetches the plan trunk from the remote, reconciles the local and remote histories, validates the reconciled record, commits the reconciliation, and pushes fast-forward. It is a writer: the reconciliation runs inside one lock holding, so a concurrent local mutation cannot interleave with it.
- Reconciliation MUST be semantic, never textual:
  - A capture on one side merges with anything on the other by construction — the capture wrote files named by an id nobody else was minting.
  - Different entries transitioned on each side: different entry blocks, different journal files. Combined.
  - The same entry transitioned on both sides: a semantic conflict, reported naming both sides, resolved by a person through `resolve` — never by whichever line won a textual merge.
  - One slug minted on both sides: an id collision, reported naming both paths; the recovery is to rephrase one title and re-capture it, keeping that capture's document and fragment together as a pair.
- A conflict MUST stop the push and MUST leave the local trunk unmodified; everything already committed locally stays committed. Exit 1, with each conflict reported.
- A rejected push means another replica advanced meanwhile: fetch and reconcile again. The verb MUST NOT force-push, ever — plan history is append-only.
- With nothing to do — no remote changes, nothing local to push — the verb says so and exits 0.
- `--json`: the report as an object conforming to `sync.schema.json` — fetched, reconciled, pushed, and any conflicts — on stdout.

## Example

```text
$ wipctl sync
remote advanced; fetched 2 commits
reconciled 1 disjoint capture
pushed the plan trunk
```

## Diagnostics

```text
no remote is configured                                                   exit 1
  wipctl: the plan repository has no remote
  wipctl: add one to host the plan — any forge, private or public — then
          re-run 'wipctl sync'

the push was rejected                                                     exit 1
  wipctl: push rejected: the remote advanced since the last sync
  wipctl: your work is committed locally and is not lost; run
          'wipctl sync' to fetch, reconcile, and push again

a semantic conflict between machines                                      exit 1
  wipctl: conflict: rate-limit-the-search-endpoint moved on both sides
  wipctl:   here    todo -> doing    2026-09-02T09:14:02Z
  wipctl:   remote  todo -> closed   2026-09-02T09:11:40Z  outcome done
  wipctl: one of these happened and the other did not; decide which,
          then run 'wipctl resolve <id> --keep here|remote'

an id collision arriving from another machine                             exit 1
  wipctl: id rate-limit-the-search-endpoint is claimed twice
  wipctl:   here    stories/rate-limit-the-search-endpoint.md
  wipctl:   remote  stories/rate-limit-the-search-endpoint.md
  wipctl: two captures minted the same slug; rephrase one title and
          re-capture it, keeping its document and fragment together
```
