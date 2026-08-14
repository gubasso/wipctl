# wipctl flow

Age, dwell, and rework per entry, folded from the transition journal — record facts, never repository archaeology.

## Usage

```text
wipctl flow [--lane <lane>] [<plan-dir>]
```

## Measures

- age — from the instant of the first event entering `doing` to the instant of the entry's last journal event when the entry is in `closed`, otherwise to now. An entry whose journal never enters `doing` has no age and reports `-`.
- dwell — the summed intervals spent in `review`: each interval runs from an event entering `review` to the next event, and an entry currently in `review` counts its open interval through now.
- rework — entries into `doing` beyond the first.

`now` is the system clock, read once per invocation and used for every open interval — the one clock this verb reads, never a substitute for a record instant. A duration is displayed in whole days, floored, so `25h` reads `1d` and anything under a day reads `0d`.

## Contract

- Default output: a table `id  age  dwell  rework`, one row per lane entry; an entry with no journal renders as its unmeasured line.
- `--lane <lane>` prints the lane's own measure for the entries currently in that lane: `--lane doing` prints `id  age  rework`, `--lane review` prints `id  dwell`. The other three lanes have no lane measure; naming one is exit 2 `flow has no lane measure for <lane>`.
- An entry with no journal MUST be reported unmeasured, with its reason — never as zero. An entry whose journal starts later than its first move reports what the journal holds and says from when.
- An entry whose journal instants decrease anywhere MUST be reported unmeasured — `unmeasured: journal out of order` — never folded into a negative, clamped, or skipped interval; the record carries the warning, and this verb refuses to pretend a number.
- The verb MUST be read-only and local: no remote, no commit metadata, no clock fallback to the close date. It MUST report the same numbers on a full clone and a depth-1 clone.
- The verb MUST NOT gate, and MUST store nothing.

## Example

```text
$ wipctl flow
supervised-child-runtime-b47d        3d   1d   1
secure-session-storage-9c2e          6d   2d   0
proxy-guide-53c9                     unmeasured: no journal
```

## Out of scope

Storing any measure, gating on any measure, a fourth measure.
