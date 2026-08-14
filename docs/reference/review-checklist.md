# Review checklist

What a reviewer holds, including everything no gate can. Items marked `no gate` exist only here.

## Opening a story

- [ ] Type and points are legal; the point value names its judgments (no gate).
- [ ] `Core` is distinct from the ordered negotiable scope; work above three points was split.
- [ ] Every fixed heading is present, with an `Example` fence for a story or spike; the example demonstrates concrete reader-side behaviour (no gate on quality).
- [ ] Durable facts live in their owning documents, not in the story (no gate on leanness).
- [ ] No sentence narrates history — no "formerly", no "used to", no "this replaces", no provenance note (no gate).
- [ ] References state claims and link to anchors, not line coordinates; `Governed by` names individual sources; every `new:` marker is genuinely a created document (no gate).
- [ ] Acceptance assertions name tests; tasks are a checklist; every known trap has an escape.

## Changing the record

- [ ] The lane files and config match their schemas; every `summary` still describes its story (no gate).
- [ ] Ids unique and matching documents; dependencies acyclic; every `epic` resolves; no entry serves two ends.
- [ ] `doing` and `review` are clean; `todo` is eligible-first and dependency-ordered.
- [ ] A close records outcome and date, appends to the bottom of `closed.yml`, and every transition went through `move` — the journal agrees with the lanes.
- [ ] A `reshaped` successor carries the epic the work actually serves (no gate); the acceptance transfer happened (no gate).
- [ ] Only the repair reorders, only on request, and no hook invokes a writer.
- [ ] A drain's drift report was read and each drift acted on (no gate); a landed record's ranking was settled by a person.

## Changing an epic

- [ ] States an end state no single member delivers, shown concretely; `Done when` is not "every member closed".
- [ ] Lists no members, no status, no parent; every fixed heading present; `Amends` resolves, is `new:`, or is `None`.
- [ ] The epic earns a document — a one-member epic is a story, a label is a tag.
- [ ] An epic nobody is pursuing is retired (no gate).

## Changing the host binding

- [ ] `plan_dir` still true; moved documents have their references updated.
- [ ] A reference genuinely governs or is genuinely changed (no gate).

## Verification

- [ ] Links resolve; both document shapes pass the headings gate.
- [ ] Every schema, every lane file, every fragment, the config, and the checker: the record passes validation with no unintended warning.
- [ ] `pending.schema.json`'s entry mirror still agrees with `lane.schema.json`'s entry fields (no gate).
- [ ] Any changed gate was proven to fail once before being trusted.
