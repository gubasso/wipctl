# Pending fragments

A fragment is one file under `pending/`, written by capture (`wipctl new`), consumed by the drain
(`wipctl land`). It states a delta — where a new entry wants to land — never a copy of anything a
lane file holds. `pending.schema.json` owns its shape.

The `pending/` directory is created on demand; its absence is legal and means nothing is pending.

## File shape

`pending/<id>.yml`, where the id equals the entry's id:

```yaml
lane: backlog
after: null
captured: "2026-08-14T09:41:07Z"
entry:
  id: rate-limit-the-search-endpoint-a7f3
  type: story
  points: 2
  summary: "Callers of the search endpoint are limited per token, and the limit is announced in the response headers rather than discovered by being cut off."
  needs: []
  epic: session-hardening-c4d1
```

- `lane` — REQUIRED. MUST be `backlog` or `todo`. A fragment may claim a planning lane and nothing
  else: landing into a work lane would be a transition, and a transition owes a journal event.
- `after` — REQUIRED, `null` or one id. The position claim: land beneath the named entry, or at
  the bottom of the claimed lane when `null`. The named id MUST be a landed entry; a position MUST
  NOT name an id that exists only as another fragment.
- `captured` — REQUIRED. The instant the capture stated, RFC 3339 UTC with `Z`, second precision.
  The drain's ordering key.
- `entry` — REQUIRED. Carries the lane entry's fields as [lane-file.md](./lane-file.md) defines
  them; `pending.schema.json` restates them so a fragment can be checked single-file, and keeping
  that restatement in agreement with `lane.schema.json` is a review responsibility
  ([../review-checklist.md](../review-checklist.md)). The closing fields (`outcome`, `closed`,
  `succeeded_by`) MUST NOT appear: a capture is never a close.

An unknown field anywhere in a fragment is a typo and a schema failure.

## Standing in the record

A pending entry is part of the record for validation and for nothing else:

- It is never eligible, has no rank, and is invisible to `next` and to the epic verbs.
- Its points do not count toward epic arithmetic.
- A story document claimed by a fragment is not an orphan; a document claimed by nothing is a
  failure.
- An id appearing in both a fragment and a lane file is a duplicate. A fragment whose entry id
  matches no story document is a failure. A fragment's `epic` MUST name an existing epic document
  and every `needs` id MUST exist in some lane or fragment — each a failure otherwise. A `needs`
  target that closed and a stale `after` position are drift, below.

## Drift

Drift is a fact a fragment states that the record has since outgrown. The drain MUST report drift
and MUST NOT repair it:

- a dependency that has since closed — `drift: <id> needs <id>, which closed`
- a position target that is another fragment, changed lane, closed, or vanished — the fragment
  lands at the bottom of its claimed lane, and the report says so.

A landing whose order nobody chose is worse than a landing that stops and reports: rank is a
claim, and only a person makes one.
