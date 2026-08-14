# wipctl board

The five lanes on one screen, derived on every run, never written anywhere.

## Usage

```text
wipctl board [<plan-dir>]
```

## Contract

- Renders the five lanes in workflow order (`backlog`, `todo`, `doing`, `review`, `closed`), each with its entry count, each row showing the entry's id and summary — which MUST be read from the lane entry, never by opening story documents.
- The current story — topmost of `doing`, else topmost of `todo` — MUST be marked with a marker glyph (`▸`).
- Blocked entries MUST be shown, never hidden, with distinct badges for dependency-blocked and question-blocked, distinguishable with escapes stripped (glyph, not colour, carries the distinction).
- Wide layout joins lanes as columns; below the 100-column breakpoint lanes stack, `todo` leading.
- An empty record renders every lane at zero and exits 0. The verb MUST NOT gate and MUST NOT write.
- No interaction: moving a card is a judgment reserved for a person and a `move` invocation.

## Example

```text
$ wipctl board
todo (2)                              doing (1)
  rate-limit-the-search-endpoint-a7f3   ▸ supervised-child-runtime-b47d
  profile-composition-e01a ⛓
```
