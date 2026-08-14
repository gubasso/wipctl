# wipctl help

Usage text on stdout, exit 0. Also reached by bare `wipctl` and by `-h`/`--help`.

## Contract

- Opens with one prose paragraph saying what the command is.
- A `commands in this build:` section MUST list every shipped verb, one per line, sorted, derived from the build's own dispatch units — never a hard-coded list.
- When the build lacks verbs the method defines (a partial implementation mid-build), a `not in this build:` section MUST list them under their own heading, with prose stating that each is planned work in the project's own record, not a missing feature. A complete build MUST omit the section.
- Closes with the exit-code table (`0` did what was asked, `1` a check did not hold, `2` the invocation was wrong, `3` the zone lock timed out) and the documentation location.

`help` is the authority on what a given build ships. Shell completion parses its command block, so the block's shape — one indented verb per line — is part of the contract.

## Example

```text
$ wipctl help
wipctl — a plan record you can validate, for a project whose docs you own.

commands in this build:
  aging
  board
  dashboard
  delete
  doctor
  epic
  epics
  fix
  flow
  graph
  help
  ids
  init
  land
  move
  new
  next
  validate
  velocity
  version

exit codes:
  0  the command did what was asked
  1  a check the command performs did not hold
  2  the invocation itself was wrong
  3  the zone lock could not be acquired

documentation: /usr/local/share/wipctl/docs
```
