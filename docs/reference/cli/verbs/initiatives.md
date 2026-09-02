# wipctl initiatives

The rollup one tier above `epics`: every initiative, one row, delivered against promised. Twinned with `initiative` under ADR-0015's pairing rule; each verb's help names the other.

## Usage

```text
wipctl initiatives [--json]
```

## Contract

- One row per document under `initiatives/`, in id order. An initiative no epic has joined MUST appear at zero rather than being omitted. An empty or absent `initiatives/` prints nothing, exit 0 — its absence is legal and means the project uses two tiers.
- Terminal row: `<id, padded to the widest>  <bar>  <closed>/<total>  <open> open`, the same shape as `epics`. The bar is a fixed-width progress bar of closed points over total points.
- Membership MUST be searched from the epic documents' `Initiative` sections. Point arithmetic MUST be shared with `epic` and `epics` — one implementation computes all three tiers. An initiative's numbers are its member epics' numbers summed; because membership above the story flows only through the epic, every entry is counted at most once at every tier, with no deduplication step to get wrong.
- The verb MUST NOT report a staleness verdict or retirement advice: whether an initiative should be retired is a review question, asked alongside "is the charter still true".
- Derived on every call and stored nowhere.
- `--json`: an object conforming to `initiatives.schema.json`.

## Example

```text
$ wipctl initiatives
trustworthy-by-default   ████████▏               6/21   11 open
```
