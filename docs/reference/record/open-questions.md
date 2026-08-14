# Open questions

`open-questions.md` in the zone root holds the questions that block named work. A question earns its place by blocking; one that blocks nothing belongs in a drafts workspace, and the checker says so.

## Section shape

One `##` section per question:

```markdown
## Q-may-a-position-name-a-fragment-1d55 — May a fragment's position name another fragment?

Raised: 2026-08-14

Blocks: land-the-capture-queue-9b21, drain-reports-drift-40ce — the drain's ordering rule depends on the answer.

Exit: a decision record, measured against a worked two-session capture.
```

- The heading carries the question id (`Q-<slug>-<uid>`, see [ids.md](./ids.md)) and the question.
- `Raised:` — the date the question was opened.
- `Blocks:` — REQUIRED on every section. Grammar: a comma-separated list of at least one existing entry id, then optionally an em dash and the reason: `Blocks: <id>[, <id>…] — <why>`.
- `Exit:` — what closes the question: a decision record, a story revision, or a measurement.

## Rules the checker holds

- Every question section MUST have a `Blocks:` line. A question that blocks nothing is a failure.
- Every id named on a `Blocks:` line MUST exist in some lane.
- A blocked id MUST NOT already be in `doing` or `review` — a question cannot block work already underway; either the question is stale or the entry moved illegally.
- At least one blocked id MUST still be open. A question whose every blocked entry is closed is stale and reported as such.

## What blocking means

Question-blocking is one of the two edges the eligibility predicate reads (the other is `needs`). A blocked entry MUST NOT enter `doing` or `review`, and ranking rule R1 keeps it below eligible entries in `todo`. Blocked is always derived from these edges — never a lane, never a field.
