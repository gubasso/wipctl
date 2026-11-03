# Machine layout

Where wipctl keeps its files on one machine, how a slot is named, the states a slot name can be in, and the suppression file. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## The host file and the four base directories

```text
host repo:
  .wipctl/project.toml          identity only; committed; travels with every clone

$XDG_DATA_HOME/wipctl/
  projects/<slot name>/plan-repo/   the plan zone, its own repository, trunk only

$XDG_STATE_HOME/wipctl/
  projects/<slot name>/             replication bookkeeping, diagnostics
    suppressions.toml               stale reminders hidden on this machine

$XDG_CACHE_HOME/wipctl/
  projects/<slot name>/             derived views; disposable; no verb reads it back

$XDG_RUNTIME_DIR/wipctl/
  <slot name>.lock                  writer lock; transaction-scoped; never in the record

fallback, where XDG_RUNTIME_DIR is unset or empty:
  /tmp/wipctl-<uid>/                 created mode 0700
```

The cache is disposable, and no verb reads it back.

The transaction lock covers the plan record. A suppression file is local state outside that record, so its writers do not take the record lock.

```text
XDG_DATA_HOME    $HOME/.local/share
XDG_STATE_HOME   $HOME/.local/state
XDG_CACHE_HOME   $HOME/.cache
```

Rules that bind this section: `zone-layout:the-record-lives-under-the-data-directory`, `zone-layout:the-lock-directory-has-a-stated-fallback`, `epics:the-cache-is-authoritative-for-nothing`, `transactions:the-lock-is-not-a-record`, `metrics:a-suppression-is-local-and-never-replicates`, `zone-layout:a-base-directory-default-is-honoured`.

## Deriving the project slug

A project slug is the readable name derived for the project. The first step that answers supplies it:

```text
1. the main worktree's forge project name
2. the main worktree's directory name
3. the directory holding .wipctl/project.toml
```

Git identifies the main worktree.

A host project does not need to be a git repository. The project file marks its root by its presence, so the third step supplies a slug without git.

A peer has no host worktree. Its first url supplies the forge project name and namespace before the same escalation applies.

The slot name becomes free by postfix escalation:

```text
1. <slug>
2. + <parent-scope>
3. + <username>
4. + <forge>
5. + 01, 02, and the first free sequential number
```

At mint time, the implementation derives the project slug, escalates it, and freezes the result as the slot name. At every read, it derives the slug again for display and drift detection.

A derived slug never selects a path, resolves a record, or compares with a committed value. It decides only the first slot name candidate at mint time.

Rules that bind this section: `slot-naming:a-project-slug-follows-one-ladder`, `slot-naming:a-taken-name-escalates`, `slot-naming:a-derived-slug-decides-only-at-mint-time`.

## Slot states

```text
matching  the slot name and the derived slug agree
drifted   they disagree; the record is intact and every verb works
absent    the project file names a project that no slot holds
```

Drift is legal. It does not fail a check or a verb.

Detection does not repair drift. A rename needs both slot-name locks, changes the registry, and moves machine-local state. An ordinary reader does not become that writer.

The explicit repair is `wipctl fix --registry [<name>]`, as the repair domain specifies.

The scaffold stays create-only. Every other write starts with a verb that a person ran.

Rules that bind this section: `slot-naming:drift-is-a-healthy-state`, `repair:the-slot-report-writes-nothing`, `repair:a-slot-repair-locks-both-names`, `attachment:the-scaffold-is-create-only`.

## Suppressions

The suppression file lives at `$XDG_STATE_HOME/wipctl/projects/<slot name>/suppressions.toml`. Each `[[suppression]]` table carries an entry `id` and exactly one end condition.

```toml
[[suppression]]
id = "the-stale-alarm"
until = "2026-10-09T12:00:00Z"

[[suppression]]
id = "keep-the-live-story"
forever = true
```

`wipctl stale --hide <id> --for <duration>` records an expiry in the transition journal's stated UTC textual form. `wipctl stale --hide <id> --forever` records a permanent suppression. `wipctl stale --show <id>` removes either form.

The file is local state, not a plan record or a machine output. It owes no schema. The metrics cases own its shape and behavior.

Rules that bind this section: `metrics:a-suppression-is-local-and-never-replicates`.
