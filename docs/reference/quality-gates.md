# Quality gates

What any implementation MUST gate, in its own toolchain. Every shipped artifact — executable,
schema, payload member, record — carries three obligations in the same change: a hook in the
project's commit gate, a case in the test suite, and its toolchain present in the project's
pinned development environment. An artifact asserted in prose and not gated reads as verified and
is not.

## The gates

- Dogfooding: the implementation project MUST plan with the method it ships, and its own plan
  record MUST pass its own `validate` — as a hook, not a habit.
- The worked example: the shipped example project MUST pass `validate`, and the example's lane
  files, fragments, and config MUST pass the schemas. The example MUST carry one permanently
  pending fragment so the pending gates demonstrably select something.
- Schema integrity: every shipped schema MUST validate against the draft 2020-12 metaschema.
- The verb contracts: a case per contract page under `reference/cli/verbs/`, covering output
  shapes, exit codes, stream discipline, and refusal behaviour. Machine formats MUST be validated
  against their schemas with real output.
- The writer guarantees: determinism, identity on legal input (byte-for-byte, final newline),
  idempotence, non-canonicality, refusal on broken content, and lock behaviour (two concurrent
  writers both survive; a bounded wait fails naming the holder; a dead holder releases).
- The journey: install into a temporary prefix, drive the full adoption sequence against that
  installation — scaffold, first story, gates, first move, capture, drain — and assert the
  checkout is unchanged afterwards. This is the one test that catches an implementation that
  works from its source tree and not from an installation.
- Prove-it-fails-once: every gate MUST be shown to fail on a deliberate defect before it is
  trusted. A hook that never selected a file is indistinguishable from a passing one.
- Stage coverage: the record gates MUST run at both the commit and push stages, so a companion
  tool or a `--no-verify` that skips commit hooks still meets the gate before publication.

## Stream and pipe assertions

Rendering verbs MUST be gated on pipe safety by running: no escape sequences when stdout is not a
terminal, colour disabled under the conventional environment variable, and the asserted content
is what survives escape stripping.

## Conformance

An implementation conforms when: it accepts and produces the record formats in
[reference/record/](./record/zone-layout.md); its verbs honour the contracts in
[reference/cli/](./cli/conventions.md), including exit codes, stream discipline, and output
shapes; its validation reports every check in [checks.md](./validation/checks.md) in the stated
classes; its machine formats validate against the schemas in [schemas/](./schemas/README.md); and
it passes every gate on this page, including validating its own plan record with its own linter.

## The self-containment rule

Everything needed to understand, build, or gate the implementation MUST live in its repository.
An outbound link is a citation, never a prerequisite; borrowed substance MUST be restated
locally. The test: a clone on a machine with nothing else still explains itself, builds, and
gates.
