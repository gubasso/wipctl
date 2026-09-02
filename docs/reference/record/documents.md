# Story, epic, and initiative documents

One Markdown document per story under `stories/`, one per epic under `epics/`, one per initiative under `initiatives/`. The filename stem is the id; the title line repeats it: `# <id> — <short title>`. The title MUST slugify to the id, or to the id minus its qualifying postfix ([ids.md](./ids.md)), so a document whose heading and filename have drifted apart in any other way is a failure, not a curiosity.

## Story headings, in exact order

Every heading MUST be present, in this order, as `##` sections:

1. `Goal` — the outcome, not the mechanism.
2. `Example` — the change shown from the reader's side, with concrete values and real output. A `story` or `spike` MUST carry a fenced block here; the fence MUST declare a language, `text` when none applies. A description of a transcript is not a transcript.
3. `Core` — what is never cut.
4. `In scope` — the ordered negotiable remainder, cut from the bottom. Correctness, tests, review, and security MUST NOT be listed here, because they are never cut.
5. `Out of scope` — what this story deliberately does not do.
6. `Reads` — the individual sources a work session MUST load before starting. Inbound references; see [../../explanation/host-integration.md](../../explanation/host-integration.md).
7. `Amends` — the documents the work MUST leave changed. Outbound references, gated. Carries paths or the literal `None` — one of the two headings that MUST NOT be left empty.
8. `Acceptance` — falsifiable assertions, each naming the test that proves it: `- <Assertion> — <test name>.` The point value counts the judgments in this section a test cannot settle. The story closes when every assertion's named test passes and every counted judgment is made; the story has no separate closing heading.
9. `Tasks` — a checkbox list. Tasks never become records of their own; MAY be empty.
10. `Rabbit holes` — known traps with pre-authorised escapes: `- <trap> — escape: <response>`. An escape written after hitting the trap is a revision.
11. `Revisions` — dated one-line entries recording changes to the agreement made after the story's own work began; MAY be empty. Cutting planned negotiable scope needs no entry; changing the agreement — a rename included — does.

A heading with nothing to say MUST carry `None`, except `Tasks` and `Revisions`, which MAY be empty lists.

## Epic headings, in exact order

1. `Initiative` — the end state above this one, or the literal `None`. First, before `Goal`, because the end state an epic serves scopes everything the reader is about to read. The rules are below.
2. `Goal` — the end state no single member delivers, written as an end state, not a summary.
3. `Example` — a simulation: what is true today against what is true at the end state.
4. `Core`
5. `Out of scope`
6. `Reads`
7. `Amends` — same rules as a story's.
8. `Done when` — MUST NOT read "every member closed": a cut member closes without delivering, so the epic states what is observably true at the end instead. Where the host keeps rule ids, the end state MAY be a set of rule ids whose verification commands all pass.
9. `Revisions`

Four story headings are deliberately absent from an epic. `In scope` needs a point budget an epic does not have. `Acceptance`, `Tasks`, and `Rabbit holes` are obligations on a work session, and no session implements an epic. `Done when` runs the other way — present on an epic and absent from a story — because a story's `Acceptance` is its closing condition and an epic has no `Acceptance` to close it.

## Initiative headings, in exact order

1. `Goal` — the end state no single member epic delivers, written as an end state.
2. `Example` — a simulation: what is true today against what is true at the end state.
3. `Core`
4. `Out of scope`
5. `Reads`
6. `Amends` — same rules as a story's and an epic's.
7. `Done when` — MUST NOT read "every member closed", by the same rule and for the same reason as an epic's.
8. `Revisions`

This is the epic sequence minus `Initiative`, and the symmetry is deliberate: one heading separates the two shapes, and it is the heading that states the tier. The four story headings absent from an epic are absent here for the same reasons, one step stronger — no session implements an initiative either. An initiative document MUST NOT carry an `Initiative` section: the ladder is exactly three tiers, an initiative has no parent, and the heading gate refuses the extra heading, so the no-parent rule needs no separate check.

## The `Initiative` section

Where an epic declares the end state it serves:

```markdown
# session-hardening — Session hardening

## Initiative

`trustworthy-by-default`

## Goal
```

- The section MUST carry exactly one inline-code id token, or the literal `None`. It is the one heading besides `Amends` that MUST NOT be left empty.
- The token MUST name an existing document under `initiatives/`. The reverse does not hold: an initiative no epic has joined is a legal first draft.
- Only the leading inline-code token is read as an id; anything after it is prose — the same rule `Reads` and `Amends` already carry.

## Path references

Under `Reads` and `Amends`, in all three shapes, each list item MUST open with the referenced path as an inline-code token, followed by a prose assertion. Only that leading inline-code token is treated as a path; identifiers appearing later in the sentence are prose. An assertion opening with `new:` (after any separator such as an em dash) promises a document the work will create: the path is exempt from the existence check for the whole life of the entry, while every shape rule still applies. Stub files MUST NOT be created to satisfy a check.

An `Amends` assertion MAY carry a typed rule delta: after the separator — and after `new:` where both appear — the assertion opens with `ADDED`, `MODIFIED`, or `REMOVED` followed by exactly one rule id as an inline-code token matching `[a-z0-9-]+:[a-z0-9-]+`. The clause states which rule in the amended document the work adds, rewords, or retires. When a clause is present its shape is gated; whether a story that changed a rule declared one is a review responsibility, because the program asserts nothing about which host documents keep rule ids. One rule per list item: a story amending three rules of one document carries three items.

```markdown
## Amends

- `docs/specs/SPEC-auth.md` — ADDED `auth:token-expiry-is-bounded`
- `docs/specs/SPEC-auth.md` — MODIFIED `auth:refresh-requires-reauth`
```

## Shape gating

All three heading sequences are gated by a required-headings check — one configuration per shape, each holding exactly one ordered headings array — wired to select `stories/*.md`, `epics/*.md`, and `initiatives/*.md` respectively. The gate proves every required heading is present and no extra one was added. It cannot prove a section was filled in, and it cannot express the Example-fence rule, the title-line identity, or the `Initiative` token's resolution, which belong to the cross-file checker.

## Sibling artifacts

A story with non-narrative artifacts (a measurement, a prototype input, a dataset) keeps them in a sibling directory `stories/<id>/`. The document narrates; the directory holds.
