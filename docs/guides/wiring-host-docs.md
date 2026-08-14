# Wiring your project's documents

How a story references what your project already has. Nothing here requires reorganising
anything; the doctrine behind the steps is
[explanation/host-integration.md](../explanation/host-integration.md).

1. Confirm the root: `.wipctl.toml` sits at the project root, and `plan_dir` still describes
   where the zone actually is. A stale declaration fails validation naming both sides.
2. Choose documents from what exists. A project keeping structured reference pages points stories
   at those; a project keeping a flat notes tree points stories at the notes. Both are correct:
   the method validates that a named path resolves, and nothing more.
3. Write `Governed by`. State the claim, not the coordinates: link to a stable heading anchor,
   never a line number; name the page that owns a rule rather than eleven records that shaped it;
   write `None.` when genuinely nothing governs.
4. Write `Amends`. The three gated rules: the path is relative; it resolves against the root
   unless the assertion opens with `new:`; only the leading inline-code token is the path. Write
   `None` when the work changes no durable document.
5. Complete the transfer when the story closes. Rewrite the acceptance assertions in the present
   tense into the amended documents, in the same change as the behaviour. This is the step the
   whole doctrine depends on, and the one no tool can see.
6. Verify, then prove the check is non-vacuous once — break one `Amends` path, watch the gate
   fail, restore it:

   ```text
   $ wipctl validate
   ```

Review-only responsibilities, named because no gate holds them: the named document is the right
one; the transfer happened; a `new:` promise was kept; `Governed by` names the sources a session
genuinely needs; a reference genuinely governs or is genuinely changed.
