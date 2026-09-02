# wipctl initiative

Decomposes one initiative into its member epics — reported, never executed. The entry-level execution plan stays the epic verb's answer and has exactly one home (ADR-0056); this verb's output is the list to run it against. The model is [../../../explanation/initiatives.md](../../../explanation/initiatives.md).

## Usage

```text
wipctl initiative [--json] [<initiative-id>]
```

An argument matching the id grammar is the initiative id; a second positional is exit 2. `--help` prints usage on stdout, exit 0.

## With no id — the listing

Lists the initiative ids, one `<id>` per line; with `--json`, an array conforming to `initiative-list.schema.json`. Exit 0. Deliberately not a fallback to the rollup: a bare invocation asks which initiatives exist, not how they are going. `initiatives` is the rollup, and each verb's help names the other — the same pairing as `epic` and `epics`, under ADR-0015.

## With an id — the decomposition

An unknown id is exit 2, naming the directory searched — a bad invocation, not a failed check.

- Membership MUST be searched from the epic documents' `Initiative` sections; the initiative document holds no member list.
- One row per member epic, each carrying the epic's own arithmetic — delivered against promised, and its open count. The verb MUST NOT resolve entries into eligible and blocked groups: execution is `epic <id>`'s answer.
- Point arithmetic MUST be shared with `epic` and `epics` — one implementation computes all three tiers, so a number cannot appear twice with two values. An entry carrying no epic is outside every initiative's arithmetic; a prerequisite from outside the initiative is not counted and not shown — the epic verb is where an outside prerequisite surfaces, marked by what it serves.
- An initiative no epic has joined MUST appear at zero rather than being omitted.
- The rollup MUST NOT report a staleness verdict or retirement advice; retirement is a review question at every tier.
- Derived on every call and stored nowhere.

```text
$ wipctl initiative trustworthy-by-default
trustworthy-by-default   6/21 pts   11 open   3 epics

  session-hardening    ██████████▏             6/13   4 open
  release-readiness    ▏                       0/8    7 open
  audit-trail          ▏                       0/0    0 open
```

## `--json`

The decomposition as an object conforming to `initiative.schema.json`, on stdout.

## Diagnostics

```text
the id names no initiative                                                exit 2
  wipctl: no document initiatives/trustworthy-by-default.md
  wipctl: 'wipctl initiative' lists the initiatives this record holds
```
