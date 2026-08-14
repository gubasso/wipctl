# Shipped schemas

Six JSON Schemas, all draft 2020-12, all setting `additionalProperties: false` at every level. An
implementation MUST ship them under its data directory and MUST validate its own record and its
own machine output against them; they MUST NOT be copied into a zone.

Two audiences:

- Instance schemas describe what a project writes. `lane.schema.json` gates each of the five lane
  files; `pending.schema.json` gates each fragment; `config.schema.json` gates `.wipctl.toml`.
  Any draft 2020-12 instance checker able to read YAML and TOML documents serves; a TOML checker
  MUST present a native date value as its ISO string for `format: date`.
- Output schemas describe what verbs emit, never what projects write. `epic.schema.json` gates
  `wipctl epic <id> --json`; `epics.schema.json` gates `wipctl epics --json`;
  `epic-list.schema.json` gates `wipctl epic --json` with no id. One schema per shape, because a
  verb offering a machine format MUST supply a schema plus a case validating real output against
  it.

The transition journal has no schema, deliberately: validating a three-column TSV with a schema
toolchain would require the very parsing the format avoids. Its rules live in
[../validation/checks.md](../validation/checks.md).

The schema half owns single-file shape — field names, types, enums, bounds, conditionals. The
cross-file checker owns everything spanning two files. The split is stated in full in
[../validation/ownership-split.md](../validation/ownership-split.md).
