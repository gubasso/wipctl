# Ids

One id names one thing — a story, an epic, or a question — across the whole record, for the whole
life of the record.

## Grammar

```text
<slug>-<uid>
```

- `slug` — the title in lowercase kebab: `[a-z0-9]+(-[a-z0-9]+)*`. Punctuation, capitals, and runs
  of spaces normalise away.
- `uid` — exactly four lowercase hexadecimal characters: `[0-9a-f]{4}`.

Examples: `rate-limit-the-search-endpoint-a7f3`, `session-hardening-c4d1`.

Question ids carry the same grammar behind a fixed prefix: `Q-<slug>-<uid>`, for example
`Q-may-a-position-name-a-fragment-1d55`.

## The id is the filename

A story's document is `stories/<id>.md`; an epic's is `epics/<id>.md`; a journal is
`journal/<id>.tsv`; a fragment is `pending/<id>.yml`. The filename stem and the id are one fact,
checked against each other in both directions. A story or epic document's title line MUST repeat
the id: `# <id> — <short title>` — the same pattern as `lane:` repeating the lane file's basename,
so a document read in isolation still says what it is.

## Opacity

A consumer MUST NOT parse an id into parts. The record library, the checker, the schemas, and
shell completion all treat the id as one token. The id carries no order, no date, no rank, and no
meaning in its interior; anything that wants the title reads the document. The one stated
exemption is the drain's tie-break, which reads the trailing uid when two captures state the same
instant ([../cli/verbs/land.md](../cli/verbs/land.md)).

## Allocation is unobservable

A mint reads nothing, locks nothing, and waits for nothing: it slugs the title and draws four hex
characters from a source of randomness. Two sessions in two clones with no shared state cannot ask
each other what the next id is, because nobody asks anything. A collision is surfaced, never
prevented by the mint: claimants that coexist in one record are named by the id-uniqueness check,
while two clones capturing one id collide on the same paths and surface as a version-control
add/add conflict, resolved by keeping one capture's document and fragment together and re-minting
the other whole (see
[../../explanation/concurrent-capture.md](../../explanation/concurrent-capture.md)). On collision
the uid MUST be re-minted, never incremented; an incremented uid is a counter.

A counter MUST NOT be used anywhere allocation is concurrent. The one excluded case is the
decision record sequence (`ADR-NNNN`), which is allocated serially by one person merging one
decision at a time.

## Burned ids

A deleted entry leaves a tombstone: a journal whose last event targets `deleted` (see
[transition-journal.md](./transition-journal.md)). The id is burned — an entry, epic, or fragment
claiming an id a tombstone holds is a validation failure naming both sides. Burning is what makes
"one id names one thing" hold across the record's whole life rather than only its present.
