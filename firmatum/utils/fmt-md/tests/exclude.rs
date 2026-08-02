//! Exclusion regression tests (R10).
//!
//! Pinned by a real incident: a run over udon `.archived` reformatted 20 raw
//! AI session transcripts, joining pasted shell scripts into single lines.
//! Every render check passed — correctly, since the rendered document really
//! was unchanged — so the only possible defence is a declared exclusion.

use fmt_md::exclude::Excluder;
use std::fs;
use std::path::Path;

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("fmt-md-test-{name}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(path: &Path, body: &str) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    fs::write(path, body).unwrap();
}

#[test]
fn ignore_file_excludes_matching_paths() {
    let root = scratch("basic");
    write(&root.join(".fmt-mdignore"), "vault/raw/\n*.generated.md\n");
    write(&root.join("vault/raw/session.md"), "x\ny\n");
    write(&root.join("vault/notes.md"), "x\ny\n");
    write(&root.join("thing.generated.md"), "x\ny\n");
    write(&root.join("prose.md"), "x\ny\n");

    let mut ex = Excluder::new();
    assert!(ex.excluded(&root.join("vault/raw/session.md")).is_some());
    assert!(ex.excluded(&root.join("thing.generated.md")).is_some());
    assert!(ex.excluded(&root.join("vault/notes.md")).is_none());
    assert!(ex.excluded(&root.join("prose.md")).is_none());
}

#[test]
fn ignore_file_applies_from_any_ancestor() {
    let root = scratch("ancestor");
    write(&root.join(".fmt-mdignore"), "**/frozen/**\n");
    write(&root.join("a/b/c/frozen/deep.md"), "x\ny\n");
    write(&root.join("a/b/c/live.md"), "x\ny\n");

    let mut ex = Excluder::new();
    assert!(ex.excluded(&root.join("a/b/c/frozen/deep.md")).is_some());
    assert!(ex.excluded(&root.join("a/b/c/live.md")).is_none());
}

#[test]
fn negation_can_readmit_a_file() {
    let root = scratch("negate");
    write(&root.join(".fmt-mdignore"), "archive/\n!archive/README.md\n");
    write(&root.join("archive/raw.md"), "x\ny\n");
    write(&root.join("archive/README.md"), "x\ny\n");

    let mut ex = Excluder::new();
    assert!(ex.excluded(&root.join("archive/raw.md")).is_some());
    assert!(ex.excluded(&root.join("archive/README.md")).is_none());
}

#[test]
fn no_ignore_file_means_nothing_excluded() {
    let root = scratch("none");
    write(&root.join("prose.md"), "x\ny\n");
    let mut ex = Excluder::new();
    assert!(ex.excluded(&root.join("prose.md")).is_none());
}

/// The incident itself, in miniature: a pasted shell script inside a list
/// item is lazy paragraph continuation to CommonMark, so fmt-md joins it and
/// every render check agrees nothing changed. The damage is real and only
/// exclusion prevents it — this test asserts both halves of that truth so
/// neither is "fixed" by accident later.
#[test]
fn transcript_damage_is_invisible_to_render_checks() {
    let transcript = "\
## Assistant

- Execute: # inventory
UDON_SESS=~/.grok/sessions/x
for d in \"$UDON_SESS\"/*/; do
  echo \"$d\"
done
";
    let formatted = fmt_md::format(transcript);
    // it really does get joined ...
    assert_ne!(formatted, transcript, "expected the joiner to act here");
    assert!(
        formatted.lines().count() < transcript.lines().count(),
        "shell lines should have collapsed into the list item"
    );
    // ... and the render check cannot object, because nothing rendered differently
    assert_eq!(
        fmt_md::render_fingerprint(transcript),
        fmt_md::render_fingerprint(&formatted),
        "render-equality is expected to hold; if this ever fails, the \
         justification for exclusions in exclude.rs needs rewriting"
    );
}

// ---------------------------------------------------------------------------
// Foreign-language guard (.udon), pinned by UDON-ASSESSMENT-2026-07-29.md.
// ---------------------------------------------------------------------------

#[test]
fn udon_extension_is_recognized_regardless_of_case_or_directory() {
    use fmt_md::exclude::foreign_language;
    assert_eq!(foreign_language(Path::new("OUTLINE.udon")), Some("udon"));
    assert_eq!(foreign_language(Path::new("a/b/c/spec.UDON")), Some("udon"));
    assert_eq!(
        foreign_language(Path::new("core/generator/x.descent.udon")),
        Some("udon")
    );
    // markdown, and lookalikes that are not the extension, stay eligible
    assert_eq!(foreign_language(Path::new("README.md")), None);
    assert_eq!(foreign_language(Path::new("udon/v2/notes.md")), None);
    assert_eq!(foreign_language(Path::new("about-udon.md")), None);
    assert_eq!(foreign_language(Path::new("udon")), None);
}

/// The guard has to sit in *front* of the render-equality gate, not lean on
/// it. This asserts both halves of why, on the exact case the assessment
/// reproduced: three UDON attributes collapse into one holding garbage, and
/// the render check has no objection — so if this test ever starts failing on
/// the render assertion, the gate grew a UDON model and the reasoning in
/// exclude.rs needs rewriting.
#[test]
fn udon_attribute_damage_is_invisible_to_render_checks() {
    let doc = "\
|article[intro].featured
  :author Joseph Wecker
  :date 2025-12-22
  :tags [udon notation design]
";
    let formatted = fmt_md::format(doc);
    assert_ne!(formatted, doc, "expected the joiner to act here");
    // Per CORE.md §6.4 a bare value runs to end of line, so once these share
    // a physical line `author` owns the ":date …" and ":tags …" text and the
    // other two attributes cease to exist.
    let collapsed = formatted
        .lines()
        .any(|l| l.contains(":author") && l.contains(":date") && l.contains(":tags"));
    assert!(
        collapsed,
        "expected the attribute rows to be joined onto one line; got:\n{formatted}"
    );
    assert_eq!(
        fmt_md::render_fingerprint(doc),
        fmt_md::render_fingerprint(&formatted),
        "render-equality is expected to hold — that is precisely why the \
         .udon guard cannot be delegated to the gate (see exclude.rs)"
    );
}

/// `!:lang:` verbatim blocks are invisible to comrak, so working source gets
/// flattened into one line. Same both-halves structure as above.
#[test]
fn udon_verbatim_blocks_are_flattened_without_objection() {
    let doc = "\
|note
  :body !:elixir:
    defmodule Hello do
      def world, do: IO.puts(\"Hello from UDON\")
    end
";
    let formatted = fmt_md::format(doc);
    assert!(
        formatted.lines().count() < doc.lines().count(),
        "expected the verbatim block to collapse; got:\n{formatted}"
    );
    assert_eq!(
        fmt_md::render_fingerprint(doc),
        fmt_md::render_fingerprint(&formatted),
        "render-equality is expected to hold here too"
    );
}
