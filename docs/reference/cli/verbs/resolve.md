# wipctl resolve

Decides a semantic conflict `sync` reported: the same entry transitioned on two machines, and one of those transitions happened while the other did not. A machine cannot know which; a person decides, and this verb records the decision.

## Usage

```text
wipctl resolve <id> --keep <here|remote>
```

Both the id and `--keep` are REQUIRED; a missing or malformed `--keep` is exit 2. `--help` prints usage on stdout, exit 0.

## Contract

- The verb is a writer: inside one lock holding it applies the kept side's transition to the reconciled record, discards the other side's — the discarded event does not enter the journal, because a journal is one unbroken path and only one of the two paths happened — validates, and commits. The next `sync` pushes the result.
- The id MUST name a conflict `sync` reported and has not yet been resolved; any other id is exit 2, naming the conflicts that are open.
- Preflight before the first byte: a refused resolution leaves every file byte-identical.
- Output is one line stating which side was kept — no machine format and no schema, under ADR-0013's one-line exemption.

## Example

```text
$ wipctl resolve rate-limit-the-search-endpoint --keep remote
kept remote: rate-limit-the-search-endpoint todo -> closed (outcome done)
$ wipctl sync
pushed the plan trunk
```

## Diagnostics

```text
nothing is in conflict                                                    exit 2
  wipctl: no conflict is recorded for rate-limit-the-search-endpoint
  wipctl: 'wipctl sync' reports conflicts when they exist; there is
          nothing to resolve
```
