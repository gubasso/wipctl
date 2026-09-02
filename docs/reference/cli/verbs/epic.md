# wipctl epic

Resolves one epic into its execution plan: what serves it, what those entries wait on, what is startable now, and where each story document lives.

## Usage

```text
wipctl epic [--json] [--write] [<epic-id>]
```

At most one positional; a second is exit 2. `--help` prints usage on stdout, exit 0, and names the verb's two neighbours: `epics`, the rollup over every epic, and `initiative`, the tier above this one.

## With no id — the listing

Lists the epic ids, one `<id>` per line; with `--json`, an array conforming to `epic-list.schema.json`. Exit 0. Deliberately not a fallback to the rollup: a bare invocation asks which epics exist, not how they are going. `epics` is the rollup, and each verb's help names the other.

## With an id — the resolution

An unknown id is exit 2, naming the directory searched and the listing that shows what exists — a bad invocation, not a failed check.

Resolution semantics, the load-bearing contract:

- Membership MUST be searched from entries' `epic` fields; the epic document holds no member list.
- The plan is the transitive closure of the members' `needs` edges. A prerequisite outside the epic MUST be included and marked by what it serves (`serves <other-epic>` or `serves no epic`), never hidden.
- Closed entries MUST be dropped from the groups; the point arithmetic still counts them.
- Entries MUST be ordered by the graph's topological order (ties broken by record position), then partitioned into `eligible now` and `blocked`, each keeping the graph order. The order is not a schedule and not a ranking: a consumer takes any eligible entry, works it, closes it, and asks again.
- The resolution MUST be derived on every call and stored nowhere.

```text
$ wipctl epic session-hardening
session-hardening   6/13 pts   4 open

eligible now
  rate-limit-the-search-endpoint  story  2  todo  ~/.local/share/wipctl/projects/payments-acme/plan-repo/stories/rate-limit-the-search-endpoint.md
  harden-the-proxy-defaults       story  2  backlog  ~/.local/share/wipctl/projects/payments-acme/plan-repo/stories/harden-the-proxy-defaults.md  serves no epic

blocked
  profile-composition  story  3  backlog  ~/.local/share/wipctl/projects/payments-acme/plan-repo/stories/profile-composition.md  needs rate-limit-the-search-endpoint
```

An empty group prints no heading. Header arithmetic: `done` delivers its own points; `cut` contributes nothing and stays in the total; a reopened member counts as open. A `reshaped` member delivers its own points exactly when its `succeeded_by` entry is closed `done` — one link is followed, never more, so a successor that is open, `cut`, reopened, or itself `reshaped` leaves the member undelivered. A successor that is itself a member counts separately as itself; the one-link rule is what keeps the two contributions distinct. The same implementation computes `epics` and the initiative tier's numbers, so a number cannot appear twice with two values.

## `--json`

The resolution as an object conforming to `epic.schema.json`, on stdout.

## `--write`

Writes the JSON payload (regardless of `--json`) to `$XDG_CACHE_HOME/wipctl/projects/<project_id>/epic-<id>.json` and prints only that path. The cache is not a store: rewritten every call, read back by no verb, required by no check, never committed; a missing, stale, or corrupt cache MUST change no answer and fail nothing.
