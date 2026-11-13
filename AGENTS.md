# AGENTS

Entry point for anyone, human or agent, working in this repository. `README.md` orients a reader. This file holds the rules.

## What the product is

wipctl is two deliverables that ship together and are judged as one: a planning method, and the tooling that proves a project's plan record obeys it. Neither one changes alone. The full statement of pillars and no-gos is `docs/explanation/charter.md`.

The project plans with the method it ships. Its own plan record lives in the project's plan repository, resolved through `.wipctl/project.toml` at the root, per `docs/specs/SPEC-attachment.md`. The record is not in this tree. A clone of this repository carries the identity, and the record is one attach away.

## The spec is normative

`docs/specs/` is the complete, language-agnostic specification of the product, and this build conforms to it. Where the code and a spec disagree, the spec wins. The divergence is either a defect in the code or a deliberate change. A deliberate change edits the spec in the same change as the behavior, gated by an ADR when the choice is significant. This is a stated local exception to the usual rule that code beats prose for current behavior.

A requirement's rule id is how code and gates cite what binds them. Cite the id, never a decision record.

The specs stay language-agnostic. They never name a programming language, a library, a package manager, or a concrete third-party tool as a requirement. They state the contract a capability satisfies, and any tool meeting the contract serves. Implementation choices belong in the code and its decision records, never in a spec. The one tool the specs name is git. It is the plan repository's replication substrate, specified against git and no other version control, per ADR-git-is-the-replication-substrate.

Public language follows `docs/specs/SPEC-vocabulary.md`. A domain, concept, field, or field value uses one lean, plain-language name that states what it means.

## Before 1.0, correctness beats compatibility

The product is below version 1.0, so its contract is not frozen. Where the right design needs a breaking change, the change lands. A compatibility shim, a fallback reader, or a second accepted form is never a reason to keep a worse design.

A breaking change owes three things in the change that lands it. The spec states the new contract alone and carries no trace of the old one. The release request and the commit body name what breaks and what an adopter does next. The commit carries the Conventional Commits breaking marker, so the release tooling computes the bump instead of an author asserting it.

The changelog is release-plz's. It writes `CHANGELOG.md` from the commit subjects in its own form, and the checks that rewrite or judge Markdown exclude it. Write the adopter guidance as a `BREAKING CHANGE:` footer in the squash body, at the start of its own line, and repeat it in the release request. Never edit `CHANGELOG.md` by hand: an entry added under `Unreleased` collides with the generated section and stops the release on a duplicate heading. The `breaking-change-carries-a-footer` hook holds the footer.

Migration is a verb the product offers, never a shape the product keeps.

## The repository is self-contained

Everything needed to understand, build, or gate this project lives in it. An outbound link is a citation for further reading, never a prerequisite. It is never a dependency on an external, local, personalized, or mutating path. Substance borrowed from elsewhere is restated here in the zone that owns it. The binding statement of the rule, and the clone-on-a-bare-machine test that settles whether it holds, are in `docs/specs/SPEC-quality-gates.md`. Read it before citing anything outside this repository. ADR-the-repository-is-self-contained records why the project is built this way.

## Where things go

- `docs/specs/`: one spec per domain. Every binding, verifiable rule, with its id and the command that decides it.
- `docs/guides/`: task sequences a reader follows to finish something.
- `docs/reference/`: exact values and templates a reader looks up.
- `docs/explanation/`: the method, the charter, and the architecture. How areas fit together.
- `docs/decisions/`: one lean ADR per significant, hard-to-reverse choice.
- `docs/glossary.md`: every term of art, resolved at the page that owns it.
- `.draft/`: the gitignored workshop for material that is not yet canonical. Promotion is a rewrite into the owning zone, never a move.

## Gates

This repository owes its own toolchain a hook, a test case, a pinned environment, and dogfooding. `docs/specs/SPEC-quality-gates.md` specifies all four. Every shipped artifact carries those obligations in the same change that ships it.

