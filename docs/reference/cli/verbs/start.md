# wipctl start

Takes work atomically: reads the head of `todo`, moves it to `doing`, and prints it, in one transaction. `next` tells you; `start` takes it. With several agents pulling from one lane, every agent that runs `next` sees the same head and all but one lose the race at `move`; `start` makes them fan out onto different entries instead ([../../../explanation/concurrency.md](../../../explanation/concurrency.md), ADR-0053). The two verbs are twins and each names the other, under ADR-0015's pairing rule.

## Usage

```text
wipctl start
```

No arguments; any argument is exit 2. `--help` prints usage on stdout, exit 0.

## Contract

- `start` is a writer. Inside one lock holding it reads at current HEAD, chooses the head of `todo`, runs the full `move`-to-`doing` preflight, writes, repairs ranking, commits (`plan: move <id> from todo to doing`), and releases. The id is chosen inside the lock: two agents running `start` at the same instant get two different entries, because the second reads a `todo` whose head the first already removed.
- It prints exactly the line `next` would have printed for the entry it took: `<id>  <title>  <points>pt  <absolute story path>`.
- Nothing startable — an empty `todo`, or a head that is not eligible, which by ranking rule R1 means no entry in the lane is — is exit 1 naming why, so a fan-out loop terminates instead of spinning.
- It MUST NOT take an entry out of `doing`; `doing` having entries does not stop it.
- One line of output, so under ADR-0013 it offers no machine format and owes no schema — the same exemption `next` and `ids` carry.

## Example

```text
$ wipctl start
rate-limit-the-search-endpoint  Rate limit the search endpoint  2pt  ~/.local/share/wipctl/projects/payments-acme/plan-repo/stories/rate-limit-the-search-endpoint.md
```

## Diagnostics

```text
nothing startable                                                         exit 1
  wipctl: nothing to start: todo is empty
  wipctl: capture work with 'wipctl new', or promote from backlog

nothing startable, blocked head                                           exit 1
  wipctl: nothing to start: no entry in todo is eligible
  wipctl: profile-composition needs rate-limit-the-search-endpoint,
          which is in doing; close it, or answer the question blocking
          another entry ('wipctl doctor' lists every blocker)
```
