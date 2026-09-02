# Resolution

How an invocation finds the plan repository it acts on. Resolution replaces discovery: no verb takes a zone argument, because there is no zone on the invocation's path to point at.

## The two steps

Every verb that touches a record resolves in two steps:

1. Walk upward from the working directory to the nearest `.wipctl.toml` and read `project_id`. Finding no config is a usage error (exit 2) — the invocation named no project.
2. Look the id up in the attachment registry. The zone is `$XDG_DATA_HOME/wipctl/projects/<project_id>/plan-repo/`. An id with no attachment is a failed check (exit 1) naming the `attach` invocation that fixes it.

Two verbs deviate, as before: `init` MUST NOT walk — scaffolding into a parent project by surprise is the failure mode the walk would create — and `help` and `version` resolve nothing. `attach` walks for the identity it is attaching.

Every worktree, and every independent clone of the same project on the machine, resolves the same plan repository through the same committed id. A caller who wants another project changes directory or runs the verb from inside that project's root; there is no positional zone argument and no project flag.

## The attachment registry

The registry is the data directory's `projects/` tree itself: an id is attached exactly when its slot holds a plan repository at `<slot>/plan-repo/`. No registry file exists — the filesystem owns this state, and a listing would be a second store of it. `attach` fills a slot by cloning or adopting a plan repository ([../cli/verbs/attach.md](../cli/verbs/attach.md)); `init` fills it by creating one ([../cli/verbs/init.md](../cli/verbs/init.md)).

## Identity

`project_id` is a human-friendly slug in the record's slug grammar, `[a-z0-9]+(-[a-z0-9]+)*`. At mint time — `init` or `attach` — it derives from the project's name, generally the repository name at the forge, which tends to equal the project root directory name.

Uniqueness is decided against this machine's attachment registry at mint time, not globally. Conflicts are resolved by appending postfixes, one at a time, in priority order, until the id is unique:

```text
1. <slug>                    the project name alone
2. + <parent-scope>          the forge namespace: the user or organisation, or the
                             group and subgroups, slugged
3. + <username>              the operator's forge account name
4. + <forge>                 the forge's own name
5. + NN                      01, 02, ... — last resort only
```

A step whose segment is already present in the id adds no disambiguation, so it MUST be skipped and the escalation moves to the next rule; no segment ever repeats. The rule is general, not a special case for any step. The case it exists for is the ordinary personal repository, where the forge namespace is the account name:

```text
scope != username (acme/payments)      scope == username (gbasso/wipctl)

1. payments                            1. wipctl
2. payments-acme                       2. wipctl-gbasso
3. payments-acme-gbasso                3. skipped — gbasso is already there
4. payments-acme-gbasso-github         4. wipctl-gbasso-github
5. payments-acme-gbasso-github-01      5. wipctl-gbasso-github-01
```

A project with no forge remote has no parent scope and no forge name: a step whose source does not exist is skipped the same way a repeated segment is, and the escalation falls through to the sequential number.

Rules:

- The derivation is a naming convenience at mint time, not a live binding. Once minted, the id is stable: renaming the repository, the owner, or the directory does not rename the id. Renaming the id is its own explicit, recorded operation, not a side effect of anything.
- To every consumer the id is one opaque token, exactly like an entry id. Nothing parses a project id into parts. The skip rule runs at mint time only, where the parts are still known; nothing reads them back out of a minted id afterwards.
- The id is committed in the host repository's `.wipctl.toml` and restated in the plan repository's `config.toml`, and the two MUST be checked against each other at every resolution.

There is a deliberate asymmetry with entry ids, stated so it does not read as an inconsistency. Entry ids track their titles ([ids.md](./ids.md)); project ids do not track their project's name. A project id is the resolution key that the attachment registry and a filesystem path are built on, so renaming it moves the plan repository on disk and invalidates the config in every checkout of the project. An entry id carries no such load.

## Diagnostics

Each resolution failure names its resolution, per the message obligations in [../cli/conventions.md](../cli/conventions.md):

```text
no config found on the upward walk                                        exit 2
  wipctl: no .wipctl.toml at or above /home/x/scratch
  wipctl: this directory is not inside a wipctl project; run
          'wipctl init' at the project root to make it one

the project is not attached on this machine                               exit 1
  wipctl: project payments-acme is not attached on this machine
  wipctl: run 'wipctl attach <plan-repo-url>' to clone its plan, or
          'wipctl attach --create' to start one

the attached plan repository is gone                                      exit 1
  wipctl: plan repository missing:
          ~/.local/share/wipctl/projects/payments-acme/plan-repo
  wipctl: it was attached but is no longer on disk; re-attach with
          'wipctl attach <plan-repo-url>'

the identity files disagree                                               exit 1
  wipctl: identity mismatch
  wipctl:   host  /home/x/src/payments/.wipctl.toml says payments-acme
  wipctl:   plan  ~/.local/share/wipctl/projects/payments-acme says billing-acme
  wipctl: this plan repository belongs to another project; re-attach the
          right one, or fix whichever file is wrong
```
