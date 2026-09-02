# Scaffold payload

What `wipctl init` lands, and the contract each landed file carries. The scaffold writes into two places — one identity file into the host repository, and the plan repository into this machine's attachment registry ([cli/verbs/init.md](./cli/verbs/init.md)). The payload ships under the product's data directory; the scaffold MUST discover its members (top-level documents plus the lane files) rather than enumerating them, so a payload that gains a member gains it in the emit.

## Inventory

| Payload member                                | Lands at                   | Notes                                                                                                           |
| --------------------------------------------- | -------------------------- | --------------------------------------------------------------------------------------------------------------- |
| host identity file                            | `<root>/.wipctl.toml`      | generated, not copied: the minted `project_id` and nothing else — the one file the host receives                |
| plan config                                   | `<zone>/config.toml`       | generated: the minted id restated, the given or today's start date, `length_days = 14` as the one baked value   |
| zone `README.md`                              | `<zone>/README.md`         | orientation: where to start, what bounds it, what stops it, what already happened                               |
| `AGENTS.md`                                   | `<zone>/AGENTS.md`         | the travelling method; gate placeholders substituted at emission                                                |
| `charter.md`                                  | `<zone>/charter.md`        | angle-bracketed placeholders for a person to fill                                                               |
| `open-questions.md`                           | `<zone>/open-questions.md` | heading plus the `Blocks:` grammar; ships without a sample question, which would fail the zone's own first lint |
| lane files                                    | `<zone>/lanes/*.yml`       | each exactly `lane: <name>` plus `stories: []`                                                                  |
| —                                             | `<zone>/stories/`          | created empty                                                                                                   |
| hook set                                      | the plan repository        | validate at commit and push, the single-branch guard, the no-force guard, the three heading-shape checks        |
| templates (story, epic, initiative, question) | not copied                 | shipped location printed for a person to copy                                                                   |

The zone here is the plan repository's working tree, which the scaffold creates whole: the repository, the trunk, the hook set, and the initial commit. Installing hooks into it is the scaffold writing what it created (ADR-0011); the host repository still receives exactly one file it did not have, and no merged content.

The scaffold MUST NOT create `epics/` or `initiatives/` (earned, not seeded), a first story (it would have to be deleted before the record could be believed), or `pending/` (created on demand by capture).

A payload member MUST NOT name a documentation method or the tooling that installs one. Which documents a host keeps durable, and under what method, is the host's alone (ADR-0033); a payload naming one would teach every scaffolded project to adopt it, and would make this method a dependent of that one. Naming a document class the host may or may not keep — a specification, a decision record — is the same fault at a smaller scale, and the reference states such a class only as one option among alternatives.

## The travelling method document

`AGENTS.md` MUST be self-sufficient, and it is the one channel through which the method reaches an adopting project, so it MUST teach:

- Where the plan lives: the two-location model — one identity file in the host, one plan repository at the machine-level slot shared by every checkout — and that a reader who has only ever seen a zone beside the code is reading about a different arrangement, taught from zero.
- All three heading sequences with a one-line purpose per heading, the `Initiative` section rule, the depth rule, and that membership above the story flows through the epic.
- The lane semantics table; the entry field list, the points scale, and the canonical subset.
- The id grammar, the slugification function, the mint's check against the live record and every tombstone, the rephrase resolution, and the hand-mint incantation: slugify the title, confirm nothing live or tombstoned holds the slug, keep the filename, the title line, and any fragment in agreement. Nothing random is drawn.
- Eligibility, R1, R2, and the `Blocks:` grammar.
- The move-and-close procedure and why an edit is not a move; `start` beside `next`, and what an agent holds while it works — which is nothing.
- Capture and drain, which lanes a fragment may claim, and what a drift report asks of a reader.
- That every plan mutation is committed to the plan trunk by the tool, with no opt-out, and that the host's own history receives nothing; `sync` when leaving a machine.
- The reference split (`Reads` inbound, `Amends` outbound, the `new:` marker, the optional typed rule delta, the acceptance transfer as a same-unit-of-review obligation).
- The gate commands and the exit codes, and the explicit list of what no gate catches.

Gate names are substituted from the emitting build; a verb the build lacks MUST be named absent, never promised. It MUST carry no relative links into any repository.

## The hook set

Installed into the plan repository, not the host — the plan repository is tool-owned, and the record's gates live with the record. The set: the record gate itself — `wipctl validate`, run whole-record at both the commit and push stages, with filename passing disabled, because every rule is cross-file; the schema checks for the lane files, the fragments, and the config; the three heading-shape checks; the single-branch guard; and the no-force guard. Two standing rules travel as comments in the installed set: the record gates run at both stages, and a hook never invokes a writer. `init --print-hooks` prints the set for inspection, and a missing installation is a `doctor` failure.
