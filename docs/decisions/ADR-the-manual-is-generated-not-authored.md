# The manual is generated and not authored

## Context and Problem Statement

The specification states one contract per verb, and the build states one parser. A shipped manual is a third statement of the same command surface. Three statements of one surface drift in the order a reader is least likely to notice, because the manual is the copy nobody runs.

## Considered Options

- The manual is derived from the command definitions the build carries.
- The manual is authored by hand, one page per verb.
- The product ships no manual, and the usage output is the whole answer.

## Decision Outcome

Chosen option: `derived from the command definitions` — derivation makes the count two and gives each statement a distinct job. The specification states what any implementation must do. The command definitions state what this build does. The manual renders the second and is authority over nothing, so it adds no source of truth.

Hand authoring was rejected as the third surface itself. Shipping no manual was rejected because the usage output carries no worked example, no environment variable, and no cross-reference, and an operator offline has nowhere else to read them.

This project already accepts the same argument for shell completion, which derives its candidates at completion time rather than from a constant.

Enforced by `manual:the-manual-is-derived-from-the-build`.

## Consequences

- Good: a verb the build lacks cannot appear in the manual, by construction.
- Good: one assertion over three sets catches a specified verb nobody built and a built verb the manual skipped.
- Bad: a generated artifact never acquires an author, and the first hand edit to a page gives the project two command surfaces with nothing to say which is real.
- Bad: the manual inherits whatever the parser expresses, so a contract the parser cannot carry has no place to live in the manual.

## Status

Accepted
