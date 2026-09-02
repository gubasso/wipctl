# wipctl new

Capture: mints the identity, writes the story document and the pending fragment, touches no lane file, and commits the capture to the plan trunk.

## Usage

```text
wipctl new <type> "<title>" [--id <id>] [--epic <id>] [--points <n>] [--lane <backlog|todo>]
```

- `<type>` — `story`, `spike`, or `chore`.
- `<title>` — the story's title; the id is its slug.
- `--id <id>` — an explicit id in place of the derived one: the qualifying-postfix case, where a title is right as written and its slug is taken. The explicit id MUST agree with the title under the invariant, and one that cannot is exit 2; it then passes every mint check against the record, whose refusals are exit 1 — a taken or burned id, and a postfix whose base slug is free, since a postfix MUST qualify an id the record holds. The tool never appends a postfix on the operator's behalf, because a qualifying postfix is a naming judgment ([../../record/ids.md](../../record/ids.md)).
- `--lane` defaults to `backlog`; only the two planning lanes are accepted.

## Contract

- Mint: the id is `slugify(title)` — or the `--id` value — checked under the lock against the live record and every tombstone. A taken or burned id MUST be refused before the first byte, naming the holder and the resolution:

  ```text
  wipctl: id rate-limit-the-search-endpoint is taken by an entry in todo
  wipctl: rephrase the title, or pass --id rate-limit-the-search-endpoint-v2
  ```

  ```text
  wipctl: id fix-the-parser is held by a tombstone: deleted 2026-08-30
  wipctl: that id is burned for the life of the record; rephrase the title
  ```

  An explicit id owes two more, one per class:

  ```text
  the id cannot agree with the title                                      exit 2
    wipctl: --id profile-composition does not agree with the title
            "Rate limit the search endpoint"
    wipctl: an id is the title's slug, plus at most a qualifying postfix;
            pass rate-limit-the-search-endpoint, or a postfix of it

  the postfix qualifies nothing                                           exit 1
    wipctl: id rate-limit-the-search-endpoint-v2 carries a postfix, but
            rate-limit-the-search-endpoint is free
    wipctl: a postfix disambiguates an id the record holds; capture under
            rate-limit-the-search-endpoint, or choose a distinct title
  ```

- The verb is a writer: mint, write, and commit (`plan: capture <id>`) are one transaction inside one lock holding. It MUST write exactly two files and print exactly their two paths on stdout, absolute, so the operator can open them:

  ```text
  ~/.local/share/wipctl/projects/payments-acme/plan-repo/stories/rate-limit-the-search-endpoint.md
  ~/.local/share/wipctl/projects/payments-acme/plan-repo/pending/rate-limit-the-search-endpoint.yml
  ```

  The document comes from the shipped story template and MUST pass the heading-shape gate as written. The fragment carries the claimed lane, `after: null`, the `captured` instant stamped from the system clock — the only clock the capture-and-drain machinery reads — and the entry fields given.
- A missing `--points` MUST be written absent rather than guessed at a value. The entry `summary` is not an input and MUST likewise be written absent — no summary a verb could derive from a title satisfies the lane contract. A fragment carrying neither is complete as far as the fragment schema is concerned, precisely so that the capture's own commit passes the gates it must run ([../../record/pending-fragment.md](../../record/pending-fragment.md)); the operator completes the fragment before landing it, and the drain refuses an incomplete one naming the file and the field (see [land.md](./land.md)).
- Write discipline: every destination MUST be checked before the first byte; an existing path MUST be reported and left alone; no overwrite flag exists. The `pending/` directory MUST be created when absent. A `--epic` naming no document MUST be refused before any write.

## Out of scope

Landing (that is `land`), editing existing files, inferring type, epic, or points from the title.
