# Wiring your project's documents

How a story references what your project already has. Nothing here requires reorganising anything. [explanation/host-integration.md](../explanation/host-integration.md) holds the doctrine behind the steps.

1. Confirm the root. `.wipctl/project.toml` sits at the project root, and its `project_id` agrees with the plan configuration found by identity. A disagreement fails validation naming both sides.
2. Answer the scaffold's host instruction prompt:
   - Enter `<path>` for the repository-relative path of the file the project already uses.
   - Enter an empty answer to skip the host pointer.

   The scaffold prints a marked block for a path and prints nothing for a skip. It leaves the host file unchanged.

3. Place the printed block in the named file. Check that it names `wipctl man` and no filesystem path.
4. Choose documents from what exists. A project keeping structured reference pages points stories at those. A project keeping a flat notes tree points stories at the notes. Both are correct. The method validates that a named path resolves, and nothing more.
5. Write `Reads`. State the claim, not the coordinates:
   - Link to a stable heading anchor, never to a line number.
   - Name the page that owns a rule, rather than the eleven records that shaped it.
   - Write `None.` when nothing genuinely must be loaded.

   Where the host's documentation method rules a class of document out of a work session's load set, that exclusion binds here too. No gate reads an inbound path, so honouring it is the reviewer's.

6. Write `Amends`. There are three gated rules:
   - the path is relative
   - it resolves against the root, unless the assertion opens with `new:`
   - only the leading inline-code token is the path

   Write `None` when the work changes no durable document. Where the work changes a rule the host identifies by id, type it, one rule per item:

   ```markdown
   - `docs/specs/SPEC-auth.md` — ADDED `auth:token-expiry-is-bounded`
   ```

7. Complete the transfer when the story closes. Rewrite the acceptance assertions in the present tense into the amended documents, in the same change as the behavior. This is the step the whole doctrine depends on, and the one no tool sees.
8. Verify, then prove the check is non-vacuous once. Break one `Amends` path, watch the gate fail, and restore it:

   ```text
   $ wipctl validate
   ```

These are review-only responsibilities, named because no gate holds them:

- the named document is the right one
- the transfer happened
- a promise marked `new:` was kept
- `Reads` names the documents a session genuinely needs
- `Sources` names the outside items the work genuinely answers
- a reference is genuinely load-bearing, or genuinely changed
- a story that changed a rule declared its typed delta, and the type matches the diff
