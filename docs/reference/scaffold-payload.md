# Scaffold payload

What the scaffold lands in a new plan zone, the block it prints for the host's instruction file, and the stale conversation the travelling document teaches. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## Payload inventory

| Payload member      | Lands at                      | Notes                                                                                                                          |
| ------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| host project file   | `<root>/.wipctl/project.toml` | generated, not copied: the minted `project_id` and nothing else. The scaffold creates the directory and the file together      |
| plan configuration  | `<zone>/.wipctl/plan.toml`    | generated: both minted identities, the window values, and `delay-cost`, `points-ascending` ranking preferences                 |
| zone `README.md`    | `<zone>/README.md`            | orientation: where to start, what bounds it, what stops it, what already happened                                              |
| `AGENTS.md`         | `<zone>/AGENTS.md`            | the travelling method; gate placeholders substituted at emission                                                               |
| `charter.md`        | `<zone>/charter.md`           | angle-bracketed placeholders for a person to fill                                                                              |
| `open-questions.md` | `<zone>/open-questions.md`    | heading plus the `Blocks:` grammar; ships without a sample question                                                            |
| lane files          | `<zone>/lanes/*.yml`          | each exactly `lane: <name>` plus `stories: []`                                                                                 |
| stories directory   | `<zone>/stories/`             | created empty                                                                                                                  |
| hook set            | the plan repository           | the record gate at both stages, the schema checks, the three heading-shape checks, the single-branch guard, the no-force guard |
| templates           | not copied                    | shipped location printed for a person to copy                                                                                  |

The zone is the plan repository's working tree, which the scaffold creates whole: the repository, the trunk, the hook set, and the initial commit.

The `watches.md` file is earned when the record gains its first outside wait. It is absent from a scaffolded record.

The scaffold mints and reports `project_id` and `plan_id`. The slot name keeps paths readable, and messages use the derived project slug.

Rules that bind this section: `scaffold:the-payload-is-discovered`, `scaffold:an-earned-directory-is-not-seeded`.

## The host pointer

The scaffold asks which existing file carries the host project's instructions. It accepts a repository-relative path or an empty answer that skips the pointer.

The scaffold created neither that file nor its repository, so it prints the block for the operator to place. It leaves the host file unchanged.

```text
<!-- wipctl:begin -->
## Planning

This project plans with wipctl. Before you read, change, or take any
planning work, run `wipctl man` and follow the method it prints.

Take work with `wipctl start`. Never edit a lane file by hand.
<!-- wipctl:end -->
```

A later placement replaces the content between the markers instead of appending another block. The command reaches the travelling document without exposing a machine-specific path.

Rules that bind this section: `scaffold:the-host-pointer-names-a-command-and-not-a-path`, `scaffold:the-host-pointer-is-placed-where-the-operator-says`, `scaffold:the-scaffold-writes-only-what-it-creates`.

## The stale conversation

The travelling document gives an agent this protocol in the order it runs:

1. At session start, run `wipctl stale` before reading what work is next.
2. Read the rows rather than the exit code. The command exits 0 both with marked rows and with none.
3. Gather evidence for each row. Check whether it names a branch, whether that branch has recent commits, and whether a needed peer entry remains open.
4. Offer options for that row from its own evidence. Use the action vocabulary that follows, without treating it as a lookup table.
5. Ask a person what to do. Perform the answer only after it arrives.

The action vocabulary is:

- `return to todo`
- `record the wait`
- `split`
- `cut`
- `keep`

Illustration one: recent commits on a named branch support offering `keep`. After a person confirms the row is alive, the agent can suppress its local reminder.

Illustration two: an open peer entry supports offering `record the wait`. A missing branch can also support offering `return to todo` for that row.

The agent changes no record fact before the answer. A suppression also waits until a person confirms that the row is alive.

Rules that bind this section: `scaffold:the-travelling-document-is-self-sufficient`, `metrics:a-marked-row-is-never-acted-on-without-an-answer`.
