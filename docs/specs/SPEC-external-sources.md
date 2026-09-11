# External Sources Specification

<!--TOC-->

- [Purpose](#purpose)
- [The declared source](#the-declared-source)
- [The reference](#the-reference)
- [Requirements](#requirements)
  - [`external-sources:the-remote-system-stays-the-source-of-truth` — The remote system stays the source of truth](#external-sourcesthe-remote-system-stays-the-source-of-truth--the-remote-system-stays-the-source-of-truth)
  - [`external-sources:a-source-ref-names-a-declared-alias` — A source reference names a declared alias](#external-sourcesa-source-ref-names-a-declared-alias--a-source-reference-names-a-declared-alias)
  - [`external-sources:a-source-ref-is-stated-and-never-verified` — A source reference is stated and never verified](#external-sourcesa-source-ref-is-stated-and-never-verified--a-source-reference-is-stated-and-never-verified)
  - [`external-sources:a-source-ref-carries-no-hierarchy` — A source reference carries no hierarchy](#external-sourcesa-source-ref-carries-no-hierarchy--a-source-reference-carries-no-hierarchy)
  - [`external-sources:a-source-ref-never-sequences-and-never-counts` — A Sources reference never sequences and never counts](#external-sourcesa-source-ref-never-sequences-and-never-counts--a-sources-reference-never-sequences-and-never-counts)
  - [`external-sources:a-declared-source-carries-a-template-and-a-command` — A declared source carries a template and a command](#external-sourcesa-declared-source-carries-a-template-and-a-command--a-declared-source-carries-a-template-and-a-command)
  - [`external-sources:an-alias-is-unique-and-slugged` — An alias is unique and slugged](#external-sourcesan-alias-is-unique-and-slugged--an-alias-is-unique-and-slugged)
  - [`external-sources:the-sources-section-is-earned` — The sources section is earned](#external-sourcesthe-sources-section-is-earned--the-sources-section-is-earned)
  - [`external-sources:the-view-runs-each-declared-command` — The view runs each declared command](#external-sourcesthe-view-runs-each-declared-command--the-view-runs-each-declared-command)
  - [`external-sources:the-view-marks-an-unreferenced-item` — The view marks an unreferenced item](#external-sourcesthe-view-marks-an-unreferenced-item--the-view-marks-an-unreferenced-item)
  - [`external-sources:a-missing-source-command-degrades-loudly` — A missing source command degrades loudly](#external-sourcesa-missing-source-command-degrades-loudly--a-missing-source-command-degrades-loudly)
  - [`external-sources:the-view-cache-is-authoritative-for-nothing` — The view cache is authoritative for nothing](#external-sourcesthe-view-cache-is-authoritative-for-nothing--the-view-cache-is-authoritative-for-nothing)
  - [`external-sources:the-report-distinguishes-empty-from-unread` — The report distinguishes empty from unread](#external-sourcesthe-report-distinguishes-empty-from-unread--the-report-distinguishes-empty-from-unread)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

A project plans in one record and often carries pending work in other systems at the same time. This domain lets a story, an epic, or an initiative name the outside items it answers. It also lets the plan repository declare where those items live. The boundary runs at the reference. This domain owns the declaration, the reference grammar, the checks over both, and the view that reports a declared source's open items. The configuration domain owns the file the declaration sits in. The documents domain owns the heading sequence that carries the reference. The zone layout domain owns where a derived view is cached. The rendering domain owns the table, and the messages domain owns the wording of a diagnostic.

The declaration is stated by a person, never derived from anything, so the rule against storing derived state in the zone is satisfied. The remote system keeps its own items, and this record keeps a key.

## The declared source

The `sources` section sits in `.wipctl/plan.toml` at the plan zone root. It names each system the project's artifacts reference, one table per alias. The host repository gains nothing: its footprint stays the one project file.

```toml
# the sources section of .wipctl/plan.toml

[sources.issues]
url_template = "https://tracker.example/issues/{key}"
list_command = ["<the command that lists open items>", "--format", "json"]

[sources.tickets]
url_template = "https://tickets.example/browse/{key}"
list_command = ["<the command that lists open items>"]
```

The `sources` half of `plan.schema.json` owns the section's shape, in the configuration spec's companion directory. The cross-file checker owns every fact spanning the section and a document.

## The reference

A source reference is one token, `<alias>#<key>`. The alias names a table in the `sources` section. The key is the item's own name in that system, and this record never parses it.

```markdown
## Sources

- `issues#412` — the report this story answers
- `tickets#PAY-88` — the same work, tracked by the platform team
```

The section carries paths for no reason and resolves against no root, so it is not a path reference. Its item shape follows the same rule the other reference sections carry: the leading inline-code token is the datum, and everything after it is prose.

The relation is many to many, and neither direction is capped. One artifact names as many items as it answers. One item is named by as many artifacts as answer it, at any tier, so an epic and its member stories can each reference it. Nothing is derived from that overlap, because membership still runs one way through the local ladder.

The reverse question, which artifacts name one item, needs no stored index and no command. One fixed-string search for the token over the plan zone answers it. That search runs offline, and it still finds an item the remote system has closed, which the view cannot.

## Requirements

### `external-sources:the-remote-system-stays-the-source-of-truth` — The remote system stays the source of truth

The record MUST store the referenced item's key alone, and MUST NOT store any other field the remote system owns.

#### Scenario: A ticket is renamed in its own system

- GIVEN a record holding a copy of the ticket's title
- WHEN the ticket is renamed where it lives
- THEN the copy is wrong and nothing here can tell, so the record keeps the key and a reader follows it

Verify: `cargo nextest run --test schemas`

### `external-sources:a-source-ref-names-a-declared-alias` — A source reference names a declared alias

Every alias appearing in a source reference MUST name a table that the `sources` section declares.

#### Scenario: An alias is dropped from the declaration

- GIVEN a document referencing an alias the section no longer declares
- WHEN validation runs
- THEN it fails naming the alias and the section, because a reference nobody can resolve points at nothing

Verify: `cargo nextest run --test validation`

### `external-sources:a-source-ref-is-stated-and-never-verified` — A source reference is stated and never verified

The implementation MUST treat the key as opaque past its shape, and MUST reach no remote system to decide any check.

#### Scenario: A referenced item is closed upstream

- GIVEN a story naming an item that the remote system has since closed
- WHEN validation runs
- THEN it passes, because the record cannot see another system and a check that reaches the network is a check that fails offline

Verify: `cargo nextest run --test validation`

### `external-sources:a-source-ref-carries-no-hierarchy` — A source reference carries no hierarchy

The implementation MUST NOT compare one artifact's source references against another's, and MUST derive no tier, membership, or ordering from the remote system's own tree.

#### Scenario: A story and its epic reference unrelated items

- GIVEN a story whose referenced item is no child of the item its epic references
- WHEN validation runs
- THEN it passes, because membership runs one way through the local ladder and a remote tree read as membership becomes a second store of it

Verify: `cargo nextest run --test validation`

### `external-sources:a-source-ref-never-sequences-and-never-counts` — A Sources reference never sequences and never counts

A source reference in a `Sources` section MUST NOT appear in `needs`, enter ranking, or contribute to any derived number. A wait belongs in a watch section, whose `Blocks` line names the affected entries.

#### Scenario: An entry waits on an outside item

- GIVEN work that cannot start until another team closes a ticket
- WHEN the wait is recorded
- THEN it becomes a watch section because a source reference says what work answers while a watch says what work waits on

Verify: `cargo nextest run --test validation`

### `external-sources:a-declared-source-carries-a-template-and-a-command` — A declared source carries a template and a command

Each declared source MUST carry a url template holding the key placeholder exactly once, and a list command as a non-empty argument vector.

#### Scenario: A source is declared with a url and no command

- GIVEN a table carrying only the template
- WHEN the schema check runs
- THEN it fails naming the missing key, because no value is guessed on a project's behalf

Verify: `cargo nextest run --test schemas`

### `external-sources:an-alias-is-unique-and-slugged` — An alias is unique and slugged

An alias MUST match the slug grammar and MUST be unique within the `sources` section.

#### Scenario: Two projects choose the same alias

- GIVEN two plan repositories that both declare `issues`
- WHEN each record is read
- THEN both are legal, because an alias is local to the plan that declares it and names nothing outside that file

Verify: `cargo nextest run --test schemas`

### `external-sources:the-sources-section-is-earned` — The sources section is earned

Where the project declares no source, the `sources` section MUST be absent, and its absence MUST fail no check.

#### Scenario: A project references no outside system

- GIVEN a scaffolded plan repository
- WHEN validation runs
- THEN it passes with no sources section, because an empty section is a statement nobody made

Verify: `cargo nextest run --test validation`

### `external-sources:the-view-runs-each-declared-command` — The view runs each declared command

The view MUST run the list command each declared alias carries, and MUST leave the plan zone untouched.

#### Scenario: A reader asks what is open across every system

- GIVEN two declared sources
- WHEN the view runs
- THEN both commands run and the record gains nothing, because a view is derived on every run and writes no state anybody later reads

Verify: `cargo nextest run --test verb_contracts`

### `external-sources:the-view-marks-an-unreferenced-item` — The view marks an unreferenced item

The view MUST mark each listed item as referenced by an artifact of this record or as referenced by none.

#### Scenario: A forge issue nobody planned

- GIVEN an open item no document references
- WHEN the view runs
- THEN it is marked unreferenced, because the reason to read several lists at once is to see what fell between them

Verify: `cargo nextest run --test verb_contracts`

The view also compares each reference-form watch against its alias's open list. It marks the watch `still listed` when the key appears and `no longer listed` when the key is absent. The second mark says only that the item left the open list. It does not claim that the item closed, was fixed, or met the watch's `Until` condition.

Each alias object carries its watch judgments beside its `answered` field. An answered source with no missing watch carries an empty result. A source that did not answer marks its watches `unjudged`, because an unread list can support no absence claim.

A phrase-form watch has no alias or key to compare. The report includes it in `skipped_watches` with the `skipped` mark instead of omitting it.

### `external-sources:a-missing-source-command-degrades-loudly` — A missing source command degrades loudly

Where a declared command does not answer, the view MUST name the degradation on the error stream and MUST report every source that did.

#### Scenario: One of two source commands is not installed

- GIVEN a machine carrying one of the two declared commands
- WHEN the view runs
- THEN the installed source reports and the absent one is named, because an optional dependency improves output and never gates it

Verify: `cargo nextest run --test verb_contracts`

### `external-sources:the-view-cache-is-authoritative-for-nothing` — The view cache is authoritative for nothing

Where the report is written to the cache, it MUST be rewritten on every call, read back by no verb, and required by no check.

#### Scenario: The cached report is deleted mid-session

- GIVEN a session holding the path the view printed
- WHEN the file is removed
- THEN every answer is unchanged and nothing fails, because a file nothing believes cannot become a second store

Verify: `cargo nextest run --test verb_contracts`

### `external-sources:the-report-distinguishes-empty-from-unread` — The report distinguishes empty from unread

The machine format MUST carry one object per declared alias, each stating whether that alias's command answered.

#### Scenario: A consumer counts open work across two sources

- GIVEN one source that answered with no items and one whose command was absent
- WHEN the report is parsed
- THEN the two are distinguishable, because a zero a reader cannot trust is worse than a stated gap

Verify: `cargo nextest run --test schemas`

The shape is `sources-report.schema.json`, beside this page. The obligation to ship that schema and a case validating real output against it belongs to `cli-conventions:a-machine-format-is-verb-local`, and is not restated here.

## Diagnostics

Each fault names its own resolution.

- A source reference naming an undeclared alias. This is a failure, exit 1. The message names the alias, the document, and the `sources` section, and offers the two resolutions: declare the alias, or correct the reference.
- A malformed reference token. This is a failure, exit 1. The message shows the token and the `<alias>#<key>` shape it must carry.
- A declared source that no reference reaches. This is a warning. It reaches the reader and never the exit code, because a source declared before its first reference is a legal first draft.
- A list command that is absent, exits non-zero, or emits unreadable output. This is a degradation, not a failure. One line on the error stream names the alias, the command, and the reader's next step. The view still exits on what the other sources answered.
- A view run where the `sources` section is absent. This is a usage error, exit 2. The message names the section and says that the project declares no source yet.
