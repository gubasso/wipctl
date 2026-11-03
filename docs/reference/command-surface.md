# Command surface

What each verb is called and what it accepts: the exit codes, the usage of every verb, the peer writer, the plan flag and the verbs that take it, and the dependency manifest the health report reads. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## Exit codes

```text
0  the command did what was asked
1  a check the command performs did not hold
2  the invocation itself was wrong
3  the writer lock could not be acquired within the bounded wait
```

Rules that bind this section: `cli-conventions:the-exit-code-is-an-interface`.

## The verb surface

The command is `wipctl`. These are its verbs and their usage. A verb's own domain states what it does. This table states what it is called and what it accepts.

```text
wipctl [--plan <alias>] <verb> [...]

wipctl init [--root DIR] [--window-start YYYY-MM-DD] [--print-hooks] [--dry-run]
wipctl attach <plan-repo-url>
wipctl attach --create
wipctl attach --mint-uid
wipctl attach --peers
wipctl new <type> "<title>" [--id <id>] [--epic <id>] [--points <n>] [--lane <backlog|todo>]
wipctl land [--report]
wipctl move <id> --to <lane> [--outcome <o>] [--succeeded-by <id>] [--closed <date>] [--session <ref>] [--dry-run]
wipctl start [--session <ref>]
wipctl delete <id> [--dry-run]
wipctl rename <id> "<new title>" [--id <new-id>]
wipctl rename <id> --id <new-id>
wipctl fix [--slots | --registry [<name>]]
wipctl sync [--all] [--json]
wipctl resolve <id> --keep <here|remote>
wipctl validate
wipctl doctor
wipctl next [--why]
wipctl ids
wipctl board [--why]
wipctl dashboard
wipctl graph [--unblocks <id>]
wipctl flow [--lane <lane>]
wipctl stale [--all]
wipctl stale --hide <id> (--for <duration> | --forever)
wipctl stale --show <id>
wipctl velocity
wipctl epic [--json] [--write] [<epic-id>]
wipctl epics [--json]
wipctl initiative [--json] [<initiative-id>]
wipctl initiatives [--json]
wipctl peer add <url> [--as <alias>]
wipctl peer alias <alias> <new-alias>
wipctl peer url <alias> (--add <url> | --remove <url> | --set <url>...)
wipctl peer remove <alias>
wipctl peers [--json]
wipctl man [<verb>]
wipctl help
wipctl version
```

Every verb except `help`, `version`, and `man` is a workflow verb. A workflow verb operates the record. The other three operate the program and resolve nothing, so they cannot inspect staleness.

Ten workflow verbs write the record: `new`, `land`, `move`, `start`, `delete`, `rename`, `fix`, `sync`, `resolve`, and `peer`. Closing is not a verb of its own. It is `move --to closed` with an outcome.

`attach` takes exactly one of four forms: a plan location, `--create`, `--mint-uid`, or `--peers`. The identity-minting form commits `plan_id` and `project_id` to the plan trunk, then prints the project file line. It does not write the host. The `peer` operation is its first positional argument, so the surface gains no subcommands.

Rules that bind this section: `cli-conventions:the-verb-names-are-fixed`, `attachment:attach-takes-exactly-one-form`.

## The peer writer

```text
wipctl peer add <url> [--as <alias>]
wipctl peer alias <alias> <new-alias>
wipctl peer url <alias> (--add <url> | --remove <url> | --set <url>...)
wipctl peer remove <alias>
```

The first positional argument is the operation, exactly as `wipctl new <type> "<title>"` takes a type first. So `peer` is one dispatchable unit, the verb list stays derived from the build, and the surface gains no subcommands.

```text
add        clone the plan at the url outside the lock, read its plan_id
           and its project_id, then write the row. The alias is the peer's
           derived project slug unless --as gives one.

alias      change a row's alias to a name this project's readers prefer,
           and rewrite every prefixed id that used the old one, in the
           same transaction.

url        --add appends a location. --remove drops one, and is refused
           where it empties the list. --set replaces the whole list in the
           order given, which is how a row is reordered and how a stale
           first locator is taken out.

remove     drop a row, refused while any prefixed id still uses the alias.
```

Every form holds this plan's writer lock and ends in one plan trunk commit, like the writing verbs already named in the command surface. `add` is the one that also clones, so it follows the five steps of [filling the slots](./verb-mechanics.md#filling-the-slots).

An alias completes from the peer view, in the shape the command surface already requires: derived at completion time, and offering nothing where the source exits non-zero.

Rules that bind this section: `peer-attachment:the-command-surface-owns-the-table`, `peers:a-row-carries-a-uid-and-a-location`, `peer-attachment:a-row-in-use-is-not-removed`, `transactions:every-mutation-ends-in-one-commit`, `cli-conventions:completion-derives-its-candidates`.

## The plan flag

```text
wipctl [--plan <alias>] <verb> [...]
```

`--plan` is a global flag, parsed before the verb, in the group the command surface already defines. It takes an alias from this plan's own peer table. Given one, the verb acts on the plan that alias names, and every other rule of that verb is unchanged.

It reaches one hop. If `alpha` declares `beta` and `beta` declares `gamma`, then `gamma` has no name in `alpha`. Running `wipctl --plan gamma <verb>` from `alpha` is a usage error, because the table has no such row.

The flag selects from one table. A verb that must act on `gamma` is run from a shell inside it. The other route is for `alpha` to declare it and give it a name of its own.

Rules that bind this section: `plan-targeting:the-flag-selects-from-this-plans-table`.

## The dispatch matrix

`init` never walks, and the manual, the usage output, and the version resolve nothing. A flag that redirects resolution has nothing to say to such a verb, so the dispatch matrix is stated rather than the word every.

```text
takes --plan     every verb that resolves a record: the readers, the ten
                 writers, peer, peers, doctor, and validate

refuses --plan   init, man, help, version. Passing it is a usage error,
                 naming the verb and the reason, because the invocation
                 asks for a redirection the verb has nowhere to apply

one form         fix --registry refuses --plan too. The slot repair is
refuses it       machine-local and acts on the project underfoot, while
                 every other form of fix takes the flag
```

The matrix is derived the same way the verb list is, from the build. A verb added later joins one side of it by existing rather than by being remembered.

Rules that bind this section: `plan-targeting:a-verb-that-resolves-nothing-refuses-it`, `plan-targeting:the-dispatch-matrix-is-derived`, `repair:a-slot-repair-takes-the-project-underfoot`.

## The dependency manifest

The required set is git, plus the language runtime the implementation needs, or none. The baseline optional set holds one entry: a draft 2020-12 instance checker, probed by validation, presence-only, whose declared fallback is the named skip.

Each list command a project declares under an external source is an optional dependency too. It is discovered from that project's own declaration rather than baked into the manifest. It is probed by the external sources view, presence-only, and its declared fallback is the named degradation.

Rules that bind this section: `doctor:only-a-required-absence-fails`.
