# Doctor Specification

## Purpose

The environment report: what the build needs, what it can use, what will degrade, and whether this project's plan machinery is sound. The boundary runs at aggregation: this domain owns the report and the dependency manifest, while the command surface domain owns what an optional dependency owes at any other verb's entry.

## The dependency manifest

The required set is git, plus the language runtime the implementation needs, or none. The baseline optional set holds one entry: a draft 2020-12 instance checker, probed by validation, presence-only, whose declared fallback is the named skip.

Each list command a project declares under an external source is an optional dependency too. It is discovered from that project's own declaration rather than baked into the manifest. It is probed by the external sources view, presence-only, and its declared fallback is the named degradation.

## Requirements

### `doctor:a-manifest-item-is-one-line` — A manifest item is one line

The report MUST print one prefixed line per manifest item, naming the tool, the version found, and whether it is required or optional.

#### Scenario: An optional tool is present

- GIVEN a manifest entry for an optional checker
- WHEN the report runs
- THEN the line names the view it serves and the fallback, because a reader deciding whether to install it needs both

Verify: `cargo nextest run --test verb_contracts`

### `doctor:a-missing-item-names-its-remediation` — A missing item names its remediation

A missing-item line MUST be followed by an indented remediation line naming what to install or configure.

#### Scenario: A colour capability is absent

- GIVEN a terminal without the capability an optional panel wants
- WHEN the report runs
- THEN the remediation names the variable and the condition, because a report that names a gap and no fix is a list of complaints

Verify: `cargo nextest run --test verb_contracts`

### `doctor:only-a-required-absence-fails` — Only a required absence fails

The report MUST exit non-zero when a required item is missing, and MUST succeed when only optional items are absent.

#### Scenario: Every optional tool is missing

- GIVEN a machine with the required set and nothing else
- WHEN the report runs
- THEN it succeeds, because an optional dependency improves output and never gates it

Verify: `cargo nextest run --test verb_contracts`

### `doctor:a-probe-covers-three-classes` — A probe covers three classes

The implementation MUST probe presence against a version floor, the environment, and behavior.

#### Scenario: A tool emits escapes with colour disabled

- GIVEN a behavioural probe feeding known input to a tool
- WHEN the output carries escape sequences anyway
- THEN the tool fails pipe safety and is treated as absent, because a tool that ignores the setting is worse than none

Verify: `cargo nextest run --test verb_contracts`

### `doctor:a-probe-only-reports` — A probe only reports

One shared probe set MUST serve the report and every verb's entry preflight, and only the report MUST aggregate and exit non-zero.

#### Scenario: A rendering verb finds a tool absent

- GIVEN a probe returning absence at a rendering verb's entry
- WHEN the verb renders
- THEN it takes its declared fallback and announces the degradation in one line, rather than failing

Verify: `cargo nextest run --test verb_contracts`

### `doctor:the-report-covers-the-plan-machinery` — The report covers the plan machinery

Inside a project, the report MUST also cover the lock directory, the attachment and identity state, the hook installation, and every blocker.

#### Scenario: A clone has the record and no hooks

- GIVEN a plan repository cloned onto a second machine
- WHEN the report runs
- THEN the missing hook installation is a miss and never a silent state, alongside the open questions and unclosed dependencies holding work

Verify: `cargo nextest run --test verb_contracts`

### `doctor:a-new-optional-tool-lands-whole` — A new optional tool lands whole

When the implementation adds an optional dependency, it MUST add the manifest row, the probe, and the fallback in the same change.

#### Scenario: A chart renderer is adopted

- GIVEN a new optional tool
- WHEN it lands with no manifest row
- THEN its absence degrades something nobody declared, so the three land together

Verify: `cargo nextest run --test verb_contracts`
