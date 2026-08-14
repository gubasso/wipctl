# wipctl next

What to start, in one parseable line.

## Usage

```text
wipctl next [<plan-dir>]
```

At most one positional (else exit 2 `next takes at most one plan directory`). No flags, and no
`--json` by design: the line is already the machine format.

## Contract

- Prints exactly one line on stdout: the head entry of `doing.yml`, or of `todo.yml` when `doing`
  is empty:

  ```text
  <id>  <title>  <points>pt  <story path relative to the project root>
  ```

  The title is the story document's H1 with its leading `<id> — ` prefix stripped; when the
  document is unreadable the id MUST stand in.
- Both lanes empty: empty stdout, exit 0 — no sentence for a consumer to strip.
- This is a head read, not a search. Ranking rule R1 sorts eligible entries above ineligible
  ones, so whenever `todo` holds a startable entry the head is one; the verb MUST NOT walk the
  lane looking for a startable entry.
- `next` MUST NOT gate. A record defect MUST be emitted on stderr as
  `wipctl: warning: <location>: <message>` and the line printed anyway. A head that is not
  startable — blocked by an open question or an unclosed need, whether from an R1 violation or a
  lane holding no eligible entry at all — MUST be warned about, not fatal:
  `wipctl: warning: rate-limit-the-search-endpoint-a7f3 is blocked by an open question`.
- Pending entries are invisible here: unlanded work MUST NOT be offered as a thing to start.

## Example

```text
$ wipctl next
supervised-child-runtime-b47d  Supervised child runtime  2pt  docs/plan/stories/supervised-child-runtime-b47d.md
```
