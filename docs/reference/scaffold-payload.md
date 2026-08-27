# Scaffold payload

What `wipctl init` lands, and the contract each landed file carries. The payload ships under the product's data directory; the scaffold MUST discover its members (top-level documents plus the lane files) rather than enumerating them, so a payload that gains a member gains it in the emit.

## Inventory

| Payload member                    | Lands at                       | Notes                                                                                                              |
| --------------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| config template                   | `<root>/.wipctl.toml`          | generated, not copied: resolved `plan_dir`, given or today's start date, `length_days = 14` as the one baked value |
| zone `README.md`                  | `<zone>/README.md`             | orientation: where to start, what bounds it, what stops it, what already happened                                  |
| `AGENTS.md`                       | `<zone>/AGENTS.md`             | the travelling method; gate placeholders substituted at emission                                                   |
| `charter.md`                      | `<zone>/charter.md`            | angle-bracketed placeholders for a person to fill                                                                  |
| `open-questions.md`               | `<zone>/open-questions.md`     | heading plus the `Blocks:` grammar; ships without a sample question, which would fail the zone's own first lint    |
| lane files                        | `<zone>/lanes/*.yml`           | each exactly `lane: <name>` plus `stories: []`                                                                     |
| —                                 | `<zone>/stories/`              | created empty                                                                                                      |
| heading-gate configs              | `<root>` lint config directory | only under `--headings-gate`: one required-headings array per document shape                                       |
| hook fragment                     | stdout                         | only via `--print-hooks`: entries with the zone path substituted, hand-copy instruction lines stripped             |
| templates (story, epic, question) | not copied                     | shipped location printed for a person to copy                                                                      |

The scaffold MUST NOT create `epics/` (earned), a first story (it would have to be deleted before the record could be believed), or `pending/` (created on demand by capture).

A payload member MUST NOT name a documentation method or the tooling that installs one. Which documents a host keeps durable, and under what method, is the host's alone (ADR-0033); a payload naming one would teach every scaffolded project to adopt it, and would make this method a dependent of that one. Naming a document class the host may or may not keep — a specification, a decision record — is the same fault at a smaller scale, and the reference states such a class only as one option among alternatives.

## The travelling method document

`AGENTS.md` MUST be self-sufficient: both heading sequences with a one-line purpose per heading; the lane semantics table; the entry field list, the points scale, and the canonical subset; the id grammar and the hand-mint incantation, with the rule that a collision is surfaced — by the gate for claimants in one record, by a version-control conflict between clones — and one side re-minted; eligibility, R1, R2, and the `Blocks:` grammar; the move-and-close procedure and why an edit is not a move; capture and drain, which lanes a fragment may claim, and what a drift report asks of a reader; the commit opt-in; the reference split (`Reads` inbound, `Amends` outbound, the `new:` marker, the optional typed rule delta, the acceptance transfer); the gate commands and the exit codes; and the explicit list of what no gate catches. Gate names are substituted from the emitting build; a verb the build lacks MUST be named absent, never promised. It MUST carry no relative links into any repository.

## The hook fragment

Five entries in the shipped fragment, each selecting by the substituted zone path: the lane files against `lane.schema.json`; the config against `config.schema.json`; the fragments against `pending.schema.json`, selected by shape (`pending/` anywhere under the zone) because the fragment set is open where the lane set is fixed; the two heading-shape checks; and the record gate itself — `wipctl validate <zone>` — which MUST run whole-record with filename passing disabled, because every rule is cross-file and a changed-files-only invocation would let a rename slip through half-staged. The fragment MUST state the two standing rules in comments: the record gates register at both commit and push stages, and a hook never invokes a writer.
