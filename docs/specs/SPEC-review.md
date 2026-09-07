# Review Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`review:a-durable-fact-lives-in-its-owning-document` — A durable fact lives in its owning document](#reviewa-durable-fact-lives-in-its-owning-document--a-durable-fact-lives-in-its-owning-document)
  - [`review:no-sentence-narrates-history` — No sentence narrates history](#reviewno-sentence-narrates-history--no-sentence-narrates-history)
  - [`review:a-reference-links-to-an-anchor` — A reference links to an anchor](#reviewa-reference-links-to-an-anchor--a-reference-links-to-an-anchor)
  - [`review:the-acceptance-transfer-is-one-unit-of-review` — The acceptance transfer is one unit of review](#reviewthe-acceptance-transfer-is-one-unit-of-review--the-acceptance-transfer-is-one-unit-of-review)
  - [`review:a-code-reference-is-honest` — A code reference is honest](#reviewa-code-reference-is-honest--a-code-reference-is-honest)
  - [`review:a-rename-is-one-reviewable-act` — A rename is one reviewable act](#reviewa-rename-is-one-reviewable-act--a-rename-is-one-reviewable-act)
  - [`review:a-container-earns-its-document` — A container earns its document](#reviewa-container-earns-its-document--a-container-earns-its-document)
  - [`review:an-unpursued-container-is-retired` — An unpursued container is retired](#reviewan-unpursued-container-is-retired--an-unpursued-container-is-retired)
  - [`review:a-drift-report-is-read-and-acted-on` — A drift report is read and acted on](#reviewa-drift-report-is-read-and-acted-on--a-drift-report-is-read-and-acted-on)
  - [`review:a-moved-host-document-carries-its-references` — A moved host document carries its references](#reviewa-moved-host-document-carries-its-references--a-moved-host-document-carries-its-references)
  - [`review:a-declared-delta-matches-the-diff` — A declared delta matches the diff](#reviewa-declared-delta-matches-the-diff--a-declared-delta-matches-the-diff)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

What a reviewer holds that no command can decide. Every requirement here is verified by a person, and each one exists because the fact it protects is a judgment rather than a match. The boundary runs at decidability: a rule a command can settle belongs to the domain that owns the file, and this domain holds only what is left.

## Requirements

### `review:a-durable-fact-lives-in-its-owning-document` — A durable fact lives in its owning document

The author MUST write a durable fact in the document that owns it, and MUST NOT restate it in a work entry.

#### Scenario: A story explains a format it depends on

- GIVEN a story restating a rule the specification already carries
- WHEN the story closes
- THEN the restatement outlives the work and drifts, so the story cites the owner instead

Verify: reviewer confirms the entry states no durable fact its owning document holds

### `review:no-sentence-narrates-history` — No sentence narrates history

The author MUST NOT write what a document used to say, what it replaces, or why something is absent.

#### Scenario: A rule is removed from a document

- GIVEN an author removing a rule
- WHEN they add a note explaining the removal
- THEN the note is refused, because the deletion belongs to the log and the document states the present

Verify: reviewer confirms the change adds no note narrating a past state

### `review:a-reference-links-to-an-anchor` — A reference links to an anchor

A reference MUST state its claim and MUST link to an anchor rather than to a line coordinate.

#### Scenario: A reference points at a line number

- GIVEN a reference naming a file and a line
- WHEN the file is edited
- THEN the coordinate is wrong and the claim is unreadable, so the reference names the anchor and states what it asserts

Verify: reviewer confirms each reference states a claim and names an anchor

### `review:the-acceptance-transfer-is-one-unit-of-review` — The acceptance transfer is one unit of review

When an entry closes, the author MUST rewrite its acceptance assertions into the amended documents, in the same unit of review.

#### Scenario: A close lands before the documents change

- GIVEN an entry closed in one review and its documents amended in another
- WHEN the first review ends
- THEN the agreed behavior is recorded nowhere, and no gate can span the two repositories to catch it

Verify: reviewer confirms the close and its document amendments were reviewed together

### `review:a-code-reference-is-honest` — A code reference is honest

The author MUST name the branch that carried the work and the commit that landed it, where the entry records them.

#### Scenario: A branch field names a different branch

- GIVEN a closed entry whose branch field is stale
- WHEN a reader follows it
- THEN it resolves to the wrong work, and no check can tell, because the record cannot verify another repository

Verify: reviewer confirms the branch and delivered fields name the work that landed

### `review:a-rename-is-one-reviewable-act` — A rename is one reviewable act

A rename MUST land as one act, with the title and id in agreement, every reference moved, and a dated revision line added.

#### Scenario: A rename resolves an id collision

- GIVEN a rename that frees no id
- WHEN it is reviewed
- THEN no tombstone is expected, because the id was never freed

Verify: reviewer confirms the rename moved every reference and recorded its revision

### `review:a-container-earns-its-document` — A container earns its document

An epic and an initiative MUST each state an end state no single member delivers, and MUST NOT name a calendar period.

#### Scenario: An initiative is declared per quarter

- GIVEN a container named for a release window
- WHEN it is reviewed
- THEN it is refused, because a calendar is not an end state, and a one-member container is its member wearing a container

Verify: reviewer confirms each container states an end state no single member delivers

### `review:an-unpursued-container-is-retired` — An unpursued container is retired

Where nobody is pursuing an epic or an initiative, the author MUST retire it.

#### Scenario: An epic sits untouched for two quarters

- GIVEN a container with no active members
- WHEN it is reviewed
- THEN it is retired, because no verb reports a staleness verdict and the question is asked here instead

Verify: reviewer confirms every container in the record is still being pursued

### `review:a-drift-report-is-read-and-acted-on` — A drift report is read and acted on

When a drain reports drift, the reader MUST act on each reported item.

#### Scenario: A landing reports two drift lines

- GIVEN fragments landed at the bottom of their lanes
- WHEN the report is read
- THEN each item is acted on, because a landed record whose ranking nobody settled is a plan nobody agreed to

Verify: reviewer confirms each drift line in the landing report was acted on

### `review:a-moved-host-document-carries-its-references` — A moved host document carries its references

When a host document moves, the author MUST update every inbound and outbound reference naming it.

#### Scenario: A specification is split into two

- GIVEN references naming the original path
- WHEN the split lands
- THEN each reference names the document that now carries the claim, because a reference that governs nothing is worse than none

Verify: reviewer confirms every reference to a moved document names its new home

### `review:a-declared-delta-matches-the-diff` — A declared delta matches the diff

Where an entry declares a typed rule delta, the reviewer MUST confirm that the declared type matches the diff.

#### Scenario: A story rewords a rule and declares nothing

- GIVEN a diff that modifies a requirement
- WHEN the entry carries no delta clause
- THEN review rejects it, because the program asserts nothing about which host documents keep rule ids

Verify: reviewer compares each declared delta against the diff, and the diff against the declarations

## Unenforced rules

Every requirement in this domain is unenforced by design. Each one is here because a command cannot decide it. Its scenario states the shape of that judgment, so no second table is needed.
