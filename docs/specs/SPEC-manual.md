# Offline Manual Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`manual:the-product-ships-an-offline-manual` — The product ships an offline manual](#manualthe-product-ships-an-offline-manual--the-product-ships-an-offline-manual)
  - [`manual:an-installing-channel-installs-the-manual` — An installing channel installs the manual](#manualan-installing-channel-installs-the-manual--an-installing-channel-installs-the-manual)
  - [`manual:the-manual-is-derived-from-the-build` — The manual is derived from the build](#manualthe-manual-is-derived-from-the-build--the-manual-is-derived-from-the-build)
  - [`manual:the-manual-is-reachable-from-the-command` — The manual is reachable from the command](#manualthe-manual-is-reachable-from-the-command--the-manual-is-reachable-from-the-command)
  - [`manual:narrative-prose-lives-with-its-definition` — Narrative prose lives with its command definition](#manualnarrative-prose-lives-with-its-definition--narrative-prose-lives-with-its-command-definition)
  - [`manual:the-manual-verb-serves-one-page` — The manual verb serves one page](#manualthe-manual-verb-serves-one-page--the-manual-verb-serves-one-page)
  - [`manual:the-three-verb-lists-are-equal` — The three verb lists are equal](#manualthe-three-verb-lists-are-equal--the-three-verb-lists-are-equal)
- [Unenforced rules](#unenforced-rules)

<!--TOC-->

## Purpose

The offline manual the product ships, where it comes from, and the verb that serves it. The boundary runs at the artifact: this domain says what the manual is and what writes it, while the command surface domain says what every verb obeys. The manual is a rendering of the command definitions and is authority over nothing. What a verb does belongs to that verb's own domain, and how a rendering verb behaves in a pipe belongs to the rendering domain.

## Requirements

### `manual:the-product-ships-an-offline-manual` — The product ships an offline manual

The product MUST ship an offline manual: one page for the command and one page per verb, in the host platform's conventional manual format.

#### Scenario: A reader works without a network

- GIVEN an operator on a machine with no network
- WHEN they ask what an argument does
- THEN the manual answers from disk, because a documentation site is not a manual

Verify: `cargo nextest run --test manual`

### `manual:an-installing-channel-installs-the-manual` — An installing channel installs the manual

Where a distribution channel installs files beside the binary, the channel MUST install the manual at the host platform's conventional location.

#### Scenario: A channel installs a binary and nothing else

- GIVEN a channel whose whole contract is placing one executable on the path
- WHEN it installs the product
- THEN it carries no manual, and the verb below is the whole answer for that channel

Verify: `cargo nextest run --test journey`

### `manual:the-manual-is-derived-from-the-build` — The manual is derived from the build

The implementation MUST derive the manual from the command definitions the build carries, and MUST NOT author a manual page by hand.

#### Scenario: A verb is renamed in the parser alone

- GIVEN a verb whose name changes in the build
- WHEN the manual is produced
- THEN the page follows the name, because the manual states this build rather than a second command surface

Verify: `cargo nextest run --test manual`

### `manual:the-manual-is-reachable-from-the-command` — The manual is reachable from the command

The command MUST serve every page of its own manual on standard output, so that a reader without a manual viewer reads it.

#### Scenario: An agent works in a container with no manual viewer

- GIVEN an agent whose only tool is this command
- WHEN it needs the worked examples and the environment variables
- THEN the verb serves them, because the usage output carries neither

Verify: `cargo nextest run --test verb_contracts`

### `manual:narrative-prose-lives-with-its-definition` — Narrative prose lives with its command definition

Where the manual carries narrative prose beyond the derived body, the implementation MUST keep that prose with the command definition it belongs to.

#### Scenario: A worked example is written in a parallel document

- GIVEN an example the manual is meant to carry
- WHEN it is authored in a file of its own
- THEN the example and the verb drift apart, so it lives at the definition

Verify: reviewer confirms every narrative passage the manual carries sits with the command definition it documents

### `manual:the-manual-verb-serves-one-page` — The manual verb serves one page

The manual verb MUST write one rendered page to standard output alone, and MUST exit 2 naming what it searched on an unknown verb.

#### Scenario: The verb is called bare

- GIVEN an invocation naming no verb
- WHEN it runs
- THEN it lists the pages the manual serves, as the command surface requires of a bare invocation

The verb renders, so the rendering domain binds it. Standard output carries no escape sequence when it is not a terminal, and the meaning stays legible with escapes stripped. The verb emits the rendered page rather than the source format. A reader who pipes it is served by either form, and an agent that reads it in process is served only by the rendered page.

Verify: `cargo nextest run --test verb_contracts`

### `manual:the-three-verb-lists-are-equal` — The three verb lists are equal

The implementation MUST keep three sets equal: the verbs this specification states, the verbs the build derives, and the pages the manual serves.

#### Scenario: A built verb is missing from the manual

- GIVEN a verb the build ships and the manual skips
- WHEN a reader asks for its page
- THEN the absence reads as a verb with no page rather than as a defect, which one assertion over three sets catches

Verify: `cargo nextest run --test manual`

## Unenforced rules

| Rule                                               | Why no command decides it                                                                                                 |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `manual:narrative-prose-lives-with-its-definition` | Whether a passage is the derived body or narrative prose the author added is a reading of the text and not a match on it. |

The requirements above name no tool. Which library derives the manual is an implementation choice, and it is recorded in the code and its decision records. No command holds that line here, because the agnosticism case denies documentation-method product names alone. A reviewer holds it.
