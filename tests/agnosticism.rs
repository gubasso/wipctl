//! VERIFIES scaffold:the-payload-names-no-documentation-method
//!
//! The denylist holds product names only. Generic convention many methods
//! share — a `SPEC-` or `ADR-` filename in an illustration — stays legal,
//! because the specification must be able to show an `Amends` item written
//! against a host that keeps requirement-level specifications.
//!
//! The scanned roots are what reaches an adopting project: this project's own
//! specification and the sources built from it. The documentation framework
//! this repository is written under installed files of its own under the same
//! roots. Those files govern how this repository documents itself and reach no
//! adopting project, so the instance manifest's own list of them is what the
//! scan skips. Reading the list rather than restating it keeps the exclusion
//! from drifting when the framework changes what it seeds.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Documentation methods and the tooling that installs one.
const DOCUMENTATION_METHODS: &[&str] = &[
    "spec-driven-docs",
    "spec driven docs",
    "openspec",
    "spec kit",
    "spec-kit",
    "strictdoc",
    "openfasttrace",
    "diataxis",
    "docusaurus",
    "mkdocs",
    "antora",
];

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn scanned_roots() -> Vec<PathBuf> {
    ["docs", "src"]
        .iter()
        .map(|name| repo_root().join(name))
        .filter(|path| path.exists())
        .collect()
}

/// Every file the documentation framework installed, by absolute path.
///
/// An absent or unreadable manifest yields an empty set, which scans more
/// rather than less. A missing exclusion fails the case; it never hides one.
fn framework_files() -> HashSet<PathBuf> {
    let manifest = repo_root().join(".spec-driven-docs/manifest.json");
    let Ok(text) = std::fs::read_to_string(manifest) else {
        return HashSet::new();
    };

    let mut excluded = HashSet::new();
    for line in text.lines() {
        let Some(rest) = line.trim().strip_prefix("\"destination\":") else {
            continue;
        };
        let trimmed = rest.trim().trim_end_matches(',').trim_matches('"');
        if !trimmed.is_empty() {
            excluded.insert(repo_root().join(trimmed));
        }
    }
    excluded
}

fn walk(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

#[test]
fn the_payload_names_no_documentation_method() {
    let roots = scanned_roots();
    assert!(!roots.is_empty(), "no scanned root is on disk");

    let excluded = framework_files();
    let mut files = Vec::new();
    for root in &roots {
        walk(root, &mut files);
    }
    files.retain(|path| !excluded.contains(path));
    assert!(!files.is_empty(), "the scan matched no file");

    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy();
        for (index, line) in text.lines().enumerate() {
            let lower = line.to_lowercase();
            for method in DOCUMENTATION_METHODS {
                assert!(
                    !lower.contains(method),
                    "{shown}:{}: names the documentation method '{method}'",
                    index + 1
                );
            }
        }
    }
}

#[test]
fn the_scan_excludes_the_framework_and_nothing_more() {
    let excluded = framework_files();
    assert!(
        !excluded.is_empty(),
        "the instance manifest must list the framework's own files"
    );
    assert!(
        excluded.iter().all(|path| path.starts_with(repo_root())),
        "an exclusion outside the repository would silence an unrelated file"
    );
    assert!(
        !excluded
            .iter()
            .any(|path| path.starts_with(repo_root().join("src"))),
        "no source file is excluded from the scan"
    );
}
