# Decisions

Why wipctl is shaped this way. One decision per record, at or below 350 words, exactly one `Status`. A record is frozen once accepted: a replaced decision gets a new superseding record, a partial change gains an `Amended by` line, and the subsystem pages — not the records — describe the present. The ADR sequence is allocated serially by one person; it is the one counter the product permits, and ADR-0026 names it as the excluded case.

The template is [template.md](./template.md). A filled record is `ADR-<number>-<decision>.md` and carries exactly one status.

## By theme

### The record

- [ADR-0001](./ADR-0001-the-unit-of-work-is-a-story-in-one-file.md) — one vertical change per document; tasks never become records of their own
- [ADR-0002](./ADR-0002-the-plan-record-is-five-lane-files.md) — the file is the lane and the position is the rank; no `status` or `priority` fields
- [ADR-0003](./ADR-0003-a-lane-entry-carries-a-capped-summary.md) — every entry restates its story in one bounded line
- [ADR-0004](./ADR-0004-an-epic-is-a-field-and-a-document.md) — membership points entry to document; an epic never schedules anything
- [ADR-0005](./ADR-0005-the-plan-record-stays-in-version-control.md) — plain files versioned with the project; no database, service, or per-user store

### The plan repository

- [ADR-0045](./ADR-0045-the-plan-record-lives-in-its-own-repository.md) — one live record per project, its own git repository at a user-level location; supersedes ADR-0005
- [ADR-0046](./ADR-0046-a-project-is-identified-by-a-minted-slug.md) — `project_id` is a minted slug resolved through the attachment registry; supersedes ADR-0008
- [ADR-0048](./ADR-0048-git-is-the-replication-substrate.md) — one plan trunk, push as backup, semantic reconciliation; supersedes ADR-0036
- [ADR-0049](./ADR-0049-a-plan-change-and-a-code-change-are-no-longer-one-commit.md) — the accepted loss: explicit code references and a same-unit-of-review obligation
- [ADR-0057](./ADR-0057-a-plan-mutation-is-always-committed.md) — one mutation, one commit, in the plan trunk's grammar; supersedes ADR-0022

### Configuration and discovery

- [ADR-0006](./ADR-0006-the-config-is-required-validated-and-has-no-defaults.md) — `.wipctl.toml` is required and validated, and no key has a default
- [ADR-0007](./ADR-0007-the-config-is-toml-at-the-project-root.md) — one hidden TOML file marks the root and declares the project's identity
- [ADR-0008](./ADR-0008-the-plan-directory-is-discovered.md) — reading verbs walk upward to the config; `init` alone never walks
- [ADR-0037](./ADR-0037-no-defaults-is-a-read-time-rule.md) — no defaults binds verbs reading the config; the scaffold generates a complete file the operator owns

### The program

- [ADR-0009](./ADR-0009-the-program-exposes-one-command.md) — one command, `wipctl`, verbs derived from the build, uniform exit codes
- [ADR-0010](./ADR-0010-the-linter-may-write-the-record.md) — the repair writes on explicit request only and never authors priority
- [ADR-0011](./ADR-0011-the-scaffold-writes-only-what-it-creates.md) — the scaffold is create-only; an existing path is reported and left alone
- [ADR-0012](./ADR-0012-the-method-travels-with-the-zone.md) — the scaffold writes one self-sufficient method document into the zone
- [ADR-0013](./ADR-0013-a-machine-format-is-verb-local-and-owes-a-schema.md) — `--json` is verb-local and every machine shape owes a schema
- [ADR-0014](./ADR-0014-a-derived-plan-may-be-cached-and-is-never-authoritative.md) — a cached epic plan is rewritten always and read back never
- [ADR-0015](./ADR-0015-one-verb-resolves-one-epic-and-another-summarises-them-all.md) — `epics` rolls up, `epic <id>` resolves; a new question earns a new verb
- [ADR-0016](./ADR-0016-an-optional-dependency-never-gates-output.md) — an optional dependency improves output and never gates it
- [ADR-0050](./ADR-0050-every-message-teaches.md) — a diagnostic that names a failure without naming its resolution is unfinished

### Rendering

- [ADR-0017](./ADR-0017-rendering-targets-a-terminal.md) — terminal text is the only target; no HTML, image, or TUI writer
- [ADR-0018](./ADR-0018-colour-carries-data-only-in-a-continuous-panel.md) — the glyph holds the value; colour carries data only in a continuous panel

### Transitions

- [ADR-0019](./ADR-0019-a-lane-transition-is-a-recorded-event.md) — a lane change is an event the record states, never derived from repository history
- [ADR-0020](./ADR-0020-the-transition-journal-lives-in-the-plan-zone.md) — the journal lives in the zone and travels with the lane files
- [ADR-0021](./ADR-0021-a-transition-journal-is-one-stream-per-entry.md) — one append-only TSV stream per entry; the last line is the current lane
- [ADR-0022](./ADR-0022-a-transition-commit-is-generated-and-opt-in.md) — the transition commit is opt-in, fixed-shape, and never bypasses hooks
- [ADR-0023](./ADR-0023-closing-is-not-terminal.md) — `closed` is a lane, not a terminus; `move` performs the reopening
- [ADR-0024](./ADR-0024-a-deletion-is-recorded-and-burns-the-id.md) — `delete` leaves a tombstone journal and burns the id
- [ADR-0025](./ADR-0025-two-writers-cannot-lose-a-transition.md) — writers take an exclusive zone lock; readers never do

