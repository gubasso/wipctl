# wipctl epics

The rollup: every epic, one row, delivered against promised.

## Usage

```text
wipctl epics [--json]
```

## Contract

- One row per document under `epics/`, in id order. An epic no entry has joined MUST appear at zero rather than being omitted. An empty `epics/` prints nothing, exit 0.
- Terminal row: `<id, padded to the widest>  <bar>  <closed>/<total>  <open> open`. The bar is a fixed-width progress bar of closed points over total points.
- Point arithmetic MUST be shared with `epic` and with the initiative tier — one implementation computes all three tiers: `done` delivers, `cut` contributes nothing but stays in the total, `reshaped` delivers exactly when its successor closed `done` (one link, as [epic.md](./epic.md) defines), a reopened member is open.
- The verb MUST NOT report a staleness verdict or retirement advice: `closed_out` states that nothing remains open, and whether an epic should be retired is a review question.
- `--json`: an object conforming to `epics.schema.json`.
- `--help` names the verb's two neighbours: `epic`, the single-epic resolution, and `initiatives`, the rollup one tier up.

## Example

```text
$ wipctl epics
session-hardening   ██████████▏             6/13   4 open
release-readiness   ████████████████████████ 8/8    0 open
```
