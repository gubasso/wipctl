# wipctl doctor

The environment report: what the build needs, what it can use, what will degrade, and whether this project's plan machinery is sound.

## Usage

```text
wipctl doctor
```

## Contract

- One line per manifest item, prefixed `ok`, `warn`, or `miss`, naming the tool, the version found, and `required` or `optional — <view served>; fallback: <fallback>`. A `miss` line MUST be followed by an indented remediation line naming what to install or configure.
- Exits non-zero when a required item is missing; optional absences alone exit 0.
- Three probe classes:
  1. presence with a version floor;
  2. environment — a UTF-8 locale, 24-bit colour capability;
  3. behavioural — feed a tool known input and check the output (a tool that emits escape sequences with colour disabled fails pipe safety and MUST be treated as absent).
- One shared probe set serves `doctor` and every verb's entry preflight: a probe MUST only report; only `doctor` aggregates and exits non-zero; a rendering verb MUST take its declared fallback and announce the degradation in exactly one stderr line.
- Run inside a project, the report MUST additionally cover the plan machinery: which lock directory is in use — `$XDG_RUNTIME_DIR/wipctl` or the stated `/tmp` replacement ([../../record/zone-layout.md](../../record/zone-layout.md)); whether the project is attached and the identity files agree; whether the plan repository's hook set is installed — a missing installation is a `miss`, never a silent state; and every blocker currently holding work (the open questions and unclosed needs the `start` diagnostics point at).

## The dependency manifest

The manifest is part of this reference page and of the shipped documentation: the required set — git, and the language runtime or none, per implementation — and per optional entry its version floor, the view it serves, and its declared fallback. The baseline manifest holds one optional entry: the JSON Schema draft 2020-12 instance checker, probed by `validate`, presence-only, with the named skip `schemas: skipped — no instance checker on PATH` as its fallback. An implementation adding an optional rendering dependency MUST add its manifest row, its probe, and its fallback in the same change.

## Example

```text
$ wipctl doctor
ok    git 2.49                required — the plan repository's substrate
ok    instance checker 0.34   optional — schema half of validate; fallback: named skip
ok    locale en_US.UTF-8      required
miss  24-bit colour           optional — continuous panels; fallback: glyph ramp
      set COLORTERM=truecolor in a terminal that supports it
ok    lock directory          /run/user/1000/wipctl
ok    plan payments-acme      attached; identities agree; hooks installed
```
