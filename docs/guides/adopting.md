# Adopting wipctl

The sequence that puts a validated plan zone in your project. It assumes only what the method assumes: your project has a documentation directory.

## 1 — Install and choose the zone

Inputs: none.

1. Install the tool and confirm it:

   ```text
   $ wipctl version
   ```

2. Decide where the zone lives. There is no default; `docs/plan` is only the most common answer. The zone can sit at any depth inside your documentation directory.

Outputs of this phase:

```text
<PLAN_DIR> — the zone path, relative to the project root; recorded in .wipctl.toml by the scaffold
```

## 2 — Scaffold

Inputs: `<PLAN_DIR>` (§1).

1. Preview, then scaffold:

   ```text
   $ wipctl init --plan-dir <PLAN_DIR> --dry-run
   $ wipctl init --plan-dir <PLAN_DIR>
   ```

   Without the flag and with a terminal, the verb prompts; without a terminal it refuses (exit 2). The scaffold is create-only — a second run writes nothing — and ends with a self-check of the zone it emitted.

2. Note the shipped template location the scaffold prints; the first story starts from it.

Outputs of this phase:

```text
<STORY_TEMPLATE_PATH> — where the shipped story template lives; printed by init
```

## 3 — Fill the charter

Inputs: none.

1. Open `charter.md` in the zone and replace every angle-bracketed placeholder: what your project is for, its pillars, its no-gos, and its cadence. The cadence also lives in `.wipctl.toml`, where the tool reads it.

Outputs: none — the charter is read in review, not by a later phase.

## 4 — Write the first story

Inputs: `<STORY_TEMPLATE_PATH>` (§2).

1. Either capture it, complete the fragment, and land it:

   ```text
   $ wipctl new story "<title>" --lane todo
   ```

   Open the printed fragment and fill the entry's `points` and `summary` — capture writes both absent, and the drain refuses an incomplete fragment. Then:

   ```text
   $ wipctl land
   ```

   or copy the template to `stories/<slug>-<uid>.md`, mint the identity as the template shows, and add its lane entry to `todo.yml` by hand.

2. Fill the document; [writing-a-story.md](./writing-a-story.md) is the sequence.

Outputs of this phase:

```text
<FIRST_STORY_ID> — the new entry's id; the filename stem of the story document
```

## 5 — Gate the record

Inputs: `<FIRST_STORY_ID>` (§4).

1. Validate — it runs from anywhere inside the project, and the `schemas:` line names which halves ran:

   ```text
   $ wipctl validate
   ```

2. Confirm the record answers:

   ```text
   $ wipctl next
   ```

   It prints `<FIRST_STORY_ID>`.

Outputs: none.

## 6 — Wire the hooks

Inputs: `<PLAN_DIR>` (§1).

1. Emit the hook entries with your zone path substituted — the verb writes nothing:

   ```text
   $ wipctl init --print-hooks
   ```

2. Copy them into your hook runner's configuration. Pin the versions, point the schema entries at your installed copies, and register the record gates at both the commit and push stages.
3. Never wire the repair (`fix`) or the drain (`land`) into a hook; no hook invokes a writer.
4. Gate the heading shapes:

   ```text
   $ wipctl init --headings-gate
   ```

   It writes the two required-headings configurations — one per document shape — and prints their hook entries. Precondition: your general lint configuration must not set a required-headings rule at any value, or it would silently override the dedicated files — the verb refuses if it does. Add the epic-shape hook entry only once `epics/` has a file, if your hook runner errors on a pattern matching nothing.

Outputs: none — the wired gates live in your hook runner's configuration.

## 7 — Start, and prove the gates

Inputs: `<FIRST_STORY_ID>` (§4).

1. Start the work:

   ```text
   $ wipctl move <FIRST_STORY_ID> --to doing
   ```

   The report shows the per-file deltas, the journal event, and the ranking repair.

2. Prove each gate is live by breaking something once — a hook that never selected a file is indistinguishable from a passing one. Revert, and you are done.

Outputs: none.
