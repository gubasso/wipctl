# Reporting Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`reporting:a-view-is-derived-and-never-written` — A view is derived and never written](#reportinga-view-is-derived-and-never-written--a-view-is-derived-and-never-written)
  - [`reporting:a-view-never-gates` — A view never gates](#reportinga-view-never-gates--a-view-never-gates)
  - [`reporting:the-board-reads-the-lane-entry` — The board reads the lane entry](#reportingthe-board-reads-the-lane-entry--the-board-reads-the-lane-entry)
  - [`reporting:the-current-story-is-marked` — The current story is marked](#reportingthe-current-story-is-marked--the-current-story-is-marked)
  - [`reporting:a-blocked-entry-is-shown-never-hidden` — A blocked entry is shown, never hidden](#reportinga-blocked-entry-is-shown-never-hidden--a-blocked-entry-is-shown-never-hidden)
  - [`reporting:a-dependency-across-plans-is-visible` — A dependency across plans is visible](#reportinga-dependency-across-plans-is-visible--a-dependency-across-plans-is-visible)
  - [`reporting:the-board-offers-no-interaction` — The board offers no interaction](#reportingthe-board-offers-no-interaction--the-board-offers-no-interaction)
  - [`reporting:a-row-names-the-key-that-placed-it` — A row names the key that placed it](#reportinga-row-names-the-key-that-placed-it--a-row-names-the-key-that-placed-it)
  - [`reporting:a-residual-tie-is-marked-without-a-flag` — A residual tie is marked without a flag](#reportinga-residual-tie-is-marked-without-a-flag--a-residual-tie-is-marked-without-a-flag)
  - [`reporting:the-preview-is-a-head-read` — The preview is a head read](#reportingthe-preview-is-a-head-read--the-preview-is-a-head-read)
  - [`reporting:a-defect-warns-and-the-line-prints` — A defect warns and the line prints](#reportinga-defect-warns-and-the-line-prints--a-defect-warns-and-the-line-prints)
  - [`reporting:pending-work-is-invisible-to-the-preview` — Pending work is invisible to the preview](#reportingpending-work-is-invisible-to-the-preview--pending-work-is-invisible-to-the-preview)
  - [`reporting:the-composite-view-computes-nothing` — The composite view computes nothing](#reportingthe-composite-view-computes-nothing--the-composite-view-computes-nothing)
  - [`reporting:a-degraded-panel-degrades-only-itself` — A degraded panel degrades only itself](#reportinga-degraded-panel-degrades-only-itself--a-degraded-panel-degrades-only-itself)
  - [`reporting:membership-is-not-drawn-as-an-edge` — Membership is not drawn as an edge](#reportingmembership-is-not-drawn-as-an-edge--membership-is-not-drawn-as-an-edge)
  - [`reporting:the-id-stream-judges-nothing` — The id stream judges nothing](#reportingthe-id-stream-judges-nothing--the-id-stream-judges-nothing)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

The read-only views: the lanes on one screen, what to start next, the composite screen, the dependency drawing, and the id stream. The boundary runs at the write. Every verb here is a pure function of the committed files. The rendering domain owns how a view looks, and each measure's own domain owns what it means.

The board draws a watch badge from the record alone. Neither the board nor the checker reaches a network or knows whether the outside item remains open.

A view that explains an order looks like a view that could fix one, so this page says it once. The explain flag adds a column. It takes no lock, writes no byte, and exits 0. A reader who wants a different order edits the record and reads again.

## Requirements

### `reporting:a-view-is-derived-and-never-written` — A view is derived and never written

A view MUST be derived on every run and MUST leave the filesystem untouched.

#### Scenario: A board is proposed as a file

- GIVEN a request to keep the rendered board in the zone
- WHEN a lane changes
- THEN the file is wrong, so every view is computed from the record on every read

Verify: `cargo nextest run --test reporting`

### `reporting:a-view-never-gates` — A view never gates

A view MUST NOT gate, MUST NOT take the writer lock, and MUST NOT block.

#### Scenario: A view runs during a write

- GIVEN a writer holding the lock
- WHEN a view runs
- THEN it returns the committed state immediately, and an empty record renders at zero and succeeds

Verify: `cargo nextest run --test reporting`

### `reporting:the-board-reads-the-lane-entry` — The board reads the lane entry

The board MUST render the five lanes in workflow order with their entry counts, and every row MUST show the entry's id and summary.

#### Scenario: A record holds many stories

- GIVEN a board over forty entries
- WHEN it renders
- THEN no document is opened, because the summary exists so a reader scanning the lane needs nothing else

The summary is read from the lane entry and never by opening a story document.

Verify: `cargo nextest run --test reporting`

### `reporting:the-current-story-is-marked` — The current story is marked

The board MUST mark the current story with the glyph `▸`, taking it from the same head the preview verb reads.

#### Scenario: Work is in flight and more is scheduled

- GIVEN entries in both lanes
- WHEN the board renders
- THEN the in-flight head carries the marker, because the current story is what a reader came to find

Verify: `cargo nextest run --test reporting`

### `reporting:a-blocked-entry-is-shown-never-hidden` — A blocked entry is shown, never hidden

The board MUST show blocked entries with distinct badges for a dependency block, a question block, and a watch block, distinguishable with escapes stripped. The watch badge MUST remain on an in-flight row.

#### Scenario: The board is piped into a log

- GIVEN entries blocked by a dependency, a question, and a watch
- WHEN the escapes are stripped
- THEN the glyphs still tell them apart, because they mark record work, a decision this project owes, and an outside item this project does not own

Verify: `cargo nextest run --test reporting`

### `reporting:a-dependency-across-plans-is-visible` — A dependency across plans is visible

The board MUST mark a row whose dependency reaches a peer, and the dependency drawing MUST name a peer's entry by its prefixed form.

#### Scenario: A reader scans a board holding both kinds of dependency

- GIVEN entries blocked inside this record and entries blocked by a peer
- WHEN the board draws
- THEN one glyph tells them apart, because a board is a scan and a scan needs a mark rather than a list. The blocked reason and the drawing name the peer's entry, which is the reader's cue that it lives elsewhere

Verify: `cargo nextest run --test rendering`

### `reporting:the-board-offers-no-interaction` — The board offers no interaction

The board MUST offer no interaction.

#### Scenario: A reader wants to drag a card

- GIVEN a rendered board
- WHEN they want to move an entry
- THEN they run the move verb, because moving a card is a judgment reserved for a person and a recorded event

Verify: `cargo nextest run --test reporting`

### `reporting:a-row-names-the-key-that-placed-it` — A row names the key that placed it

Under the explain flag, the board and the preview MUST name one key per row, which is the key that decided that row against the row above it, and MUST print no score.

#### Scenario: A reader asks why a row sits where it does

- GIVEN a lane rendered in chain order under the explain flag
- WHEN the reader reads one row
- THEN one key is named and the rest of the chain is not, because the reader wants to know why this row is here rather than one line up

The board renders each lane in chain order, and the explain flag adds a column and never a write.

```text
$ wipctl board --why

TODO
  ^  rate-limit-the-search-endpoint     class: expedite
     secure-session-storage             points: 1
     profile-composition                points: 2
     audit-the-token-store              id (tied)
```

Verify: `cargo nextest run --test reporting`

### `reporting:a-residual-tie-is-marked-without-a-flag` — A residual tie is marked without a flag

Where the residual key decides a row, the board MUST mark that row whether or not the explain flag is given.

#### Scenario: The ordinary board renders two tied entries

- GIVEN two entries tied on every stated key, rendered with no flag
- WHEN the board draws
- THEN the lower row carries the tie mark, because a mark only a flag reveals is a mark nobody sees and the tool must not look like it decided a priority

`ranking:the-residual-key-is-the-full-id` makes the chain total, and a total chain hides the difference between a decided order and an accidental one. This mark puts the difference back.

Verify: `cargo nextest run --test reporting`

### `reporting:the-preview-is-a-head-read` — The preview is a head read

The preview verb MUST print the head of the in-flight lane, or of the scheduled lane when that is empty, and MUST walk neither.

#### Scenario: The head is not startable

- GIVEN a scheduled lane whose head is blocked
- WHEN the preview runs
- THEN it warns and prints the line anyway, because the chain's first key already sorts eligible entries above ineligible ones

Verify: `cargo nextest run --test reporting`

### `reporting:a-defect-warns-and-the-line-prints` — A defect warns and the line prints

Where the preview finds a record defect, it MUST warn on the diagnostic stream and MUST print its line anyway.

#### Scenario: Both lanes are empty

- GIVEN a record with nothing scheduled and nothing in flight
- WHEN the preview runs
- THEN stdout is empty and the run succeeds, because there is no sentence for a consumer to strip

Verify: `cargo nextest run --test reporting`

### `reporting:pending-work-is-invisible-to-the-preview` — Pending work is invisible to the preview

The preview MUST NOT offer an unlanded entry as work to start.

#### Scenario: A capture is waiting to land

- GIVEN a fragment claiming the scheduled lane
- WHEN the preview runs
- THEN it is invisible, because an entry outside every lane is outside the chain

Verify: `cargo nextest run --test reporting`

### `reporting:the-composite-view-computes-nothing` — The composite view computes nothing

The composite view MUST call the verbs that own each panel and MUST compute nothing itself.

#### Scenario: A number appears in two panels

- GIVEN a count shown in the lanes panel and the scope panel
- WHEN both render
- THEN they agree, because the composite view is pure composition

Verify: `cargo nextest run --test reporting`

### `reporting:a-degraded-panel-degrades-only-itself` — A degraded panel degrades only itself

Where a panel's owning verb cannot answer, the composite view MUST render that reason in the panel's place.

#### Scenario: One measure is unavailable

- GIVEN a panel whose verb reports unknown
- WHEN the screen renders
- THEN the other three are unchanged, because one degraded panel never degrades the rest

Verify: `cargo nextest run --test reporting`

### `reporting:membership-is-not-drawn-as-an-edge` — Membership is not drawn as an edge

The dependency drawing MUST draw dependency edges and question edges with distinct glyphs, and MUST NOT draw epic or initiative membership as an edge.

#### Scenario: Two entries share an epic

- GIVEN entries with no dependency between them
- WHEN the drawing renders
- THEN no edge joins them, because membership is not sequencing at any tier

Verify: `cargo nextest run --test reporting`

### `reporting:the-id-stream-judges-nothing` — The id stream judges nothing

The id stream MUST print every id, one per line, in record order, and MUST report what is there without judging it.

#### Scenario: The record would fail validation

- GIVEN a record the checker rejects
- WHEN the id stream runs
- THEN it yields the ids it read and succeeds. A completion source that fails on a broken record fails exactly when it is needed

Verify: `cargo nextest run --test reporting`

## Unenforced rules

| Rule                                            | Why no command decides it                                                                    |
| ----------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `reporting:the-composite-view-computes-nothing` | Whether a panel recomputed a number or called the owning verb is a reading of the code path. |

Under concurrent agents the preview's line is advisory by construction. Two agents that both run it legitimately see the same head. The one that acts second learns so at the move, or avoids the race entirely by taking work instead.
