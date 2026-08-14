# wipctl graph

The dependency graph as a ladder: one row per node in topological order, one column per edge in
flight — a rendering that needs no layout engine and no dependency beyond the base runtime.

## Usage

```text
wipctl graph [--unblocks <id>] [<plan-dir>]
```

## Contract

- One row per entry, in the record library's topological order; the node's id and title sit to
  the right of the glyph columns.
- Edge kinds: dependency edges from `needs`; question edges from `Blocks:` lines, drawn with a
  distinct glyph. Epic membership MUST NOT be drawn as an edge.
- Column lifecycle: a column opens when a node is drawn, closes at that node's last child, and is
  reused immediately, so the drawing's width is the number of edges in flight, not the number of
  edges. Junction glyphs are chosen from the four up/down/left/right bits, Box Drawing tier with
  a `--plain` ASCII fallback.
- `--unblocks <id>` answers the planning question directly:

  ```text
  $ wipctl graph --unblocks rate-limit-the-search-endpoint-a7f3
  closing rate-limit-the-search-endpoint-a7f3 makes eligible: profile-composition-e01a
  ```

- A record with no edges renders the bare node list. The drawing MUST survive a pipe; the verb
  MUST NOT gate and MUST NOT write.

## Out of scope

Edge routing, crossing minimisation, nested or clustered layouts, a second layout behind a flag.
