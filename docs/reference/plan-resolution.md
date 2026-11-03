# Plan resolution

How an invocation finds the plan it acts on, and how a dependency finds the entry it names: from the working directory, from a peer row, from the plan flag, and from a prefixed id. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## Where each identity enters resolution

```text
the ordinary path   walk upward to .wipctl/project.toml, read project_id,
                    then find the slot whose plan file declares that value

where a plan id     read plan_id from the peer row, then find the slot whose
is given            plan file declares that value
```

The search reads `projects/*/plan-repo/.wipctl/plan.toml`, where both identities sit at a fixed depth. Two slots claiming one project are an ambiguity and never a choice.

The directory that resolution opens carries a slot name, as `slot-naming:a-slot-name-is-recorded-only-as-the-directory` states. Neither identity appears in a path.

The peer attachment domain owns the walk that fills a peer's slot. The plan targeting domain owns the third resolution path, where an invocation names one of this plan's peers.

Rules that bind this section: `attachment:duplicate-project-identities-are-ambiguous`.

## The third resolution path

```text
no --plan     walk upward to .wipctl/project.toml, read project_id,
              and find the slot whose plan file declares it

--plan <a>    resolve this project's plan as above, read its peers
              section, resolve <a> to plan_id, and find the slot whose
              plan file declares that identity
```

The second path still starts with the first, because the table that binds the alias lives in this plan. A caller outside any project has no table to read, so the flag there is the ordinary no-configuration usage error.

## A dependency that crosses plans

```yaml
- id: profile-composition
  type: story
  points: 2
  summary: "Compose a profile from the session identity the payments plan stores, so that a signed-in reader sees their own name."
  needs: [session-token-parsing, "payments#secure-session-storage"]
```

The example mixes both forms in one list on purpose. They sit side by side, and nothing distinguishes them past resolution.

```text
bare id       the five lane files of this record

alias#id      1. the alias resolves in the peers section to a plan_id
              2. the plan_id resolves in the attachment registry to a slot
              3. the id resolves in that slot's five lane files, read at
                 the tree of its current commit
```

A prefixed id names a lane entry and nothing else, exactly as a bare id does.

The separator is `#`.
