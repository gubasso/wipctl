# wipctl

wipctl is a planning method and the tooling that proves a project's plan record obeys it, shipped
together and judged as one. The method plans with bounded stories, judgment-counting points, five
lanes, and recorded transitions. The tooling is one command, `wipctl`, that scaffolds a plan zone,
answers what to start next, validates the record, moves entries between lanes as recorded events,
mints identities for concurrent capture, lands captured work, and renders the record as terminal
views.

A project adopts wipctl by answering one question: where should the plan zone live. The zone is a
directory of plain files inside the project's own documentation directory — five lane files, one
document per story, one per epic, a charter, open questions, a transition journal, and an
on-demand capture directory. The single assumption about the adopting project is that such a
documentation directory exists; everything else belongs to the project.

## Documentation

Everything about the product lives under [docs/](./docs/README.md), organised by reader need:
guides for task sequences, reference for exact contracts and formats, explanation for the method
and the design, decisions for why. The reference zone is the normative specification this build
conforms to; `AGENTS.md` states how the two relate while the build is under way.

Start at [docs/explanation/overview.md](./docs/explanation/overview.md) to learn the method, or at
[docs/reference/record/zone-layout.md](./docs/reference/record/zone-layout.md) to build against
the specification.
