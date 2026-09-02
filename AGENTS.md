# AGENTS

Entry point for anyone — human or agent — working in this repository. `README.md` orients a reader; this file holds the rules.

## What the product is

wipctl is two deliverables that ship together and are judged as one: a planning method, and the tooling that proves a project's plan record obeys it. Neither may be changed alone. The full statement of pillars and no-gos is `docs/explanation/charter.md`.

The project plans with the method it ships. Its own plan record lives in the project's plan repository — resolved through `.wipctl.toml` at the root, per `docs/reference/record/resolution.md` — and not in this tree: a clone of this repository carries the identity, and the record is one attach away.

## The spec is normative

`docs/reference/` is the complete, language-agnostic specification of the product, and this build conforms to it. Where the code and the reference disagree, the reference wins: the divergence is either a defect in the code or a deliberate change, and a deliberate change edits the reference — gated by an ADR when the choice is significant — in the same change as the behavior. This is a stated local exception to the usual rule that code beats prose for current behavior.

The reference stays language-agnostic. It never names a programming language, a library, a package manager, or a concrete third-party tool as a requirement; it states the contract a capability satisfies, and any tool meeting the contract serves. Implementation choices belong in the code and its decision records, never in the reference pages. The one tool the reference names is git: it is the plan repository's replication substrate, specified against git and no other version control, per ADR-0048.

## The repository is self-contained

Everything needed to understand, build, or gate this project lives in it. An outbound link is a citation for further reading, never a prerequisite, and never a dependency on an external, local, personalized, or mutating path. Substance borrowed from elsewhere is restated here in the zone that owns it. The binding statement of the rule, and the clone-on-a-bare-machine test that settles whether it holds, are in `docs/reference/quality-gates.md`; read it before citing anything outside this repository. ADR-0039 records why the project is built this way.

## Where things go

- `docs/guides/` — task sequences a reader follows to finish something.
- `docs/reference/` — exact contracts, formats, values, and the review checklist.
- `docs/explanation/` — the method, the charter, and the architecture: how areas fit together.
- `docs/decisions/` — one lean ADR per significant, hard-to-reverse choice.
- `docs/glossary.md` — every term of art, resolved at the page that owns it.
- `.draft/` — the gitignored workshop for material that is not yet canonical. Promotion is a rewrite into the owning zone, never a move.

## Documentation maintenance

- Write each durable fact once at its owning home and cross-link from everywhere else; a cross-link names the reason to follow it.
- Name every ADR `ADR-<number>-<decision>.md`, keep its filled body at or below 350 words, use the five sections of `docs/decisions/template.md`, and give it exactly one `Status`.
- Never delete an accepted decision. Supersede, deprecate, reject, or amend it; a record changed only in part keeps its status and gains an `Amended by ADR-NNNN` line under `Status`.
- Keep the current design in the reference and explanation pages; ADR bodies are frozen once accepted and are never rewritten to describe the present.
- No document narrates its own history: no `formerly`, no `used to`, no `this replaces`, no note explaining an absence. A decision record's context is the one home for it.
- Let the filesystem own its state: a list, table, or tree stays only when its entries teach something beyond what is on disk.
- Use no bold and no italics. Put identifiers, paths, flags, and statuses in inline code, and give every fenced block a language, `text` when none applies.
- Binding requirements use uppercase RFC 8174 keywords, and only in `docs/reference/`; narrative zones stay lowercase.
- Preserve lowercase `<angle>` placeholders in project-agnostic material; upper-snake `<TOKENS>` are guide artifacts and never carry a real or realistic value.
- Report documentation changes by ownership: which source of truth changed and which links were added.

## Gates

What this repository owes its own toolchain — hooks, tests, pinned environment, dogfooding — is specified in `docs/reference/quality-gates.md`, and every shipped artifact carries those obligations in the same change that ships it.
