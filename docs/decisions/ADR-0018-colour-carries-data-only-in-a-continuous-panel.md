# ADR-0018: Colour carries data only in a continuous panel

## Context and Problem Statement

Colour is the cheapest channel and the least durable: it dies in a pipe, a paste, a colour-blind reader, and a log.

## Considered Options

- Glyphs carry values everywhere; colour carries data only in a continuous panel that declares it
- Colour everywhere with a `--no-color` fallback that changes the data — two renderings that can disagree; the fallback becomes the truth nobody looks at
- No colour at all — surrenders the one place colour is genuinely the densest honest channel

## Decision Outcome

Chosen option: colour carries data only in a continuous panel — everywhere else the glyph holds the value. In a row-read panel, the value lives in the glyph; colour may reinforce, never carry. Only a continuous panel — a heatmap, a cumulative flow area, the half-block technique with two data rows per text row — may let colour alone carry data, and such a panel must say so in its heading. The mechanical test: strip the escape sequences and assert what survives.

## Consequences

- Good: alarms are glyphs (`⚠`), badges are glyphs, and the aging chart survives a pipe.
- Good: continuous panels remain possible without lowering the bar for every other panel.
- Bad: a continuous panel's data genuinely dies in a pipe, which its heading must admit.

## Status

Accepted
