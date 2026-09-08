# Capture Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`capture:capture-writes-exactly-two-files` — Capture writes exactly two files](#capturecapture-writes-exactly-two-files--capture-writes-exactly-two-files)
  - [`capture:the-document-passes-its-gate-as-written` — The document passes its gate as written](#capturethe-document-passes-its-gate-as-written--the-document-passes-its-gate-as-written)
  - [`capture:an-absent-field-is-written-absent` — An absent field is written absent](#capturean-absent-field-is-written-absent--an-absent-field-is-written-absent)
  - [`capture:the-claimed-lane-is-a-planning-lane` — The claimed lane is a planning lane](#capturethe-claimed-lane-is-a-planning-lane--the-claimed-lane-is-a-planning-lane)
  - [`capture:an-explicit-id-agrees-with-the-title` — An explicit id agrees with the title](#capturean-explicit-id-agrees-with-the-title--an-explicit-id-agrees-with-the-title)
  - [`capture:a-named-epic-must-exist` — A named epic must exist](#capturea-named-epic-must-exist--a-named-epic-must-exist)
  - [`capture:a-destination-is-checked-before-the-first-byte` — A destination is checked before the first byte](#capturea-destination-is-checked-before-the-first-byte--a-destination-is-checked-before-the-first-byte)
  - [`capture:capture-infers-nothing-from-the-title` — Capture infers nothing from the title](#capturecapture-infers-nothing-from-the-title--capture-infers-nothing-from-the-title)

<!--TOC-->

## Purpose

Capture mints an identity, writes the story document and the pending fragment, touches no lane file, and commits. The boundary runs at landing: this domain owns what capture writes and refuses, while the drain domain owns what happens to the fragment afterwards.

## Requirements

### `capture:capture-writes-exactly-two-files` — Capture writes exactly two files

Capture MUST write the story document and the pending fragment, MUST touch no lane file, and MUST print exactly their two absolute paths.

#### Scenario: An operator wants to open what was just written

- GIVEN a successful capture
- WHEN the paths print
- THEN both are absolute, because the record no longer lives under the working directory

Verify: `cargo nextest run --test verb_contracts`

### `capture:the-document-passes-its-gate-as-written` — The document passes its gate as written

The story document capture writes MUST come from the shipped template and MUST pass the heading-shape gate unedited.

#### Scenario: The capture's own commit runs the hooks

- GIVEN a document written from the template
- WHEN the transaction commits
- THEN the gate passes, because a capture that cannot commit its own output is a capture nobody can use

Verify: `cargo nextest run --test verb_contracts`

### `capture:an-absent-field-is-written-absent` — An absent field is written absent

Capture MUST write an unsupplied point value and the summary absent, and MUST NOT derive either from the title.

#### Scenario: A capture is committed before the work is understood

- GIVEN a fragment with neither field
- WHEN it commits
- THEN the fragment schema accepts it and the drain refuses it later. No summary derived from a title satisfies the lane contract

Verify: `cargo nextest run --test verb_contracts`

### `capture:the-claimed-lane-is-a-planning-lane` — The claimed lane is a planning lane

Capture MUST accept only the two planning lanes and MUST default to the backlog.

#### Scenario: A capture asks to start immediately

- GIVEN a claimed work lane
- WHEN the invocation parses
- THEN it is refused, because landing into a work lane is a transition, and a transition owes a journal event

Verify: `cargo nextest run --test verb_contracts`

### `capture:an-explicit-id-agrees-with-the-title` — An explicit id agrees with the title

Where an explicit id is given, it MUST agree with the title under the identity invariant, and any other id MUST be a usage error.

#### Scenario: An unrelated id is passed

- GIVEN an id that is not the title's slug or a postfix of it
- WHEN the invocation parses
- THEN it is a usage error, because the invocation itself is wrong, while every check against the record is a failed check instead

Verify: `cargo nextest run --test verb_contracts`

### `capture:a-named-epic-must-exist` — A named epic must exist

Where capture names an epic, the implementation MUST refuse before any write when no such document exists.

#### Scenario: An epic is named before it is written

- GIVEN a capture claiming an epic with no document
- WHEN the preflight runs
- THEN it refuses, because landing it later writes an entry the epic rules refuse

Verify: `cargo nextest run --test verb_contracts`

### `capture:a-destination-is-checked-before-the-first-byte` — A destination is checked before the first byte

Capture MUST check every destination before writing, MUST leave an existing path alone, and MUST offer no overwrite.

#### Scenario: A capture repeats an id that already has a document

- GIVEN a story document already on disk
- WHEN capture runs
- THEN it reports and leaves it, because a capture that overwrites is a capture that loses work

Verify: `cargo nextest run --test verb_contracts`

### `capture:capture-infers-nothing-from-the-title` — Capture infers nothing from the title

Capture MUST NOT infer the type, the epic, or the point value from the title, and MUST NOT edit an existing file.

#### Scenario: A title suggests a type

- GIVEN a title that reads like a chore
- WHEN capture runs without a type
- THEN it is a usage error, because a guessed classification is a value nobody wrote down

Verify: `cargo nextest run --test verb_contracts`
