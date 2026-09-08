# wipctl documentation

wipctl is two deliverables that ship together and are judged as one. The first is a planning method, built on bounded stories, judgment-counting points, five lanes, and recorded transitions. The second is the tooling that proves a project's plan record obeys that method. The method without the tooling is advice. The tooling without the method is a linter for a format nobody explained. Neither changes alone.

This tree is organised by reader need:

- a reader finishing a task starts in `guides/`
- a reader looking up a binding rule starts in `specs/`
- a reader learning how an area fits together starts in `explanation/`
- a reader asking why the project chose a shape starts in `decisions/`

## Specifications — the normative specification

`specs/` is written so that a team holding nothing but this tree can build wipctl from scratch, in any language, honouring that language's own idioms. Nothing in it names a programming language, a library, or a concrete third-party tool as a requirement. Where an external capability is needed, it states the contract that capability satisfies, and any tool meeting the contract serves. The one named tool is git. It is the plan repository's replication substrate, specified against git and no other version control. [decisions/ADR-git-is-the-replication-substrate.md](./decisions/ADR-git-is-the-replication-substrate.md) records that choice.

One spec per domain, each requirement carrying a rule id, an example scenario, and the command that decides it. An implementer starts at the data model, because every verb and every check is defined against it.

- The data model: [SPEC-zone-layout.md](./specs/SPEC-zone-layout.md), [SPEC-lane-file.md](./specs/SPEC-lane-file.md), [SPEC-documents.md](./specs/SPEC-documents.md), [SPEC-ids.md](./specs/SPEC-ids.md), [SPEC-transition-journal.md](./specs/SPEC-transition-journal.md), [SPEC-pending-fragment.md](./specs/SPEC-pending-fragment.md), [SPEC-open-questions.md](./specs/SPEC-open-questions.md).
- What a plan declares about itself and about others: [SPEC-configuration.md](./specs/SPEC-configuration.md), [SPEC-peers.md](./specs/SPEC-peers.md).
- The command surface: [SPEC-cli-conventions.md](./specs/SPEC-cli-conventions.md), [SPEC-messages.md](./specs/SPEC-messages.md), [SPEC-transactions.md](./specs/SPEC-transactions.md), [SPEC-manual.md](./specs/SPEC-manual.md).
- The verbs: [SPEC-capture.md](./specs/SPEC-capture.md), [SPEC-transitions.md](./specs/SPEC-transitions.md), [SPEC-drain.md](./specs/SPEC-drain.md), [SPEC-deletion.md](./specs/SPEC-deletion.md), [SPEC-rename.md](./specs/SPEC-rename.md), [SPEC-attachment.md](./specs/SPEC-attachment.md), [SPEC-peer-attachment.md](./specs/SPEC-peer-attachment.md), [SPEC-sync.md](./specs/SPEC-sync.md).
- The views: [SPEC-reporting.md](./specs/SPEC-reporting.md), [SPEC-metrics.md](./specs/SPEC-metrics.md), [SPEC-epics.md](./specs/SPEC-epics.md), [SPEC-initiatives.md](./specs/SPEC-initiatives.md), [SPEC-rendering.md](./specs/SPEC-rendering.md).
- What is checked and by which half: [SPEC-validation.md](./specs/SPEC-validation.md), [SPEC-ranking.md](./specs/SPEC-ranking.md), [SPEC-repair.md](./specs/SPEC-repair.md), [SPEC-doctor.md](./specs/SPEC-doctor.md).
- [SPEC-external-sources.md](./specs/SPEC-external-sources.md): naming an item in a system outside the plan repository.
- [SPEC-scaffold.md](./specs/SPEC-scaffold.md): what the scaffold lands, and the contract each landed file carries.
- [SPEC-quality-gates.md](./specs/SPEC-quality-gates.md): what any implementation gates, and what it means to conform.
- [SPEC-review.md](./specs/SPEC-review.md): the human review gate, holding every rule no command can decide.

The shipped JSON Schemas live beside the spec that owns each shape, in the companion directory named for it.

## Explanation — the method and the design

- [explanation/charter.md](./explanation/charter.md): what the product is for, its pillars, and its no-gos.
- [explanation/overview.md](./explanation/overview.md): the method in one page. A team lead starts here.
- [explanation/stories-and-estimation.md](./explanation/stories-and-estimation.md), [explanation/lanes-and-ranking.md](./explanation/lanes-and-ranking.md), [explanation/epics.md](./explanation/epics.md), [explanation/initiatives.md](./explanation/initiatives.md), [explanation/transitions.md](./explanation/transitions.md), [explanation/concurrency.md](./explanation/concurrency.md), and [explanation/replication.md](./explanation/replication.md): why the record is shaped this way.
- [explanation/host-integration.md](./explanation/host-integration.md): the single assumption about an adopting project, and the doctrine that follows from it.
- [explanation/architecture.md](./explanation/architecture.md): the components a conforming implementation needs, by responsibility and never by code structure.

## Guides — task sequences

- [guides/adopting.md](./guides/adopting.md): scaffold the identity and the plan repository, fill the charter, land the first story, prove the gates.
- [guides/writing-a-story.md](./guides/writing-a-story.md): from an itch to a gated entry.
- [guides/running-an-epic.md](./guides/running-an-epic.md): declare, populate, drive, and close out a goal larger than one story.
- [guides/running-an-initiative.md](./guides/running-an-initiative.md): declare, watch, and retire an end state larger than one epic.
- [guides/wiring-host-docs.md](./guides/wiring-host-docs.md): connect stories to the host's own documents.
- [guides/parallel-sessions.md](./guides/parallel-sessions.md): several agents on one record, and a second machine when you leave the first.
- [guides/building-wipctl.md](./guides/building-wipctl.md): the milestone sequence for building the product from zero, with per-milestone acceptance.

## Decisions

`decisions/` records why one option was chosen over serious alternatives, one lean record per choice, each named for the choice it settles. A spec states what is true, and a decision states why. Nobody needs the decisions to build the product. They exist so nobody rebuilds a rejected alternative.

## Glossary

[glossary.md](./glossary.md) resolves every term of art at the page that owns it.
