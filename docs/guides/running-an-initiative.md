# Running an initiative

Declare, watch, and retire an end state larger than one epic. Why the tier is shaped this way — and why most projects never need it — is [explanation/initiatives.md](../explanation/initiatives.md); this page is the sequence.

## 1 — Open one

Inputs: none.

1. Apply the test first: open an initiative when several epics turn out to serve one end state that is invisible from any of them. Two epics rarely qualify. Do not open: an initiative with one epic (an epic wearing a container); an initiative per quarter or per release (a calendar, not an end state); a container before the epics exist.
2. Mint an id from the shared namespace — the title's slug — and write `initiatives/<id>.md` in the initiative shape. The document lists no members, carries no status, and names no parent: the ladder stops here.
3. Write the end state, not a summary; make `Example` a simulation — what is true today against what is true at the end; make `Done when` observable, never "every member closed".
4. Put the id in each serving epic's `Initiative` section — the section holds one inline-code id, and the epics that do not serve it say `None`.

Outputs of this phase:

```text
<INITIATIVE_ID> — the initiative's id; the filename stem under initiatives/, named by each member epic's Initiative section
```

## 2 — Watch one

Inputs: `<INITIATIVE_ID>` (§1).

1. Ask for the decomposition:

   ```text
   $ wipctl initiative <INITIATIVE_ID>
   ```

   One row per member epic, delivered against promised. This is a report, not an execution plan — a session works one end state at a time, so the startable entries are the epic verb's answer.

2. Drive each member epic as its own end state; [running-an-epic.md](./running-an-epic.md) is that loop. The initiative verb's output is the list to run it against.
3. The rollup across every initiative is `wipctl initiatives`, the same shape one tier up from `epics`.

Outputs: none.

## 3 — Retire one

Inputs: `<INITIATIVE_ID>` (§1).

1. An initiative whose every member epic is closed out needs no action; closed out is not done, and whether it delivered its `Done when` is a judgment, not a count.
2. An initiative nobody is pursuing is retired by a person, in review, alongside "is the charter still true". Nothing derives retirement from the numbers, at this tier or any other.

Outputs: none.
