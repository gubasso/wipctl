//! Cases for the schema gate in `scripts/check-schemas`.
//!
//! The gate has two halves. One validates every shipped schema against the
//! draft 2020-12 metaschema, because a schema that is not itself valid gates
//! nothing. The other validates the two-plan fixture against the schemas that
//! govern it, because a valid schema can still say the wrong thing.
//! VERIFIES quality-gates:every-schema-validates-against-the-metaschema
//!
//! Each case below plants a deliberate defect and asserts that the gate fails
//! on it. A gate whose selector reaches no file reports success without
//! reading one, and only a planted defect tells the two apart.
//! VERIFIES quality-gates:a-gate-fails-once-before-it-is-trusted
//!
//! The script is repository tooling and is excluded from the published
//! package, so each case resolves it from the manifest directory and skips
//! when it is absent rather than failing a packaged build.
//!
//! The fallible calls sit in the `#[test]` bodies rather than in the helpers:
//! `allow-expect-in-tests` recognizes a test function, not a plain helper an
//! integration test happens to call.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The gate's exit code for "my validator is not on PATH".
const VALIDATOR_ABSENT: i32 = 3;

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn script() -> Option<PathBuf> {
    let path = repo().join("scripts").join("check-schemas");
    path.exists().then_some(path)
}

/// Copy `from` onto `to`, creating every directory on the way.
fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// A scratch directory that removes itself when the case ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(case: &str) -> io::Result<Self> {
        let path = std::env::temp_dir().join(format!("wipctl-check-schemas-{case}"));
        fs::remove_dir_all(&path).ok();
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).ok();
    }
}

/// Run the gate with the given arguments and report whether it accepted the tree.
///
/// `Ok(None)` means the gate could not judge: the script is absent, which is
/// the packaged-crate case, or its validator is absent, which is any build
/// that does not supply the pinned tool.
///
/// The second case is the one worth stating. Without the validator every
/// invocation fails, and a case asserting a refusal then passes without the
/// refusal ever being earned. That is the exact failure this file exists to
/// catch, so the script reports the absence as its own exit code and the
/// suite skips rather than believing it.
fn accepts(args: &[&str]) -> io::Result<Option<bool>> {
    let Some(script) = script() else {
        return Ok(None);
    };

    let status = Command::new(&script)
        .args(args)
        .current_dir(repo())
        .status()?;

    if status.code() == Some(VALIDATOR_ABSENT) {
        return Ok(None);
    }

    Ok(Some(status.success()))
}

#[test]
fn the_gate_reports_a_missing_validator_as_its_own_code() {
    // The skip path above is load-bearing, so it is proven rather than
    // trusted. Running the gate with a path that carries every tool it needs
    // except the validator must produce the skip code, and not a refusal.
    let Some(script) = script() else { return };

    let scratch = Scratch::new("no-validator").expect("a scratch directory should be creatable");
    let bin = scratch.path().join("bin");
    fs::create_dir_all(&bin).expect("a scratch directory should be creatable");

    // Everything the script runs, minus check-jsonschema.
    for tool in ["bash", "git", "find", "sort", "head"] {
        let Ok(found) = which(tool) else {
            return; // an environment this case cannot construct
        };
        std::os::unix::fs::symlink(found, bin.join(tool)).expect("the symlink should be creatable");
    }

    let status = Command::new(&script)
        .current_dir(repo())
        .env("PATH", &bin)
        .status()
        .expect("the gate should be runnable");

    assert_eq!(
        status.code(),
        Some(VALIDATOR_ABSENT),
        "a missing validator must be its own exit code, not a refusal"
    );
}

/// Resolve a tool on the current path, so the case above can rebuild a path
/// without one of them.
fn which(tool: &str) -> io::Result<PathBuf> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path)
        .map(|dir| dir.join(tool))
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, tool.to_owned()))
}

/// Copy the fixture, replace `from` with `to` in one of its files, and report
/// whether the gate accepted the result.
///
/// One helper rather than one block per defect: the interesting part of each
/// case below is the defect it plants, and repeating the copy around it hides
/// that.
fn accepts_fixture_with(case: &str, file: &str, from: &str, to: &str) -> io::Result<Option<bool>> {
    let scratch = Scratch::new(case)?;
    let fixtures = scratch.path().join("two-plans");
    copy_tree(&repo().join("tests/fixtures/two-plans"), &fixtures)?;

    let target = fixtures.join(file);
    let body = fs::read_to_string(&target)?;
    assert!(
        body.contains(from),
        "the fixture no longer holds {from:?}, so this case plants nothing"
    );
    fs::write(&target, body.replace(from, to))?;

    accepts(&["--fixtures", &fixtures.to_string_lossy()])
}

#[test]
fn the_gate_accepts_the_shipped_tree() {
    let ok = accepts(&[]).expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(
        ok,
        "every shipped schema and every fixture instance must pass the gate"
    );
}

