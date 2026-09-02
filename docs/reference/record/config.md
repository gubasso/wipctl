# Configuration

Two files, one principle: the host file identifies, and the plan repository configures.

## The host identity file

`.wipctl.toml` lives at the project root of the host repository. The directory holding it is, by definition, the project root: every verb finds the project by walking upward to this file, so nothing else marks the root. The file is hidden, committed, and travels with every clone.

```toml
project_id = "payments-acme"
```

- `project_id` — REQUIRED, and the file's only key. The project's identity in the slug grammar `[a-z0-9]+(-[a-z0-9]+)*`, minted at `init` or `attach` and resolved through the attachment registry; see [resolution.md](./resolution.md). Unknown keys MUST be rejected.

The file carries no schema of its own: one required key is below a schema's floor, the same reasoning under which a one-line output owes no machine format (ADR-0013). The cross-file checker owns its presence, its grammar, and its agreement with the plan repository's identity.

## The plan repository's config

`config.toml` lives at the zone root, inside the plan repository. It restates the identity and carries every fact about the plan itself — the plan owns its own home, so its cadence and thresholds live with it, not with the host.

```toml
project_id = "payments-acme"

[iteration]
start = 2026-07-06
length_days = 14

[aging]
threshold_days = 21
```

- `project_id` — REQUIRED. MUST equal the host file's value; the two are checked against each other at every resolution, and a disagreement is a failed check naming both sides.
- `iteration.start` — REQUIRED. Calendar date anchoring the first iteration window.
- `iteration.length_days` — REQUIRED. MUST be an integer of at least 1. Together with `start` it defines every velocity window; changing either restarts the series.
- `aging` — OPTIONAL table.
  - `aging.threshold_days` — MUST be an integer of at least 1. The age past which `wipctl aging` marks a row. Absent, bars render with no marks — there is no default threshold.

No key has a read-time default: a verb finding a required key absent MUST fail the check rather than assume a value, because a value guessed on a project's behalf is a value nobody wrote down. The scaffold generates the file complete — the minted id, the given or current start date, `length_days = 14` — and from that moment its operator owns every value in it (see [../cli/verbs/init.md](../cli/verbs/init.md)).

There is no `commit` table. Every plan mutation is committed to the plan trunk in the tool's own grammar, with no configuration that turns it off, and the host repository gains no commit from any verb (see [../cli/conventions.md](../cli/conventions.md)).

Unknown keys MUST be rejected at every level. `config.schema.json` owns the types and constraints of `config.toml`; the cross-file checker owns presence and agreement.

## Diagnostics

- No `.wipctl.toml` at or above the working directory — a usage error (exit 2): the invocation named no project. The message names `wipctl init` as the resolution.
- `project_id` absent or outside the slug grammar, in either file — failure naming the file and the field.
- The two `project_id` values disagreeing — failure naming both files and both values, and the choice between re-attaching and correcting whichever file is wrong.
- A required key absent from `config.toml` — failure naming the field.
- A malformed value — the schema's job, reported by the schema half of `validate`.
