# Adopting wipctl

The sequence that gives your project a validated plan. It lands one identity file in your repository, and a plan repository of its own on this machine. It assumes only what the method assumes: your project can carry one committed file at its root.

## 1 — Install and scaffold

Inputs: none.

1. Install the tool and confirm it:

   ```text
   $ wipctl version
   ```

2. Preview, then scaffold, at the project root:

   ```text
   $ wipctl init --dry-run
   $ wipctl init
   ```

   The scaffold writes into two places. It writes `.wipctl.toml` into your repository, carrying the minted project id. That is the only file your repository receives. It writes the plan repository into this machine's registry slot, with its zone, its configuration, its hook set, and its first commit. The scaffold is create-only, so a second run writes nothing. It ends with a self-check of the zone it emitted.

3. Commit `.wipctl.toml`, so every clone and worktree of your project resolves the same plan.
4. Note the shipped template location the scaffold prints. The first story starts from it.

Outputs of this phase:

```text
<PROJECT_ID> — the minted identity; printed by init and recorded in .wipctl.toml
<PLAN_REPO_PATH> — where the plan lives on this machine; printed by init
<STORY_TEMPLATE_PATH> — where the shipped story template lives; printed by init
```

## 2 — Host the plan, when you want it hosted

Inputs: `<PLAN_REPO_PATH>` (§1).

1. The plan works locally with no remote. When you want backup, replication, or a second machine, create an empty repository at any forge. Private or public is your choice. Add it as the plan repository's remote.
2. Push with the tool, not by hand:

   ```text
   $ wipctl sync
   ```

3. On any other machine, clone your project and attach its plan:

   ```text
   $ wipctl attach <plan-repo-url>
   ```

Outputs: none. Every machine now resolves the same record through the same committed id.

## 3 — Fill the charter

Inputs: `<PLAN_REPO_PATH>` (§1).

1. Open `charter.md` in the plan repository. Replace every angle-bracketed placeholder: what your project is for, its pillars, its no-gos, and its cadence. The cadence also lives in the plan repository's `config.toml`, where the tool reads it.

Outputs: none. The charter is read in review, not by a later phase.

## 4 — Write the first story

Inputs: `<STORY_TEMPLATE_PATH>` (§1).

1. Capture it, complete the fragment, and land it:

   ```text
   $ wipctl new story "<title>" --lane todo
   ```

   Open the printed fragment and fill the entry's `points` and `summary`. Capture writes both absent, and the drain refuses an incomplete fragment. Then:

   ```text
   $ wipctl land
   ```

2. Fill the document. [writing-a-story.md](./writing-a-story.md) is the sequence.

Outputs of this phase:

```text
<FIRST_STORY_ID> — the new entry's id, which is the title's slug and the story document's filename stem
```

## 5 — Gate the record

Inputs: `<FIRST_STORY_ID>` (§4).

1. Validate. It runs from anywhere inside your project, and the `schemas:` line names which halves ran:

   ```text
   $ wipctl validate
   ```

2. The record's gates arrived with the plan repository. The scaffold installed its hook set: validation at commit and at push, the heading-shape checks, and the single-branch and no-force guards. Confirm they are live:

   ```text
   $ wipctl doctor
   ```

3. Prove the gates by breaking something once. A hook that never selected a file is indistinguishable from a passing one. Revert, and the gates are trusted.
4. Never wire the repair or the drain into a hook. No hook invokes a writer.

Outputs: none.

## 6 — Start

Inputs: none.

1. Take the work atomically:

   ```text
   $ wipctl start
   ```

   It prints `<FIRST_STORY_ID>` and moves it to `doing`, in one transaction. From here the loop is the method's. Work it, move it to `review`, close it, and ask again.

Outputs: none.