#[test]
fn a_malformed_plan_uid_fails() {
    // The identity's whole grammar is 32 lowercase hexadecimal characters, and
    // a value one character short is the defect a hand edit produces.
    let ok = accepts_fixture_with(
        "plan-uid",
        "alpha/config.toml",
        "9f2c41a08b7d4e63a15c8f02d7e4b619",
        "9f2c41a08b7d4e63a15c8f02d7e4b61",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a plan_uid outside its grammar must fail the gate");
}

#[test]
fn a_lane_entry_outside_the_schema_fails() {
    // Four points does not exist: work above three splits along its judgments.
    let ok = accepts_fixture_with(
        "lane-entry",
        "alpha/lanes/todo.yml",
        "points: 2",
        "points: 4",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a point value off the scale must fail the gate");
}

#[test]
fn a_peer_row_without_a_uid_fails() {
    // A peer is identified by the uid its own plan declares. A row naming only
    // a location names a place, not a plan.
    let ok = accepts_fixture_with(
        "peer-no-uid",
        "alpha/peers.toml",
        "uid = \"4c81d0e7f39a4b25861d7c04e9a2f358\"\n",
        "",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a peer row with no uid must fail the gate");
}

#[test]
fn a_peer_row_without_a_location_fails() {
    // A peer nobody can locate is not a peer, and no default is invented for it.
    let ok = accepts_fixture_with(
        "peer-no-url",
        "alpha/peers.toml",
        "[\"https://git.example.org/acme/beta-plan.git\"]",
        "[]",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a peer row with an empty url list must fail the gate");
}

#[test]
fn a_peer_alias_outside_the_slug_grammar_fails() {
    // The alias is a slug. An uppercase key is the defect a hand edit produces,
    // and it is the one a case-insensitive filesystem hides.
    let ok = accepts_fixture_with(
        "peer-alias",
        "alpha/peers.toml",
        "[peers.beta]",
        "[peers.Beta]",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "an alias outside the slug grammar must fail the gate");
}

#[test]
fn a_peer_url_carrying_a_password_fails() {
    // A committed file is read by everyone who clones this plan, so a password
    // in it is a leak whatever the tool prints. A bare account name is legal,
    // which is why the rule turns on the colon rather than on the at sign.
    let ok = accepts_fixture_with(
        "peer-url",
        "alpha/peers.toml",
        "https://git.example.org",
        "https://someone:PLACEHOLDER@git.example.org",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a url carrying a password must fail the gate");
}

#[test]
fn a_peer_url_hiding_a_password_in_an_escape_fails() {
    // The rule turns on the colon, so the escape for a colon has to be
    // refused with it. Without that, %3A carries a password past a check that
    // reads the url as written.
    let ok = accepts_fixture_with(
        "peer-url-escaped",
        "alpha/peers.toml",
        "https://git.example.org",
        "https://someone%3APLACEHOLDER@git.example.org",
    )
    .expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(
        !ok,
        "a url hiding a password in an escape must fail the gate"
    );
}

#[test]
fn a_fixture_file_no_schema_governs_fails() {
    // The instance half selects a schema from the file's name and position. A
    // file the map does not recognise must be reported rather than skipped,
    // because a skipped instance and a checked one look the same in a green
    // run, and a later phase adding one would never learn.
    let scratch = Scratch::new("unknown-file").expect("a scratch directory should be creatable");
    let fixtures = scratch.path().join("two-plans");
    copy_tree(&repo().join("tests/fixtures/two-plans"), &fixtures)
        .expect("the fixture should be copyable");

    fs::write(fixtures.join("alpha/sources.toml"), "[sources]\n")
        .expect("the scratch fixture should be writable");

    let fixtures = fixtures.to_string_lossy().into_owned();
    let ok = accepts(&["--fixtures", &fixtures]).expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a fixture file no schema governs must fail the gate");
}

#[test]
fn a_schema_the_draft_rejects_fails() {
    // The metaschema half is the one that catches a schema which is valid JSON
    // and not a valid schema. A type keyword taking a number is that defect.
    let scratch = Scratch::new("metaschema").expect("a scratch directory should be creatable");
    let schemas = scratch.path().join("specs");
    copy_tree(&repo().join("docs/specs"), &schemas).expect("the schema tree should be copyable");

    let schema = schemas.join("SPEC-configuration/config.schema.json");
    let body = fs::read_to_string(&schema).expect("the shipped schema should be readable");
    fs::write(
        &schema,
        body.replace("\"type\": \"object\"", "\"type\": 42"),
    )
    .expect("the copied schema should be writable");

    let schemas = schemas.to_string_lossy().into_owned();
    let ok = accepts(&["--schemas", &schemas]).expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(
        !ok,
        "a schema the draft 2020-12 metaschema rejects must fail the gate"
    );
}

#[test]
fn an_empty_schema_tree_fails() {
    // The failure this whole file exists to catch: a selector that reaches no
    // file reports success without reading one.
    let scratch = Scratch::new("empty-tree").expect("a scratch directory should be creatable");
    let schemas = scratch.path().join("specs");
    fs::create_dir_all(&schemas).expect("a scratch directory should be creatable");

    let schemas = schemas.to_string_lossy().into_owned();
    let ok = accepts(&["--schemas", &schemas]).expect("the gate should be runnable");
    let Some(ok) = ok else { return };
    assert!(!ok, "a schema tree holding nothing must fail the gate");
}
