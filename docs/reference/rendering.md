# Terminal rendering

Terminal text is the only rendering target. No HTML writer, no image writer, no TUI: dragging a card is exactly the judgment the method reserves for a person, and a second output format is a second implementation to keep true. These constraints bind every view verb.

## Pipe safety

Measured by running, not by reading: a renderer MUST NOT emit escape sequences when stdout is not a terminal, MUST honour `NO_COLOR`, and MUST remain legible with every escape stripped. Colour MUST NOT be load-bearing in a row-read panel — the value lives in the glyph. Only a continuous panel (a heatmap, a cumulative flow area) MAY let colour alone carry data, and such a panel MUST say so in its heading. The mechanical test: strip the escapes and assert what survives.

Every panel MUST open with its headline fact in prose, so a screen reader and a log file receive the finding before the picture.

## Glyph tiers

- ASCII — the universal fallback; `--plain` swaps every drawing to it.
- Block Elements (U+2580–U+259F) — the default drawing tier; every glyph is display-width 1.
- Box Drawing — junctions for the ladder graph, chosen from the four up/down/left/right bits.
- Braille patterns exist as a density tier but are weakest in font coverage and are announced as braille by screen readers; nothing in the baseline views requires them.

## The width trap

Padding by character count is wrong twice: wide glyphs occupy two display cells, and byte-count arithmetic breaks outside a UTF-8 locale. The rule: labels that participate in column arithmetic MUST be ASCII; drawing glyphs MUST be the width-1 Block Elements tier; the environment probe MUST assert a UTF-8 locale before any drawing verb renders.

## Bars and precision

Horizontal bars MUST render at eighth-cell precision using the left-anchored eighth blocks plus the full block. A flat series MUST render at mid-height rather than at zero, so an unchanging value reads as presence. Vertical bars stack levels beyond eight. The half-block technique (`▀` with foreground and background colour, two data rows per text row) puts all meaning in colour and is therefore reserved for continuous panels.

## The ladder graph

The dependency drawing is a ladder, not a layered graph: one row per node in topological order, one column per edge in flight. A column opens when its source node is drawn, closes at the last child, and is immediately reused, so width tracks concurrent edges. No layout engine, no edge routing, no crossing minimisation — the day nested graphs are wanted is the day a heavier renderer is reconsidered.

## Layout

Two layouts per composite view, one breakpoint at 100 columns, read from the terminal size at render time. Below the breakpoint, panels and lanes stack, `todo` leading where lanes are stacked.

## Optional rendering tools

An implementation MAY adopt optional tools (a column joiner, an aggregator, a chart renderer) under the optional-dependency contract: each MUST be pipe-safe by the test above, MUST only improve output, MUST carry a manifest row with a version floor and a declared fallback, and its absence MUST degrade exactly one thing, announced in one line.
