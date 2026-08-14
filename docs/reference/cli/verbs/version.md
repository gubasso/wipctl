# wipctl version

One bare version string on stdout, exit 0. Also reached by `-V`/`--version`.

## Contract

- Takes no arguments; any argument is exit 2 with `version takes no arguments`.
- The verb MUST print exactly the version string — no banner, no name, no newline decoration
  beyond the line itself — so a script captures it without parsing.
- The version MUST have one source of truth in the installed artifact (a single version file or
  embedded constant, the implementation's choice); when that source is missing the install is
  broken and the verb MUST exit 1.

## Example

```text
$ wipctl version
1.0.0
```
