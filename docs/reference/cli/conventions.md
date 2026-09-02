# Command surface conventions

One command, `wipctl`, with capabilities as verbs. The rules here bind every verb; each verb's own contract is one page under [verbs/](./verbs/help.md).

## Exit codes

The exit code is an interface, uniform across every verb:

- `0` — the command did what was asked.
- `1` — a check the command performs did not hold.
- `2` — the invocation itself was wrong.
- `3` — the writer lock could not be acquired within the bounded wait.

A warning MUST NOT reach an exit code. The 1/2 split is load-bearing: a caller retrying on `1` MUST NOT spin on a malformed invocation, and a hook failing on `1` MUST NOT fail on a typo it should report as usage.

## Streams

stdout MUST carry data and nothing else; every diagnostic, log line, prompt, and progress message MUST go to stderr. A diagnostic a verb speaks about its own invocation or progress carries a prefix: `wipctl: <message>`, `wipctl: warning: <message>`, `wipctl: error: <message>`. A validation diagnostic is the stated exception: each is one unprefixed `<location>: <message>` line, specified in [../validation/ownership-split.md](../validation/ownership-split.md). A verb whose output is a single value (`version`) MUST print it bare, with no banner.

Paths in a mutation report are relative to the plan repository root, which every line of one report shares; a path printed for a consumer to open — a story document from `next` or `new`, a member path from `epic` — MUST be absolute, because the record no longer lives under the working directory.

## Every message teaches

A command's output tells its reader, human or agent, what happened, what it means for the record, and what to do next. A diagnostic that names a failure without naming its resolution is unfinished. The obligations, binding on every verb page:

- A failure diagnostic MUST carry four things: where — a path, and a line where one exists — what was expected, what was found, and the resolution: a command to run or the judgment to make.
- A refusal MUST name the verb or the edit that unblocks it. A `move` refused for an unclosed need names the need; a drain refused for an incomplete fragment names the file and the field.
- Success output MUST state what changed and where, and MUST name the next step where one exists and is not obvious.
- Never a bare code, never a bare `invalid`, never a stack trace, never a message whose meaning requires reading the source.
- Guidance goes to stderr under the `wipctl:` prefix; the data stream stays parseable. This is the stream discipline above, unchanged.
- The text is written for a person and MUST be stable enough for an agent to match on.

A message MUST NOT offer a flag that no verb page defines. The obligation is gated, not asserted: every refusal path's contract case asserts that its message names the resolution ([../quality-gates.md](../quality-gates.md)).

## Global flags and dispatch

Global flags are parsed only before the verb: `-h`/`--help`, `-V`/`--version`, and `--` ending option parsing. Any other leading option is exit 2 with `unknown option '<x>'; run 'wipctl help' for usage`. A bare `wipctl` prints help and exits 0. An unknown verb is exit 2: `unknown command '<v>'; run 'wipctl help' for the commands this build ships`.

The shipped verb list MUST be derived from the build itself — one dispatchable unit per verb, discovered, never a hard-coded constant — so `help` cannot claim a verb the build lacks and a new verb joins the listing by existing.

## Resolution

A verb acts on the project the working directory is inside. Every verb that touches a record resolves it in two steps — the committed `project_id` found by the upward walk, then the attachment registry — specified with the failure cases in [../record/resolution.md](../record/resolution.md). There is no positional zone argument and no project flag; a caller who wants another project changes directory. Three verbs deviate: `init` never walks, and `help` and `version` resolve nothing.

## Writers and the transaction lock

Nine verbs write a maintained record: `new`, `fix`, `move`, `land`, `delete`, `start`, `rename`, `sync`, and `resolve` — every verb whose transaction ends in a plan trunk commit, the mint included, because the mint reads the record it is about to write into. Each MUST hold the writer lock — exclusive, one per project per machine, at `$XDG_RUNTIME_DIR/wipctl/<project_id>.lock`, spanning every worktree and clone of the project on the machine — for one transaction: from the deciding read to the final commit, any rank repair included. The lock protects a transaction, never a session; an agent drafting a story for twenty minutes holds nothing, and hold times are subsecond. The reasoning is [../../explanation/concurrency.md](../../explanation/concurrency.md); the lock directory's required fallback is in [../record/zone-layout.md](../record/zone-layout.md).

- A waiting writer MUST say on stderr that it is waiting and for whom: `wipctl: waiting: lock held by pid 41291 (wipctl move) since 0.4s ago`.
- The wait MUST be bounded. On expiry the verb MUST exit 3, naming the holder and the judgment: `wipctl: lock still held by pid 41291 after 10s` / `wipctl: a transaction takes milliseconds, so that process is stuck rather than busy; wait and retry, or end it and retry`.
- A stale holding (a dead holder) MUST be released, never inherited.
- The lock is a lock and not a record: it MUST live outside the plan repository, be absent from a clone, and answer no question about the plan. There is no durable queue; bounded wait plus retry is the queue at the only depth that occurs, and strict fairness is never promised.
- Read-only verbs MUST NOT take the lock and MUST NOT block. They read the committed state.

Every writer MUST honour preflight-before-first-byte: every check a write depends on runs before anything is written, against the record at current HEAD inside the lock, so a refused operation leaves every file byte-identical.

## The plan trunk commit

Every semantic mutation MUST end in one commit on the plan trunk, inside the same transaction, staging only the paths the tool owns. There is no configuration that turns it off, and the host repository gains no commit from any verb. The message grammar is fixed and is the tool's, not the project's:

```text
plan: capture <id>
plan: move <id> from <from> to <to>
plan: close <id> as <outcome>
plan: land <n> fragments
plan: rename <old> to <new>
plan: fix ranking
plan: delete <id>
plan: reconcile <n> changes
plan: resolve <id> keeping <side>
```

The plan repository's own hooks MUST run — never bypassed. A refused commit MUST leave the record written and exit 1 with `the record was written but the commit was refused; the paths are staged` — an abnormal state `doctor` helps diagnose, never a hidden one. Push is never part of the transaction; replication is [verbs/sync.md](./verbs/sync.md).

## Machine formats

There is no global machine-format flag. A verb whose output has structure to lose MAY offer a verb-local `--json`, and a verb that offers one MUST ship a schema for the shape and a case validating real output against it. Verbs whose output is already one parseable token per line (`next`, `start`, `ids`, `attach`, `resolve`) offer none. Drawings offer none.

## Optional runtime dependencies

An optional tool MUST improve output and MUST NOT gate it. Each optional dependency owes: an entry in the dependency manifest ([verbs/doctor.md](./verbs/doctor.md)) naming its version floor, the view it serves, and its declared fallback; a probe at verb entry, never mid-render; and exactly one stderr line naming the degradation when the fallback is taken. A skipped check MUST always be named — `schemas: skipped — no instance checker on PATH` — never silently green.

## Terminal output

Every rendering verb MUST obey [../rendering.md](../rendering.md): no escape sequences when stdout is not a terminal, `NO_COLOR` honoured, meaning legible with escapes stripped, and a plain-character fallback behind `--plain` where a verb draws glyphs.

## Shell completion

Completion MUST derive the verb list from `help` output at completion time, never from a hard-coded list. Verbs taking an id complete from `wipctl ids`; verbs taking a lane complete the five lane names. When the id source exits non-zero or prints nothing, completion MUST offer nothing — never a fallback to filenames.
