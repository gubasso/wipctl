//! Case for ADR-0044: the shipped payload names no documentation method.
//!
//! ADR-0033 keeps the host unassumed, and this is the half of it a command
//! can decide. The denylist holds product names only. Generic convention many
//! methods share — a `SPEC-` or `ADR-` filename in an illustration — stays
//! legal, because the reference must be able to show an `Amends` item written
//! against a host that keeps requirement-level specifications.
//!
//! The scanned roots are the normative reference and the sources built from
//! it, which is what reaches an adopting project.

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

fn scanned_roots() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    ["docs", "src"]
        .iter()
        .map(|name| root.join(name))
        .filter(|path| path.exists())
        .collect()
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

    let mut files = Vec::new();
    for root in &roots {
        walk(root, &mut files);
    }
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
