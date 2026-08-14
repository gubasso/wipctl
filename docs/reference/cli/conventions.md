# Command surface conventions

One command, `wipctl`, with capabilities as verbs. The rules here bind every verb; each verb's own
contract is one page under [verbs/](./verbs/).

## Exit codes

The exit code is an interface, uniform across every verb:

- `0` — the command did what was asked.
- `1` — a check the command performs did not hold.
- `2` — the invocation itself was wrong.
- `3` — the zone lock could not be acquired within the bounded wait.

A warning MUST NOT reach an exit code. The 1/2 split is load-bearing: a caller retrying on `1`
MUST NOT spin on a malformed invocation, and a hook failing on `1` MUST NOT fail on a typo it
should report as usage.

## Streams

stdout MUST carry data and nothing else; every diagnostic, log line, prompt, and progress message
MUST go to stderr. A diagnostic a verb speaks about its own invocation or progress carries a
prefix: `wipctl: <message>`, `wipctl: warning: <message>`, `wipctl: error: <message>`. A
validation diagnostic is the stated exception: each is one unprefixed `<location>: <message>`
line, specified in [../validation/ownership-split.md](../validation/ownership-split.md). A verb
whose output is a single value (`version`) MUST print it bare, with no banner.

## Global flags and dispatch

Global flags are parsed only before the verb: `-h`/`--help`, `-V`/`--version`, and `--` ending
option parsing. Any other leading option is exit 2 with
`unknown option '<x>'; run 'wipctl help' for usage`. A bare `wipctl` prints help and exits 0. An
unknown verb is exit 2:
`unknown command '<v>'; run 'wipctl help' for the commands this build ships`.

The shipped verb list MUST be derived from the build itself — one dispatchable unit per verb,
discovered, never a hard-coded constant — so `help` cannot claim a verb the build lacks and a new
verb joins the listing by existing.

## Zone resolution

Every reading verb accepts one optional positional `<plan-dir>`: the place the upward walk starts,
not the zone itself. The walk, the root definition, and the failure cases are specified in
[../record/config.md](../record/config.md). Two verbs deviate: `init` never walks, and `help` and
`version` resolve nothing.

## Writers and the zone lock

Exactly four verbs write a maintained record: `fix`, `move`, `land`, and `delete`. Each MUST take
an exclusive lock on the plan zone, held from its first read to its last rename, with any rank
repair it performs inside the same holding — the lost update is decided when a second process
reads a lane the first has not yet replaced.

- A waiting writer MUST say on stderr that it is waiting and for whom.
- The wait MUST be bounded. On expiry the verb MUST exit 3, naming the holder, rather than
  blocking forever.
- A stale holding (a dead holder) MUST be released, never inherited.
- The lock is a lock and not a record: it MUST live outside the zone's tracked content, be absent
  from a clone, and answer no question about the plan.
- Read-only verbs MUST NOT take the lock and MUST NOT block.

Every writer MUST honour preflight-before-first-byte: every check a write depends on runs before
anything is written, so a refused operation leaves every file byte-identical.

## Machine formats

There is no global machine-format flag. A verb whose output has structure to lose MAY offer a
verb-local `--json`, and a verb that offers one MUST ship a schema for the shape and a case
validating real output against it. Verbs whose output is already one parseable token per line
(`next`, `ids`) offer none. Drawings offer none.

## Optional runtime dependencies

An optional tool MUST improve output and MUST NOT gate it. Each optional dependency owes: an
entry in the dependency manifest ([verbs/doctor.md](./verbs/doctor.md)) naming its version floor,
the view it serves, and its declared fallback; a probe at verb entry, never mid-render; and
exactly one stderr line naming the degradation when the fallback is taken. A skipped check MUST
always be named — `schemas: skipped — no instance checker on PATH` — never silently green.

## Terminal output

Every rendering verb MUST obey [../rendering.md](../rendering.md): no escape sequences when
stdout is not a terminal, `NO_COLOR` honoured, meaning legible with escapes stripped, and a
plain-character fallback behind `--plain` where a verb draws glyphs.

## Shell completion

Completion MUST derive the verb list from `help` output at completion time, never from a
hard-coded list. Verbs taking an id complete from `wipctl ids`; verbs taking a lane complete the
five lane names. When the id source exits non-zero or prints nothing, completion MUST offer
nothing — never a fallback to filenames.
