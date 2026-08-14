# wipctl new

Capture: mints the identity, writes the story document and the pending fragment, touches no lane
file. Two sessions in two clones capture at the same moment and cannot collide on purpose,
because nobody asks what the next id is.

## Usage

```text
wipctl new <type> "<title>" [--epic <id>] [--points <n>] [--lane <backlog|todo>] [<plan-dir>]
```

- `<type>` — `story`, `spike`, or `chore`.
- `<title>` — the story's title; the slug is derived from it.
- `--lane` defaults to `backlog`; only the two planning lanes are accepted.

## Contract

- Mint: the title's slug (lowercase kebab; punctuation, capitals, and runs of spaces normalised)
  plus four lowercase hex characters from a source of randomness. A uid collision with an
  existing id MUST be re-minted, never incremented — an incremented uid is a counter.
- The verb MUST write exactly two files and print exactly their two paths on stdout:

  ```text
  docs/plan/stories/rate-limit-the-search-endpoint-a7f3.md
  docs/plan/pending/rate-limit-the-search-endpoint-a7f3.yml
  ```

  The document comes from the shipped story template and MUST pass the heading-shape gate as
  written. The fragment carries the claimed lane, `after: null`, the `captured` instant stamped
  from the system clock — the only clock the capture-and-drain machinery reads — and the entry
  fields given.
- A missing `--points` MUST be written absent, so the schema tells the operator rather than the
  verb guessing a value. The entry `summary` is not an input and MUST likewise be written absent —
  no summary a verb could derive from a title satisfies the lane contract. The operator completes
  the fragment before landing it; the schema half reports each absent field until then, and the
  drain refuses an incomplete fragment (see [land.md](./land.md)).
- Write discipline inherited from `init`: every destination MUST be checked before the first
  byte; an existing path MUST be reported and left alone; no overwrite flag exists. The
  `pending/` directory MUST be created when absent. A `--epic` naming no document MUST be refused
  before any write.
- The verb MUST NOT invoke version control, ever.

## Out of scope

Landing (that is `land`), editing existing files, inferring type, epic, or points from the title.