The commit stage and the push stage of the hooks together run every check CI runs, per `quality-gates:the-local-stages-hold-every-ci-check`. The push stage holds the costly checks: the integration tests, the builds, and the networked checks. It repeats none of the commit stage. A new check is a new hook at one of the two stages, never a new CI job. `README.md` lists the checks that only CI can run, with the reason for each.

Report a documentation change by ownership: which source of truth changed, and which links were added.

<!-- BEGIN release-kit -->

## Releases

- This repository runs the release-kit convention. `rk method invariants` states what must stay true. A release needs a commit typed `feat`, `fix`, or `perf`, or a breaking change, that changed a file the package ships. A dependency update that changes what ships is `fix(deps)`, and one for development only is `chore(deps-dev)` or `build(deps-dev)`. A security update is always `fix(deps)`, also where it changes only a shipped lock file. Its body names the advisory ID: `RUSTSEC-`, `GHSA-`, or `CVE-`.
- An agent here guides and never drives. It reads this convention and tells the operator which step comes next. It takes no git or forge action unless the operator's request named that action. The bounded actions include the following. Create, switch, or delete a branch. Mint a branch at the forge from an issue. Create or remove a worktree. Commit, push, or tag. Integrate a branch onto the trunk, locally or through a request. Open, update, or merge a pull request. A request to change code authorizes the file changes alone.
- The workflow terms live in `GLOSSARY.md` at this repository's root. The terms release-kit owns there are `implement-and-request`, `implement-and-merge`, and `full-implement`. A request that carries one of them names every action that term lists.
- The title of every commit that reaches the trunk MUST be a scoped Conventional Commit, because the version and the changelog come from it.
- The request's title becomes that commit's message, and its body carries the context and lands with it. The body names no internal planning artifact and carries no agent attribution. The landed rk-message hook, the forge's body check, and the observed body source hold that rule.
- Every commit follows the same scoped convention. The landed commit-msg hook requires a scope on every one, and the title check holds it to lowercase letters, digits, and `_ . / -`.
- The scope names the area you changed, and reads as `area/subarea` where that is clearer. Prefer a scope this repository already uses, which `git log --format=%s | sed -n 's/^[a-z]*(\([^)]*\)).*/\1/p' | sort -u` lists. Coin a new scope only where no existing one names the area.
- Never author a tag, and never hand-edit a generated artifact workflow.
- Run `rk status` before changing anything under `.github/workflows/` or `.gitlab-ci.yml`, or any file `.release-kit/manifest.json` names. Before reading a gap in `rk setup check` as a setup defect, run `rk setup status`. A credential this runtime cannot reach leaves a step unknown, which is an observation boundary and not drift. Never ask the operator to mount, expose, recreate, or rotate a secret only so an agent check can pass.
- The full method is `rk method --list`. The recovery paths are `rk method recovery`.

<!-- END release-kit -->

<!-- BEGIN spec-driven-docs docs -->
## Documentation

- Load the affected specs before editing governed content: `docs/specs/SPEC-<domain>.md`. <!-- markdownlint-disable-line MD013 -->
- Treat decision records as immutable rationale and load them only when asked why. <!-- markdownlint-disable-line MD013 -->
- Read the writing style before you author or edit prose: `sdd method writing-style`. <!-- markdownlint-disable-line MD013 -->
- Write and edit step-by-step guides to the adopted guides spec, `docs/specs/SPEC-guides.md`. <!-- markdownlint-disable-line MD013 -->
- Name a document by a slug drawn from its subject, never by a number. Where its directory holds documents with no kind prefix, give that directory a `README.md` saying what it holds and what each document covers. <!-- markdownlint-disable-line MD013 -->
- For a question about this method or about operating `sdd`, run `sdd docs` and read only the topic it names. <!-- markdownlint-disable-line MD013 -->
- Run `sdd verify` before handoff. <!-- markdownlint-disable-line MD013 -->
- Keep adopted specs, the tracking registry, and local integration instance-owned. <!-- markdownlint-disable-line MD013 -->
<!-- END spec-driven-docs docs -->
