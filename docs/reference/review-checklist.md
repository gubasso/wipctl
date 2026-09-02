# Review checklist

What a reviewer holds, including everything no gate can. Items marked `no gate` exist only here.

## Opening a story

- [ ] Type and points are legal; the point value names its judgments (no gate).
- [ ] `Core` is distinct from the ordered negotiable scope; work above three points was split.
- [ ] Every fixed heading is present, with an `Example` fence for a story or spike; the example demonstrates concrete reader-side behaviour (no gate on quality).
- [ ] Durable facts live in their owning documents, not in the story (no gate on leanness).
- [ ] No sentence narrates history — no "formerly", no "used to", no "this replaces", no provenance note (no gate).
- [ ] References state claims and link to anchors, not line coordinates; `Reads` names individual sources and honours whatever the host's documentation method keeps out of a session's load set; every `new:` marker is genuinely a created document (no gate).
- [ ] Acceptance assertions name tests; tasks are a checklist; every known trap has an escape.

## Changing the record

- [ ] The lane files and config match their schemas; every `summary` still describes its story (no gate).
- [ ] Ids unique and matching documents; dependencies acyclic; every `epic` resolves; no entry serves two ends.
- [ ] `doing` and `review` are clean; `todo` is eligible-first and dependency-ordered.
- [ ] A close records outcome and date, appends to the bottom of `closed.yml`, and every transition went through `move` or `start` — the journal agrees with the lanes.
- [ ] A `reshaped` successor carries the epic the work actually serves (no gate).
- [ ] The acceptance transfer happened in the same unit of review as the close — the story's assertions rewritten in the present tense into the amended documents, reviewed together even though no gate can span the two repositories (no gate). The code references are honest: the `branch` names the branch that carried the work, and `delivered` names the commit that landed it (no gate).
- [ ] A story that changed a rule the host identifies by id declared the typed delta under `Amends`, and the declared type matches the diff (no gate).
- [ ] A renamed entry is one reviewable act: the title and the id agree, the old id is burned by a `renamed` tombstone, every reference moved with it, and the rename is a dated line under `Revisions`.
- [ ] Only the repair reorders, only on request, and no hook invokes a writer.
- [ ] A drain's drift report was read and each drift acted on (no gate); a landed record's ranking was settled by a person.
- [ ] A sync conflict was decided by a person — `resolve` recorded the decision, and nobody merged by hand (no gate).

## Changing an epic

- [ ] States an end state no single member delivers, shown concretely; `Done when` is not "every member closed".
- [ ] Lists no members and no status; every fixed heading present; `Amends` resolves, is `new:`, or is `None`.
- [ ] The `Initiative` section names a real document or `None`; the epic is not an initiative with one member wearing a container.
- [ ] The epic earns a document — a one-member epic is a story, a label is a tag.
- [ ] An epic nobody is pursuing is retired (no gate).

## Changing an initiative

- [ ] States an end state no single member epic delivers; `Done when` is observable, not "every member closed".
- [ ] Lists no members, carries no `Initiative` section of its own, and nothing sequences through it.
- [ ] The initiative earns a document — an initiative with one epic is an epic wearing a container, and an initiative per quarter or per release is a calendar, not an end state.
- [ ] An initiative nobody is pursuing is retired (no gate).

## Changing the host binding

- [ ] The two identity files agree, and the project resolves from every checkout that should reach it.
- [ ] Moved host documents have their `Reads` and `Amends` references updated.
- [ ] A reference genuinely governs or is genuinely changed (no gate).

## Verification

- [ ] Links resolve; all three document shapes pass the headings gate.
- [ ] Every schema, every lane file, every fragment, the config, and the checker: the record passes validation with no unintended warning.
- [ ] `pending.schema.json`'s entry mirror still agrees with `lane.schema.json`'s entry fields (no gate).
- [ ] Any changed gate was proven to fail once before being trusted.
