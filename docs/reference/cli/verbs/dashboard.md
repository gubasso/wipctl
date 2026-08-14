# wipctl dashboard

Four panels, one screen, each titled by the reader's question. Pure composition: it calls the
verbs that own each panel and computes nothing itself, so a number can never appear in two places
with two values.

## Usage

```text
wipctl dashboard [<plan-dir>]
```

## Contract

- Four panels, grouped by question, each opening with its headline fact in prose:
  - `now` — the lanes and the current story (from `board`/`next`).
  - `delivery` — velocity per window (from `velocity`).
  - `scope` — epic progress, delivered against promised (from `epics`).
  - `map` — the dependency ladder (from `graph`).
- Two layouts, one breakpoint at 100 columns read from the terminal size; below it panels stack.
- A panel whose owning verb reports unknown MUST render that reason in its place rather than
  being omitted — one degraded panel never degrades the other three.
- The verb MUST NOT write to disk. Empty record: four panels at zero, exit 0.
