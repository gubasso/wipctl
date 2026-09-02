# wipctl velocity

Points delivered per iteration window, from `closed.yml` and the configured cadence.

## Usage

```text
wipctl velocity
```

## Contract

- One row per iteration window from `iteration.start` to today: `YYYY-MM-DD..YYYY-MM-DD  <bar>  N pts`, the current window suffixed `(partial)`.
- Arithmetic: only `outcome: done` counts — the closing entry's own points, in the window of its own `closed:` date. `cut` and `reshaped` contribute zero here: a reshaped entry's delivery is its successor's own close, counted once, as the successor's points in the successor's window, so no points are ever counted twice. A window with no closes MUST render at zero rather than being omitted.
- The windows are anchored by the plan repository's `config.toml`; changing the anchor or the length restarts the series, and the verb MUST NOT stitch old windows onto new.
- The verb MUST read the record only — never repository history, never a remote.

## Example

```text
$ wipctl velocity
2026-07-06..2026-07-19  ████████▏      5 pts
2026-07-20..2026-08-02  ▏              0 pts
2026-08-03..2026-08-16  ██████████▌    7 pts (partial)
```

## Out of scope

Forecasting, averaging, counting anything but points.
