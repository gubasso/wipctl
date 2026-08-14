# The check catalog

Every cross-file rule the checker holds, grouped by subject. Class is failure unless stated. Single-file shape is the schemas' job and is not repeated here.

## Config

- `.wipctl.toml` MUST exist at or above the zone.
- `plan_dir`, `iteration.start`, and `iteration.length_days` MUST each be present; each absent one is named individually.
- The declared `plan_dir` MUST resolve to the very directory being checked, compared after path resolution.

## Lane parse

- All five lane files MUST exist, even when empty.
- `lane:` in each file MUST equal the file's basename.
- A field MUST NOT appear outside an entry; a line MUST NOT fall outside the canonical subset; a field MUST NOT be unknown (the schema owns the field list); `needs` and `tags` MUST be one-line flow sequences.
- Ids MUST be unique across every lane: `id <x> already appears in <lane>.yml`.

## Stories

- Every entry MUST have a story document at `stories/<id>.md`; the filename stem MUST equal the id.
- Every story document MUST have an entry — or a pending fragment claiming its id. A document claimed by neither: `no entry and no fragment names this document`.
- A story or epic document's title line MUST open `# <id> —` with the id equal to the filename stem — the heading gate proves the `##` sections and cannot see the H1, so this identity is the checker's.
- A `story` or `spike` MUST have a fenced block under its `Example` heading; a `chore` need not.

## Epics

- Every epic filename stem MUST be a well-formed id; two epic documents MUST NOT claim one id; an epic id MUST NOT also be an entry id — one id names one thing.
- Every entry's `epic` field MUST name an existing epic document. The reverse does not hold: an epic no entry has joined is a legal first draft.

## Ids

- Every id MUST parse under the grammar. A malformed uid — wrong length or case — is reported as such rather than absorbed into the slug: `id '<x>' is not a name and four hex`. This is the one place the checker is more specific than the schema, because the grammar's tail is where a typo hides.
- Question headings MUST carry the `Q-<slug>-<uid>` grammar.
- An id MUST NOT equal an id a tombstone holds: a burned id is refused, naming both the claimant and the tombstone.

## Graph and lanes

- An entry MUST NOT need itself; every `needs` id MUST exist in some lane.
- An entry in `doing` or `review` MUST NOT need an entry that is not closed, and MUST NOT be blocked by an open question.
- A closed entry MUST have `outcome` and `closed`; a non-closed entry MUST have neither.
- `succeeded_by` MUST name an entry in some lane, and MUST NOT name the entry itself.
- The dependency graph MUST be acyclic: `dependency cycle among: <ids>`.

## Open questions

- Every `Blocks:` line MUST name at least one existing id; none already in a work lane; at least one still open (else `stale: every story it blocks is closed`).
- Every question section MUST have a `Blocks:` line: `a question that blocks nothing belongs in
  drafts`.

## Amends paths

In stories and epics alike: only the leading inline-code token of an `Amends` list item is a path; the path MUST be relative, MUST carry no `.` or `..` segment, and MUST resolve against the project root — unless its assertion opens with `new:`, which exempts existence and nothing else. `Governed by` paths are deliberately unchecked: a review responsibility, stated as such.

## Pending

- A fragment's entry id MUST match a story document; a fragment claiming a document that does not exist is a failure.
- An id in both a fragment and a lane file is a duplicate.
- A fragment's `captured` instant MUST be arithmetically valid, held to the same rule as a journal instant.
- A `needs` id that resolves in no lane and no fragment is a failure. A fragment's `epic` MUST name an existing epic document — landing it otherwise would write an entry the epic rules refuse.
- Drift is a warning naming the fact, never a failure: a `needs` target that closed, and an `after` target that is not a landed entry in the claimed lane — a fragment, a vanished id, an entry in a different lane, a closed entry. The vocabulary and the drain's handling are in [../record/pending-fragment.md](../record/pending-fragment.md).
- A pending entry is never eligible, has no rank, and its points are outside epic arithmetic.

## Transition journal

- A journal for an id in no lane is a failure — unless its last event targets `deleted` (a tombstone), which is exempt from this rule and from lane agreement, and held to every other.
- Every line MUST be exactly three non-empty tab-separated fields.
- Every instant MUST be RFC 3339 UTC with `Z`, arithmetically valid (real day of month, leap years, hour ≤ 23, minute ≤ 59, second ≤ 60 for a leap second).
- Both lanes on an event MUST be real lanes; the destination MAY be `deleted`.
- An event's source MUST NOT equal its destination, and MUST equal the destination of the event above it — a journal is one unbroken path; the first event's source is bound only to be a real lane.
- An event dated after now is a failure; an event preceding the one above it is a warning.
- The last event's destination MUST equal the entry's lane; the failure names both sides and offers both repairs. A closed entry's last event MUST enter `closed` on the entry's `closed:` date.
- Journal-or-nothing: an entry without a journal is held to none of these rules.

## Ranking

- R1, `todo.yml` only (ranking failure): an eligible entry MUST NOT sit below an ineligible one — whenever any entry in `todo` is eligible, the head is.
- R2, `backlog.yml` and `todo.yml` (ranking failure): an entry MUST NOT sit above an entry it needs; only same-lane edges count.
- `closed.yml`: a close date out of ascending sequence is a warning naming the repair. A gap left by a reopening departure is not a defect.