### Identity

- [ADR-0026](./ADR-0026-a-counter-is-forbidden-where-allocation-is-concurrent.md) — no counter allocates a record id; the gate reports collisions between claimants in one record
- [ADR-0027](./ADR-0027-an-entry-is-named-by-its-filename-stem.md) — an id lives as the filename stem of everything it names
- [ADR-0051](./ADR-0051-the-slug-is-the-id.md) — the id is the title's slug, checked under the lock, disambiguated by a chosen postfix; nothing random is minted; supersedes ADR-0030
- [ADR-0052](./ADR-0052-a-title-and-its-id-always-match.md) — a title and its id always match, and `rename` is the operation that changes either while holding them in agreement
- [ADR-0058](./ADR-0058-a-qualifying-postfix-is-part-of-the-id.md) — a qualifying postfix is part of the id it disambiguates, and burning follows whether the id was freed; amends ADR-0024, ADR-0051, and ADR-0052

### Concurrent capture

- [ADR-0028](./ADR-0028-a-concurrent-capture-is-a-lane-delta.md) — a capture is a story plus a pending fragment stating a lane delta
- [ADR-0029](./ADR-0029-the-drain-orders-by-the-stated-instant.md) — the drain orders by the stated `captured` instant, ties broken on the full id
- [ADR-0030](./ADR-0030-the-program-mints-the-identity.md) — `wipctl new` mints the id and writes both files itself
- [ADR-0031](./ADR-0031-a-position-names-only-a-landed-entry.md) — `after` names a landed entry only; a fragment target is drift
- [ADR-0032](./ADR-0032-landing-is-a-creation-not-a-transition.md) — landing writes no journal event; a journal begins at the first `move`
- [ADR-0038](./ADR-0038-capture-disjointness-holds-outside-collision.md) — the no-conflict merge holds outside an id collision; recovery is rephrasing and renaming one side

### Concurrency

- [ADR-0047](./ADR-0047-the-writer-lock-is-per-project-per-machine.md) — the transaction lock spans every checkout of a project on a machine; supersedes ADR-0025
- [ADR-0053](./ADR-0053-taking-work-is-a-verb-of-its-own.md) — `start` reads and takes atomically; `next` stays read-only

### The host and the doctrine

- [ADR-0033](./ADR-0033-the-host-stays-unassumed.md) — the product assumes one thing of the host: a documentation directory exists
- [ADR-0034](./ADR-0034-a-story-is-a-diff-a-durable-document-is-a-state.md) — a closing story's truth transfers to durable documents; the story freezes
- [ADR-0035](./ADR-0035-an-amends-path-may-be-promised.md) — `new:` promises a document and exempts its path from the existence check
- [ADR-0041](./ADR-0041-an-amends-item-may-carry-a-typed-rule-delta.md) — an `Amends` assertion may type a rule change: `ADDED`, `MODIFIED`, or `REMOVED` plus the rule id
- [ADR-0042](./ADR-0042-a-story-has-no-done-when.md) — a story's closing condition is its `Acceptance`; `Done when` belongs to epics alone
- [ADR-0043](./ADR-0043-the-inbound-heading-is-reads.md) — the inbound reference heading is `Reads`, symmetric with `Amends`
- [ADR-0044](./ADR-0044-the-payload-names-no-documentation-method.md) — the shipped payload names no documentation method, so ADR-0033 fails a test rather than a reading
- [ADR-0036](./ADR-0036-the-program-speaks-git-and-no-other-vcs.md) — the program speaks git in one opt-in place and no other version control ever
- [ADR-0039](./ADR-0039-the-repository-is-self-contained.md) — every load-bearing fact lives in-repo; an outbound link is a citation and never a prerequisite

### Initiatives

- [ADR-0054](./ADR-0054-an-initiative-is-a-document-and-a-pointer.md) — one document plus a pointer from the epic; the ladder stops at three tiers; amends ADR-0004
- [ADR-0055](./ADR-0055-an-epic-declares-its-initiative-in-its-document.md) — an `Initiative` section holding one id or `None`, first in the epic shape
- [ADR-0056](./ADR-0056-an-initiative-is-reported-never-executed.md) — the decomposition is one row per member epic; execution stays the epic verb's answer

### Release and distribution

- [ADR-0040](./ADR-0040-crates-io-trusted-publishing.md) — a release run mints its own registry credential; no publishing secret exists between releases

## Statuses

`Ideation`, `Proposed`, `Accepted`, `Implemented`, `Deprecated`, `Superseded`, `Rejected`. Every record in this set is `Accepted` or `Superseded`; a superseded record keeps its body and names its successor under `Status`.
