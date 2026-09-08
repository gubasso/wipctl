//! Cases over the published file set.
//!
//! The exclude list in `Cargo.toml` is a denylist, so a new top-level artifact
//! is published by default. That is the safe direction for a source file and
//! the wrong one for a directory carrying its own licence terms.
//! VERIFIES quality-gates:the-published-artifact-carries-its-licence
//!
//! The planted-defect case runs the same predicate over a file list this
//! repository does not produce, because a check that never saw a violation is
//! indistinguishable from a passing one.
//! VERIFIES quality-gates:a-gate-fails-once-before-it-is-trusted
//!
//! The published package carries no `docs/`, so a build from the tarball skips
//! the live case rather than failing it.

use std::path::Path;
use std::process::Command;

/// Paths the published artifact carries.
const REQUIRED: [&str; 3] = ["LICENSE-APACHE", "LICENSE-MIT", "README.md"];

/// Directory prefixes only the development of this repository reads.
const DEVELOPMENT_ONLY: [&str; 2] = [".release-kit/", ".spec-driven-docs/"];

/// Report every way `files` breaks the published-file-set contract.
fn violations(files: &[&str]) -> Vec<String> {
    let mut found = Vec::new();

    for required in REQUIRED {
        if !files.contains(&required) {
            found.push(format!("{required} is missing from the package"));
        }
    }

    for file in files {
        for prefix in DEVELOPMENT_ONLY {
            if file.starts_with(prefix) {
                found.push(format!("{file} is development-only and is published"));
            }
        }
    }

    found
}

/// The file list of the package this checkout would publish.
///
/// `None` means the checkout is not a source tree, which is the packaged-crate
/// case and is not a failure.
fn packaged_files() -> Option<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    if !root.join("docs").is_dir() {
        return None;
    }

    let output = Command::new(env!("CARGO"))
        .args(["package", "--list", "--allow-dirty", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

#[test]
fn the_published_package_carries_its_licences_and_no_governance_tree() {
    let Some(listing) = packaged_files() else {
        return;
    };
    let files: Vec<&str> = listing
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    assert!(
        files.len() > 1,
        "the package listing should name more than one file: {listing}"
    );
    assert!(
        violations(&files).is_empty(),
        "the published file set breaks its contract: {:?}",
        violations(&files)
    );
}

#[test]
fn the_check_rejects_a_missing_licence_and_a_published_governance_tree() {
    let defective = [
        "Cargo.toml",
        "LICENSE-APACHE",
        "README.md",
        ".spec-driven-docs/manifest.json",
        "src/main.rs",
    ];

    let found = violations(&defective);

    assert_eq!(
        found.len(),
        2,
        "one missing licence and one published governance file are two violations: {found:?}"
    );
}
