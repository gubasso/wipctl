# wipctl aging

How long the in-flight work has been in flight, with the alarm carried by a glyph so it survives a pipe.

## Usage

```text
wipctl aging [<plan-dir>]
```

## Contract

- One bar per entry in `doing` and `review`, scaled to the longest, labelled `id  title  <bar>  Nd`.
- Ages MUST come from the `flow` code path — the journal fold — never recomputed here; the two verbs cannot disagree.
- When `.wipctl.toml` carries `aging.threshold_days`, rows past the threshold MUST carry a warning glyph (`⚠`), and a rule line marks the threshold: `└──────┴ threshold 21d`. Absent the key, bars render with no marks — there is no default threshold, and the verb MUST NOT gate on it.
- An entry unmeasured by `flow` MUST be shown unmeasured with its reason.
- Empty work lanes render nothing and exit 0.

## Example

```text
$ wipctl aging
supervised-child-runtime-b47d  Supervised child runtime  ████████████████████  24d ⚠
oauth-hardening-2f4b           OAuth hardening           █████▏                 6d
└──────────────────────────────────────────────┴ threshold 21d
```
