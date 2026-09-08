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
  - [`external-sources:a-source-ref-never-sequences-and-never-counts` — A source reference never sequences and never counts](#external-sourcesa-source-ref-never-sequences-and-never-counts--a-source-reference-never-sequences-and-never-counts)
  - [`external-sources:a-declared-source-carries-a-template-and-a-command` — A declared source carries a template and a command](#external-sourcesa-declared-source-carries-a-template-and-a-command--a-declared-source-carries-a-template-and-a-command)
  - [`external-sources:an-alias-is-unique-and-slugged` — An alias is unique and slugged](#external-sourcesan-alias-is-unique-and-slugged--an-alias-is-unique-and-slugged)
  - [`external-sources:the-declaration-file-is-earned` — The declaration file is earned](#external-sourcesthe-declaration-file-is-earned--the-declaration-file-is-earned)
  - [`external-sources:the-view-runs-each-declared-command` — The view runs each declared command](#external-sourcesthe-view-runs-each-declared-command--the-view-runs-each-declared-command)
  - [`external-sources:the-view-marks-an-unreferenced-item` — The view marks an unreferenced item](#external-sourcesthe-view-marks-an-unreferenced-item--the-view-marks-an-unreferenced-item)
  - [`external-sources:a-missing-source-command-degrades-loudly` — A missing source command degrades loudly](#external-sourcesa-missing-source-command-degrades-loudly--a-missing-source-command-degrades-loudly)
  - [`external-sources:the-view-cache-is-authoritative-for-nothing` — The view cache is authoritative for nothing](#external-sourcesthe-view-cache-is-authoritative-for-nothing--the-view-cache-is-authoritative-for-nothing)
- [Diagnostics](#diagnostics)

<!--TOC-->

## Purpose

A project plans in one record and often carries pending work in other systems at the same time. This domain lets a story, an epic, or an initiative name the outside items it answers. It also lets the plan repository declare where those items live. The boundary runs at the reference. This domain owns the declaration file, the reference grammar, the checks over both, and the view that reports a declared source's open items. The documents domain owns the heading sequence that carries the reference. The zone layout domain owns where the declaration file sits and where a derived view is cached. The rendering domain owns the table, and the messages domain owns the wording of a diagnostic.

The declaration file is stated by a person, never derived from anything, so the rule against storing derived state in the zone is satisfied. The remote system keeps its own items, and this record keeps a key.

## The declared source

`sources.toml` lives at the zone root, inside the plan repository. It names each system the project's artifacts reference, one table per alias. The host repository gains nothing: its footprint stays the one identity file.

```toml
# sources.toml, at the plan zone root

[sources.issues]
url_template = "https://tracker.example/issues/{key}"
list_command = ["<the command that lists open items>", "--format", "json"]

[sources.tickets]
url_template = "https://tickets.example/browse/{key}"
list_command = ["<the command that lists open items>"]
```

`sources.schema.json` owns the file's shape. The cross-file checker owns every fact spanning the file and a document.

## The reference

A source reference is one token, `<alias>#<key>`. The alias names a table in `sources.toml`. The key is the item's own name in that system, and this record never parses it.

```markdown
## Sources

- `issues#412` — the report this story answers
- `tickets#PAY-88` — the same work, tracked by the platform team
```

The section carries paths for no reason and resolves against no root, so it is not a path reference. Its item shape follows the same rule the other reference sections carry: the leading inline-code token is the datum, and everything after it is prose.

## Requirements

### `external-sources:the-remote-system-stays-the-source-of-truth` — The remote system stays the source of truth

The record MUST store the referenced item's key alone, and MUST NOT store any other field the remote system owns.

#### Scenario: A ticket is renamed in its own system

- GIVEN a record holding a copy of the ticket's title
- WHEN the ticket is renamed where it lives
- THEN the copy is wrong and nothing here can tell, so the record keeps the key and a reader follows it

Verify: `cargo nextest run --test schemas`

### `external-sources:a-source-ref-names-a-declared-alias` — A source reference names a declared alias

Every alias appearing in a source reference MUST name a table that `sources.toml` declares.

#### Scenario: An alias is dropped from the declaration file

- GIVEN a document referencing an alias the file no longer declares
- WHEN validation runs
- THEN it fails naming the alias and the file, because a reference nobody can resolve points at nothing

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

### `external-sources:a-source-ref-never-sequences-and-never-counts` — A source reference never sequences and never counts

A source reference MUST NOT appear in `needs`, enter ranking, or contribute to any derived number.

#### Scenario: An entry waits on an outside item

- GIVEN work that cannot start until another team closes a ticket
- WHEN the wait is recorded
- THEN it is recorded as prose in the entry's note, because `needs` is the only sequencing fact and it names entries of this record

Verify: `cargo nextest run --test validation`

### `external-sources:a-declared-source-carries-a-template-and-a-command` — A declared source carries a template and a command

Each declared source MUST carry a url template holding the key placeholder exactly once, and a list command as a non-empty argument vector.

#### Scenario: A source is declared with a url and no command

- GIVEN a table carrying only the template
- WHEN the schema check runs
- THEN it fails naming the missing key, because no value is guessed on a project's behalf

Verify: `cargo nextest run --test schemas`

### `external-sources:an-alias-is-unique-and-slugged` — An alias is unique and slugged

An alias MUST match the slug grammar and MUST be unique within `sources.toml`.

#### Scenario: Two projects choose the same alias

- GIVEN two plan repositories that both declare `issues`
- WHEN each record is read
- THEN both are legal, because an alias is local to the plan that declares it and names nothing outside that file

Verify: `cargo nextest run --test schemas`

### `external-sources:the-declaration-file-is-earned` — The declaration file is earned

Where the project declares no source, `sources.toml` MUST be absent, and its absence MUST fail no check.

#### Scenario: A project references no outside system

- GIVEN a scaffolded plan repository
- WHEN validation runs
- THEN it passes with no declaration file, because an empty file is a statement nobody made

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

## Diagnostics

Each fault names its own resolution.

- A source reference naming an undeclared alias. This is a failure, exit 1. The message names the alias, the document, and `sources.toml`, and offers the two resolutions: declare the alias, or correct the reference.
- A malformed reference token. This is a failure, exit 1. The message shows the token and the `<alias>#<key>` shape it must carry.
- A declaration file that no reference reaches. This is a warning. It reaches the reader and never the exit code, because a source declared before its first reference is a legal first draft.
- A list command that is absent, exits non-zero, or emits unreadable output. This is a degradation, not a failure. One line on the error stream names the alias, the command, and the reader's next step. The view still exits on what the other sources answered.
- A view run where the declaration file is absent. This is a usage error, exit 2. The message names the file and says that the project declares no source yet.
