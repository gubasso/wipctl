# Rendering Specification

<!--TOC-->

- [Purpose](#purpose)
- [Glyph tiers](#glyph-tiers)
- [Requirements](#requirements)
  - [`rendering:the-terminal-is-the-only-target` — The terminal is the only target](#renderingthe-terminal-is-the-only-target--the-terminal-is-the-only-target)
  - [`rendering:a-renderer-is-pipe-safe` — A renderer is pipe-safe](#renderinga-renderer-is-pipe-safe--a-renderer-is-pipe-safe)
  - [`rendering:colour-is-not-load-bearing` — Colour is not load-bearing](#renderingcolour-is-not-load-bearing--colour-is-not-load-bearing)
  - [`rendering:a-panel-opens-with-its-finding` — A panel opens with its finding](#renderinga-panel-opens-with-its-finding--a-panel-opens-with-its-finding)
  - [`rendering:a-column-label-is-ascii` — A column label is ASCII](#renderinga-column-label-is-ascii--a-column-label-is-ascii)
  - [`rendering:the-locale-is-probed-before-drawing` — The locale is probed before drawing](#renderingthe-locale-is-probed-before-drawing--the-locale-is-probed-before-drawing)
  - [`rendering:a-bar-renders-at-eighth-cell-precision` — A bar renders at eighth-cell precision](#renderinga-bar-renders-at-eighth-cell-precision--a-bar-renders-at-eighth-cell-precision)
  - [`rendering:a-flat-series-renders-at-mid-height` — A flat series renders at mid-height](#renderinga-flat-series-renders-at-mid-height--a-flat-series-renders-at-mid-height)
  - [`rendering:the-dependency-drawing-is-a-ladder` — The dependency drawing is a ladder](#renderingthe-dependency-drawing-is-a-ladder--the-dependency-drawing-is-a-ladder)
  - [`rendering:a-composite-view-has-two-layouts` — A composite view has two layouts](#renderinga-composite-view-has-two-layouts--a-composite-view-has-two-layouts)
  - [`rendering:an-optional-renderer-degrades-one-thing` — An optional renderer degrades one thing](#renderingan-optional-renderer-degrades-one-thing--an-optional-renderer-degrades-one-thing)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

The constraints every view verb obeys. Terminal text is the only rendering target. Dragging a card is exactly the judgment the method reserves for a person, and a second output format is a second implementation to keep true. The boundary runs at the drawing. This domain says how a view looks, and each verb's own domain says what it shows.

## Glyph tiers

| Tier             | Use                                                                                                            |
| ---------------- | -------------------------------------------------------------------------------------------------------------- |
| ASCII            | the universal fallback, selected by the plain flag                                                             |
| Block Elements   | the default drawing tier; every glyph is display-width 1                                                       |
| Box Drawing      | junctions for the ladder graph, from the four direction bits                                                   |
| Braille patterns | a density tier, weakest in font coverage, announced as braille by screen readers; no baseline view requires it |

A cross-plan marker is drawn from the default tier, and its plain-flag form comes from the ASCII tier, like every other badge. It carries one fact, which is that a dependency of this row reaches another plan, so one width-1 glyph is the whole of it.

## Requirements

### `rendering:the-terminal-is-the-only-target` — The terminal is the only target

The implementation MUST render to terminal text and MUST NOT ship a second output format for a view.

#### Scenario: A web view is proposed

- GIVEN a request for a browser rendering of the board
- WHEN it is weighed
- THEN it is refused, because dragging a card is the judgment the method reserves for a person

Verify: `cargo nextest run --test rendering`

### `rendering:a-renderer-is-pipe-safe` — A renderer is pipe-safe

A renderer MUST honour the no-colour environment variable, MUST emit no escape sequence into a pipe, and MUST stay legible without escapes.

#### Scenario: A panel is captured into a log

- GIVEN a view whose output is redirected
- WHEN the escapes are stripped and the content asserted
- THEN the meaning survives, because the property is measured by running and not by reading

Verify: `cargo nextest run --test rendering`

### `rendering:colour-is-not-load-bearing` — Colour is not load-bearing

Colour MUST NOT carry data alone in a row-read panel, and a continuous panel that lets it MUST say so in its heading.

#### Scenario: A heatmap carries its value in colour

- GIVEN a continuous panel where colour is the only channel
- WHEN it renders
- THEN the heading says so, while in every row-read panel the value lives in the glyph

Verify: `cargo nextest run --test rendering`

### `rendering:a-panel-opens-with-its-finding` — A panel opens with its finding

Every panel MUST open with its headline fact in prose.

#### Scenario: A screen reader reaches a chart

- GIVEN a panel whose picture carries the finding
- WHEN the reader arrives
- THEN the prose already stated it, because a log file and a screen reader receive the finding before the picture

Verify: `cargo nextest run --test rendering`

### `rendering:a-column-label-is-ascii` — A column label is ASCII

A label participating in column arithmetic MUST be ASCII, and a drawing glyph MUST come from the width-1 tier.

#### Scenario: A wide glyph lands in a padded column

- GIVEN a label padded by character count
- WHEN a wide glyph occupies two display cells
- THEN the column breaks, as byte-count arithmetic breaks outside a UTF-8 locale

Verify: `cargo nextest run --test rendering`

### `rendering:the-locale-is-probed-before-drawing` — The locale is probed before drawing

Before a drawing verb renders, the implementation MUST assert a UTF-8 locale.

#### Scenario: A drawing verb runs under a legacy locale

- GIVEN an environment without a UTF-8 locale
- WHEN the probe runs at entry
- THEN the verb reports it rather than drawing broken glyphs, because a probe mid-render splits one view across two shapes

Verify: `cargo nextest run --test rendering`

### `rendering:a-bar-renders-at-eighth-cell-precision` — A bar renders at eighth-cell precision

A horizontal bar MUST render at eighth-cell precision using the left-anchored eighth blocks and the full block.

#### Scenario: Two close values are compared

- GIVEN two bars differing by less than one cell
- WHEN they render
- THEN the eighth blocks separate them, while vertical bars stack levels beyond eight

Verify: `cargo nextest run --test rendering`

### `rendering:a-flat-series-renders-at-mid-height` — A flat series renders at mid-height

A flat series MUST render at mid-height rather than at zero.

#### Scenario: A value does not change across a window

- GIVEN a series with one repeated value
- WHEN it renders
- THEN an unchanging value reads as presence, because a flat line at zero reads as absence

Verify: `cargo nextest run --test rendering`

### `rendering:the-dependency-drawing-is-a-ladder` — The dependency drawing is a ladder

The dependency drawing MUST be one row per node and one column per edge in flight, and MUST NOT use a layout engine.

#### Scenario: Two dependencies overlap

- GIVEN a column opened at its source node
- WHEN its last child is drawn
- THEN the column closes and is reused immediately, so width tracks concurrent edges

Verify: `cargo nextest run --test rendering`

### `rendering:a-composite-view-has-two-layouts` — A composite view has two layouts

A composite view MUST carry two layouts with one breakpoint at 100 columns, read from the terminal size at render time.

#### Scenario: A narrow terminal renders the board

- GIVEN a terminal below the breakpoint
- WHEN the view renders
- THEN the panels and lanes stack, with the scheduled lane leading, because side-by-side columns below the breakpoint wrap into noise

Verify: `cargo nextest run --test rendering`

### `rendering:an-optional-renderer-degrades-one-thing` — An optional renderer degrades one thing

Where the implementation adopts an optional rendering tool, that tool MUST be pipe-safe, MUST carry a manifest row, and MUST degrade exactly one thing.

#### Scenario: A chart renderer is missing

- GIVEN a view that draws on an optional renderer
- WHEN the tool is absent
- THEN one line names the degradation and the rest of the view is unchanged

Verify: `cargo nextest run --test rendering`

## Unenforced rules

| Rule                                       | Why no command decides it                                                                        |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| `rendering:a-panel-opens-with-its-finding` | Whether an opening line states the finding or merely labels the panel is a reading of the prose. |

The half-block technique draws two data rows per text row, using foreground and background colour. It puts all meaning in colour, so it is reserved for continuous panels. The day nested graphs are wanted is the day a heavier renderer is reconsidered.
