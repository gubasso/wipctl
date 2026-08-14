# wipctl documentation

wipctl is two deliverables that ship together and are judged as one: a planning method built on bounded stories, judgment-counting points, five lanes, and recorded transitions; and the tooling that proves a project's plan record obeys that method. The method without the tooling is advice; the tooling without the method is a linter for a format nobody explained. Neither changes alone.

This tree is organised by reader need. A reader finishing a task starts in `guides/`; a reader looking up an exact value, format, or contract starts in `reference/`; a reader learning how an area fits together starts in `explanation/`; a reader asking why the project chose a shape starts in `decisions/`.

## Reference — the normative specification

`reference/` is written so that a team holding nothing but this tree can build wipctl from scratch, in any language, honouring that language's own idioms. Nothing in it names a programming language, a library, or a concrete third-party tool as a requirement; where an external capability is needed, it states the contract that capability satisfies, and any tool meeting the contract serves. The one named tool is git: the opt-in transition commit is specified against git and no other version control, a recorded decision ([decisions/ADR-0036-the-program-speaks-git-and-no-other-vcs.md](./decisions/ADR-0036-the-program-speaks-git-and-no-other-vcs.md)).

- [reference/record/](./reference/record/zone-layout.md) — the data model: the zone layout, every file format, every field, every bound. An implementer starts here, because every verb and every check is defined against the data model.
- [reference/cli/](./reference/cli/conventions.md) — the command surface: global conventions and one contract page per verb under `reference/cli/verbs/`.
- [reference/validation/](./reference/validation/ownership-split.md) — what is checked, by which half, with which diagnostics, and the rules under which the tool may write the record.
- [reference/schemas/](./reference/schemas/README.md) — the shipped JSON Schemas, ready to gate a conforming implementation.
- [reference/rendering.md](./reference/rendering.md) — the constraints every terminal view obeys.
- [reference/scaffold-payload.md](./reference/scaffold-payload.md) — what `wipctl init` lands, and the contract each landed file carries.
- [reference/quality-gates.md](./reference/quality-gates.md) — what any implementation gates, and what it means to conform.
- [reference/review-checklist.md](./reference/review-checklist.md) — the human review gate, including the explicit list of what no gate catches.

## Explanation — the method and the design

- [explanation/charter.md](./explanation/charter.md) — what the product is for, its pillars, and its no-gos.
- [explanation/overview.md](./explanation/overview.md) — the method in one page. A team lead starts here.
- [explanation/stories-and-estimation.md](./explanation/stories-and-estimation.md), [explanation/lanes-and-ranking.md](./explanation/lanes-and-ranking.md), [explanation/epics.md](./explanation/epics.md), [explanation/transitions.md](./explanation/transitions.md), [explanation/concurrent-capture.md](./explanation/concurrent-capture.md) — why the record is shaped this way and how to think with it.
- [explanation/host-integration.md](./explanation/host-integration.md) — the single assumption about an adopting project, and the reference doctrine that follows from it.
- [explanation/architecture.md](./explanation/architecture.md) — the components a conforming implementation needs, by responsibility, never by code structure.

## Guides — task sequences

- [guides/adopting.md](./guides/adopting.md) — scaffold a zone, fill the charter, land the first story, wire the gates.
- [guides/writing-a-story.md](./guides/writing-a-story.md) — from an itch to a gated entry.
- [guides/running-an-epic.md](./guides/running-an-epic.md) — declare, populate, drive, and close out a goal larger than one story.
- [guides/wiring-host-docs.md](./guides/wiring-host-docs.md) — connect stories to the host's own documents.
- [guides/parallel-sessions.md](./guides/parallel-sessions.md) — capture in concurrent sessions and drain the result.
- [guides/building-wipctl.md](./guides/building-wipctl.md) — the milestone sequence for building the product from zero, with per-milestone acceptance.

## Decisions

[decisions/](./decisions/README.md) records why one option was chosen over serious alternatives, one lean record per choice. The reference states what is true; a decision states why. Nobody needs the decisions to build the product; they exist so nobody rebuilds a rejected alternative.

## Glossary

[glossary.md](./glossary.md) resolves every term of art at the page that owns it.
