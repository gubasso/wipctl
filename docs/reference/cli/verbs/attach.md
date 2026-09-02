# wipctl attach

Fills this machine's attachment registry slot for the project the working directory is inside: clones the project's plan repository from a remote, or creates a fresh one locally. The registry and the identity rules are [../../record/resolution.md](../../record/resolution.md).

## Usage

```text
wipctl attach <plan-repo-url>
wipctl attach --create
```

Exactly one of the two forms; both together, or neither, is exit 2. `--help` prints usage on stdout, exit 0.

## Contract

- The verb resolves the identity first: the upward walk to `.wipctl.toml` names the `project_id` being attached. No config found is exit 2, as for every verb.
- With a URL: clones the plan repository into `$XDG_DATA_HOME/wipctl/projects/<project_id>/plan-repo/`, then verifies the identity agreement — the cloned repository's `config.toml` MUST name the same `project_id`. A disagreement is exit 1, the clone is removed, and the message names both sides and the choice (see the identity-mismatch diagnostic in [../../record/resolution.md](../../record/resolution.md)).
- With `--create`: creates the plan repository in the slot — the zone scaffold, `config.toml` restating the id, the hook set, the plan trunk, and the initial commit — exactly what `init` creates on the plan side, for a project whose host already carries its identity. The operator adds a remote whenever hosting is wanted; private, public, any forge, decided per project.
- A slot already holding a plan repository for this id is reported and left alone, exit 0 — attaching is idempotent. A slot holding a repository whose `config.toml` names a different id is exit 1 naming both ids: the slot belongs to another project.
- The verb MUST NOT touch the host repository and MUST NOT push.
- Output is one line naming the id and one naming the path — no machine format and no schema, under ADR-0013's one-line exemption.

## Example

```text
$ wipctl attach ssh://git@gitlab.example/acme/payments-plan.git
attached payments-acme
plan: ~/.local/share/wipctl/projects/payments-acme/plan-repo
```

## Diagnostics

```text
the clone failed                                                          exit 1
  wipctl: could not clone ssh://git@gitlab.example/acme/payments-plan.git
  wipctl: <the transport's own error, relayed>
  wipctl: check the URL and your access to the remote, then re-run
          'wipctl attach <plan-repo-url>'

the cloned repository is not a plan repository                            exit 1
  wipctl: ssh://git@gitlab.example/acme/payments-plan.git has no config.toml
          at its root
  wipctl: that repository is not a wipctl plan; check the URL, or start a
          plan for this project with 'wipctl attach --create'
```
