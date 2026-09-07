//! Cases for the prose gate in `scripts/`.
//!
//! Every shipped artifact owes a case in the test suite, and a gate owes more
//! than that: it must be shown to fail on a deliberate defect, because a gate
//! that never selected a file is indistinguishable from a passing one.
//! VERIFIES quality-gates:a-gate-fails-once-before-it-is-trusted
//!
//! The scripts are repository tooling and are excluded from the published
//! package, so each case resolves them from the manifest directory and skips
//! when they are absent rather than failing a packaged build.
//!
//! The fallible calls sit in the `#[test]` bodies rather than in the helper:
//! `allow-expect-in-tests` recognizes a test function, not a plain helper an
//! integration test happens to call.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn script(name: &str) -> Option<PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join(name);
    path.exists().then_some(path)
}

/// Write `body` to a uniquely named file and report whether the gate accepted it.
///
/// `Ok(None)` means the script is absent, which is the packaged-crate case and
/// is not a failure.
fn accepts(script_name: &str, case: &str, body: &str) -> io::Result<Option<bool>> {
    let Some(script) = script(script_name) else {
        return Ok(None);
    };

    let file = std::env::temp_dir().join(format!("wipctl-{script_name}-{case}.md"));
    fs::write(&file, body)?;

    let status = Command::new(&script).arg(&file).status();
    fs::remove_file(&file).ok();

    status.map(|status| Some(status.success()))
}

#[test]
fn emphasis_gate_rejects_bold_and_italics() {
    let bold = accepts("check-emphasis", "bold", "# T\n\nA **bold** span.\n")
        .expect("the gate should be runnable");
    let Some(bold) = bold else { return };
    assert!(!bold, "bold must fail the gate");

    let italic = accepts("check-emphasis", "italic", "# T\n\nAn _italic_ span.\n")
        .expect("the gate should be runnable")
        .expect("the script was present a moment ago");
    assert!(!italic, "italics must fail the gate");
}

#[test]
fn emphasis_gate_ignores_code_and_honours_the_escape_hatch() {
    // A glob inside a fence, a literal in a code span, and a snake_case
    // identifier are not emphasis. This is the false-positive case that
    // decides whether the gate is usable at all.
    let safe = accepts(
        "check-emphasis",
        "safe",
        "# T\n\n```text\nsee **/*.md and *ptr\n```\n\nA `**literal**` span, snake_case_word.\n",
    )
    .expect("the gate should be runnable");
    let Some(safe) = safe else { return };
    assert!(safe, "fenced blocks, code spans and snake_case must pass");

    let escaped = accepts(
        "check-emphasis",
        "escape",
        "# T\n\n<!-- allow-emphasis: quoting upstream verbatim -->\nIt printed **FAILED**.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(escaped, "the escape hatch must exempt the following line");
}
