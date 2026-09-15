//! Cases for the local prose and message gates in `scripts/`.
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
fn chapter_line_cap_rejects_the_first_line_over_the_cap() {
    // The boundary is the whole rule, so both sides of it are asserted. A cap
    // that is off by one rejects a legal chapter or admits an illegal one, and
    // neither shows up in a count taken well away from the edge.
    let body = |lines: usize| "x\n".repeat(lines);

    let at_cap =
        accepts("check-chapter-lines", "at-cap", &body(200)).expect("the gate should be runnable");
    let Some(at_cap) = at_cap else { return };
    assert!(at_cap, "200 lines is at the cap and must pass");

    let over_cap = accepts("check-chapter-lines", "over-cap", &body(201))
        .expect("the gate should be runnable")
        .expect("the script was present a moment ago");
    assert!(!over_cap, "201 lines is over the cap and must fail");
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
fn emphasis_gate_sees_between_two_double_backtick_spans() {
    // A double-backtick span exists to hold a backtick, so the gate strips it
    // before matching. Stripping from the first opener to the last closer would
    // take the prose between two spans with it, and the emphasis there would
    // never be checked.
    let hidden = accepts(
        "check-emphasis",
        "between",
        "# T\n\n``a`b`` **hidden emphasis** ``c`d``\n",
    )
    .expect("the gate should be runnable");
    let Some(hidden) = hidden else { return };
    assert!(
        !hidden,
        "emphasis between two code spans must fail the gate"
    );

    let inner = accepts(
        "check-emphasis",
        "inner",
        "# T\n\nA ``span with `ticks` inside`` and plain prose.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(inner, "a backtick inside a double-backtick span must pass");
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

#[test]
fn requirement_keyword_gate_rejects_a_binding_sentence_outside_the_spec_zone() {
    // The defect the gate exists for: a sentence that binds a reader while
    // carrying no rule id, no scenario, and no verification.
    let guide = accepts(
        "check-requirement-keywords",
        "guide",
        "# Adopting\n\nThe operator MUST run the command before the push.\n",
    )
    .expect("the gate should be runnable");
    let Some(guide) = guide else { return };
    assert!(
        !guide,
        "an uppercase keyword outside the spec zone must fail"
    );

    // Every other RFC 2119 keyword binds the same way, and a negated form
    // opens with one already listed, so the pattern needs no case of its own.
    for (case, keyword) in [
        ("shall", "SHALL"),
        ("should-not", "SHOULD NOT"),
        ("may", "MAY"),
        ("required", "REQUIRED"),
    ] {
        let body = format!("# T\n\nThe record is {keyword} carry the field.\n");
        let accepted = accepts("check-requirement-keywords", case, &body)
            .expect("the gate should be runnable")
            .expect("the script was present a moment ago");
        assert!(!accepted, "{keyword} must fail the gate");
    }
}

#[test]
fn requirement_keyword_gate_ignores_code_and_honours_the_escape_hatch() {
    // The false-positive case that decides whether the gate is usable. A
    // keyword inside a fence or a code span is a quoted command or a grep
    // pattern, and the lowercase word is ordinary English.
    let safe = accepts(
        "check-requirement-keywords",
        "safe",
        "# T\n\n```sh\nrg 'MUST|SHALL' docs/\n```\n\nThe `MUST NOT` keyword is capped, and a reader must read the spec.\n",
    )
    .expect("the gate should be runnable");
    let Some(safe) = safe else { return };
    assert!(
        safe,
        "fenced blocks, code spans and lowercase prose must pass"
    );

    let escaped = accepts(
        "check-requirement-keywords",
        "escape",
        "# T\n\n<!-- allow-requirement-keyword: quoting the adopted rule verbatim -->\nThe author MUST give every requirement a verification line.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(escaped, "the escape hatch must exempt the following line");

    // A word that merely contains a keyword is not a keyword.
    let substring = accepts(
        "check-requirement-keywords",
        "substring",
        "# T\n\nThe MUSTARD constant and the plan's MAYBE branch are names.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(substring, "a keyword inside a longer word must pass");
}

#[test]
fn breaking_footer_gate_rejects_a_marked_subject_with_no_footer() {
    // The defect this gate exists for, and the one it was written after two
    // breaking changes shipped it. The marker tells the release tooling to
    // compute a major bump, and the body then owes the guidance the generated
    // entry is built from.
    let missing = accepts(
        "check-breaking-footer",
        "no-footer",
        "feat(record)!: rename the key\n\nSome context about the change.\n",
    )
    .expect("the gate should be runnable");
    let Some(missing) = missing else { return };
    assert!(
        !missing,
        "a marked subject with no footer must fail the gate"
    );

    // The two shapes an author reaches for instead. Neither reaches the
    // generated entry, and before this gate neither failed anything.
    let heading = accepts(
        "check-breaking-footer",
        "heading",
        "feat(record)!: rename the key\n\n## Breaking change\n\nAn adopter renames it.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(!heading, "a markdown heading is not a footer");

    let prose = accepts(
        "check-breaking-footer",
        "prose",
        "feat(record)!: rename the key\n\nBreaking: an adopter renames it.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(!prose, "prose naming the break is not a footer");
}

#[test]
fn breaking_footer_gate_admits_a_footer_and_ignores_what_is_not_breaking() {
    // The accepting side. A gate that refused every message would pass the
    // case above while making the repository uncommittable, and only the
    // messages it must let through tell the two apart.
    let footed = accepts(
        "check-breaking-footer",
        "footer",
        "feat(record)!: rename the key\n\nSome context.\n\nBREAKING CHANGE: the key is renamed, so an adopter updates every row.\n",
    )
    .expect("the gate should be runnable");
    let Some(footed) = footed else { return };
    assert!(footed, "a message carrying its footer must pass");

    // Conventional Commits accepts both spellings, and git-cliff reads both.
    let hyphenated = accepts(
        "check-breaking-footer",
        "footer-hyphen",
        "feat(record)!: rename the key\n\nBREAKING-CHANGE: the key is renamed, so an adopter updates every row.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(hyphenated, "the hyphenated footer token must pass");

    // No marker, no obligation.
    let ordinary = accepts(
        "check-breaking-footer",
        "ordinary",
        "feat(record): add a field\n\nSome context.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(ordinary, "a message with no breaking marker must pass");

    // The bot writes its own release message, and it carries no marker.
    let release = accepts(
        "check-breaking-footer",
        "release",
        "chore: release v0.6.0\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(release, "the release request's own message must pass");

    // Git strips comment lines before it writes the commit, so a footer that
    // only appears in the template's commentary is not a footer.
    let commented = accepts(
        "check-breaking-footer",
        "commented",
        "feat(record)!: rename the key\n\n# BREAKING CHANGE: an adopter renames it.\n",
    )
    .expect("the gate should be runnable")
    .expect("the script was present a moment ago");
    assert!(!commented, "a commented-out footer is not a footer");
}
