# Repository Docs Specification

## Purpose

The rules this repository adds to the documentation convention it adopts. The adopted specs carry the convention as it ships, unchanged. This domain holds what this repository asks of its own documents beyond them. The boundary runs at the adopted specs: a rule that every adopter shares belongs upstream, and a rule that only this repository needs lives here.

## Requirements

### `repository-docs:requirement-keyword-stays-in-the-spec-zone` — An uppercase requirement keyword stays in the spec zone

The author MUST keep every uppercase requirement keyword inside a spec under `docs/specs/`, and MUST cite the rule ID from anywhere else that needs it.

#### Scenario: A guide states a rule in requirement language

- GIVEN a guide whose step reads "the operator MUST run the command before the push"
- WHEN a reader looks for what binds them
- THEN the sentence binds while carrying no rule ID, no scenario, and no verification, so it reads as gated while nothing decides it, and the gate rejects the document

Verify: `prek run requirement-keyword-zone --all-files`

### `repository-docs:a-placeholder-carries-no-value` — A placeholder carries no value

The author MUST write a placeholder in lowercase angle form, and MUST NOT give a guide token a real or realistic value.

#### Scenario: A guide shows a command with a token

- GIVEN a guide whose token stands for a value the reader supplies
- WHEN an example fills it in
- THEN the token is no longer a placeholder, so the upper-snake form stays empty and the lowercase angle form travels

Verify: reviewer confirms every placeholder is a form and never a value

## Unenforced

| Rule                                             | Reviewer confirms                                                                                                             |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| `repository-docs:a-placeholder-carries-no-value` | every placeholder is a form and never a value, because whether a token reads as a realistic value is a reading of the example |
