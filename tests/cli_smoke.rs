//! Smoke test for the installed `wipctl` binary: it starts, prints, and exits clean.
//!
//! Not a test of any command's behaviour — each command owns its own integration
//! test file. This one guards the entry point and its exit-code mapping.

use std::process::Command;

#[test]
fn binary_runs_and_exits_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_wipctl"))
        .output()
        .expect("the wipctl binary should be runnable");

    assert!(
        output.status.success(),
        "wipctl exited with {}",
        output.status
    );

    let stdout = String::from_utf8(output.stdout).expect("wipctl should print valid UTF-8");
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "wipctl should report its version, printed: {stdout:?}"
    );
}
