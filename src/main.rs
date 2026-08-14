//! Process entry point for the `wipctl` command: startup and exit-code mapping.
//!
//! Not a home for product logic. Argument parsing, command dispatch, the plan-record
//! domain, and terminal output each land in their own modules as the command surface
//! is built; `main` only ever turns a result into an exit status.

use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("wipctl: {message}");
            // BSD sysexits EX_SOFTWARE.
            ExitCode::from(70)
        }
    }
}

/// The fallible program. Every failure path returns here so that `main` stays the
/// single place the process decides its exit status.
#[expect(
    clippy::unnecessary_wraps,
    reason = "the entry point is fallible by contract; the first command that can \
              fail makes the wrap load-bearing and retires this attribute"
)]
fn run() -> Result<(), String> {
    println!("wipctl {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn run_succeeds() {
        assert!(run().is_ok());
    }
}
