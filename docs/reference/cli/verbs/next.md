# wipctl next

What to start, in one parseable line. `next` tells you; `start` takes it — the twin verbs name each other, and only `start` writes anything ([start.md](./start.md)).

## Usage

```text
wipctl next
```

No arguments (else exit 2 `next takes no arguments`). No flags, and no `--json` by design: the line is already the machine format.

## Contract

- Prints exactly one line on stdout: the head entry of `doing.yml`, or of `todo.yml` when `doing` is empty:

  ```text
  <id>  <title>  <points>pt  <absolute story path>
  ```

  The title is the story document's H1 with its leading `<id> —` prefix stripped; when the document is unreadable the id MUST stand in. The path is absolute, because the record does not live under the working directory.
- Both lanes empty: empty stdout, exit 0 — no sentence for a consumer to strip.
- This is a head read, not a search. Ranking rule R1 sorts eligible entries above ineligible ones, so whenever `todo` holds a startable entry the head is one; the verb MUST NOT walk the lane looking for a startable entry.
- `next` MUST NOT gate, MUST NOT take the lock, and MUST NOT block — it is a pure function of the committed files. A record defect MUST be emitted on stderr as `wipctl: warning: <location>: <message>` and the line printed anyway. A head that is not startable — blocked by an open question or an unclosed need, whether from an R1 violation or a lane holding no eligible entry at all — MUST be warned about, not fatal: `wipctl: warning: rate-limit-the-search-endpoint is blocked by an open question`.
- Pending entries are invisible here: unlanded work MUST NOT be offered as a thing to start.
- Under concurrent agents the line is advisory by construction: two agents that both run `next` legitimately see the same head, and the one that acts second learns so at `move` (exit 2) or avoids the race entirely with `start`.

## Example

```text
$ wipctl next
supervised-child-runtime  Supervised child runtime  2pt  ~/.local/share/wipctl/projects/payments-acme/plan-repo/stories/supervised-child-runtime.md
```
