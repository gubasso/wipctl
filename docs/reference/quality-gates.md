# Quality gates

What any implementation MUST gate, in its own toolchain. Every shipped artifact — executable, schema, payload member, record — carries three obligations in the same change: a hook in the project's commit gate, a case in the test suite, and its toolchain present in the project's pinned development environment. An artifact asserted in prose and not gated reads as verified and is not.

## The gates

- Dogfooding: the implementation project MUST plan with the method it ships. Its own plan record lives where the method puts every record — the project's plan repository, not the implementation's working tree — so what the gate asserts is that the record exists, is attached, and passes the project's own `validate` — as a hook, not a habit. What it can no longer assert is that a clone of the implementation repository carries the record; a clone carries the identity, and the record is one `attach` away.
- The worked example: the shipped example project MUST pass `validate`, and the example's lane files, fragments, and config MUST pass the schemas. The example MUST carry one permanently pending fragment so the pending gates demonstrably select something.
- Schema integrity: every shipped schema MUST validate against the draft 2020-12 metaschema — all ten, the output schemas included.
- Payload agnosticism: the shipped payload MUST NOT name a documentation method or the tooling that installs one, and a case MUST hold that denial over the normative reference and the sources built from it. Generic convention many methods share stays legal; product names do not (ADR-0044).
- The verb contracts: a case per contract page under `reference/cli/verbs/`, covering output shapes, exit codes, stream discipline, and refusal behaviour. Machine formats MUST be validated against their schemas with real output. Every refusal path's case MUST additionally assert that its message names the resolution — the guidance obligation of [cli/conventions.md](./cli/conventions.md), gated rather than asserted.
- The writer guarantees: determinism, identity on legal input (byte-for-byte, final newline), idempotence, non-canonicality, refusal on broken content, one commit per transaction, and lock behaviour (two concurrent writers both survive and commit; two concurrent `start` invocations take two different entries; a bounded wait fails naming the holder; a dead holder releases).
- The journey: install into a temporary prefix, then drive the full two-repository adoption sequence against that installation — `init` a host project, confirm the identity file and the plan repository both exist, capture, drain, first `start`, close, and a second machine's half: `attach` the plan repository by URL and resolve the same record — asserting afterwards that the host checkout gained exactly one file and the plan repository is as the contracts state. This is still the one test that catches an implementation that works from its source tree and not from an installation.
- Prove-it-fails-once: every gate MUST be shown to fail on a deliberate defect before it is trusted. A hook that never selected a file is indistinguishable from a passing one.
- Stage coverage: the record gates MUST run at both the commit and push stages of the plan repository, so a `--no-verify` that skips commit hooks still meets the gate before publication.

## Stream and pipe assertions

Rendering verbs MUST be gated on pipe safety by running: no escape sequences when stdout is not a terminal, colour disabled under the conventional environment variable, and the asserted content is what survives escape stripping.

## Conformance

An implementation conforms when: it accepts and produces the record formats in [reference/record/](./record/zone-layout.md); its verbs honour the contracts in [reference/cli/](./cli/conventions.md), including exit codes, stream discipline, and output shapes; its validation reports every check in [checks.md](./validation/checks.md) in the stated classes; its machine formats validate against the schemas in [schemas/](./schemas/README.md); and it passes every gate on this page, including validating its own plan record with its own linter.

## The self-containment rule

Everything needed to understand, build, or gate the implementation MUST live in its repository. An outbound link is a citation, never a prerequisite; borrowed substance MUST be restated locally. The test: a clone on a machine with nothing else still explains itself, builds, and gates. The project's own plan record is deliberately outside this rule's scope: it is coordination state, not substance — nothing in it is needed to understand, build, or gate the implementation, and the dogfooding gate above is what binds it.
